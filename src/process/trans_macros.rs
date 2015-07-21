use itertools::Itertools;
use clang::Cursor;
use features::Features;
use util::ResultOptionExt;

use super::{EMIT_STUBS, NameMap, add_to_name_map_checked, escape_ident, file_stem, mod_qual};
use super::output::OutputItems;

/**
Process a single macro definition.
*/
pub fn process_macro_defn(
    defn_cur: Cursor,
    output: &mut OutputItems,
    feat: Features,
    name_map: &mut NameMap,
) -> Result<(), String> {
    use ::ppmac::parse;
    use ::ppmac::parse::Result as PResult;

    debug!("process_macro_defn({}, ..)", defn_cur);

    // Note: we skip the last token because it's just a newline.
    let toks = defn_cur.tokenize();
    let first_tok = toks.at(0);
    let next_tok = toks.get(1);
    let toks: Vec<_> = toks.into_iter().dropping(1).dropping_back(1).map(|t| t.spelling()).collect();

    // If it has no tokens... well, there's not much point.
    if toks.len() == 0 { return Ok(()); }

    let name = defn_cur.spelling();
    let header = file_stem(&defn_cur);
    let annot = defn_cur.location().display_short().to_string();

    // Work out whether this is a functionish macro or not.
    let is_fn_macro = {
        let first_col = first_tok.extent().expect("extent for macro first tok").end().column();
        let next_col = next_tok.map(|t| t.extent().expect("extent for macro next tok").end().column()).unwrap_or(!0);
        first_col + 1 == next_col
    };

    let (args, exp_toks) = if is_fn_macro {
        let args_end = toks.iter().take_while(|tok| *tok != ")").count();
        (&toks[0..args_end+1], &toks[args_end+1..])
    } else {
        (&toks[0..0], &toks[0..])
    };

    let exp_ast = match parse::expression(exp_toks) {
        PResult::Parsed(node, rem) => {
            if rem.len() != 0 {
                return Err(format!("incomplete parse: {:?}, leaving {:?}", node, rem));
            }
            node
        },
        PResult::Mismatch(err, rem) => {
            return Err(format!("could not parse {}, leaving {:?}", err, rem));
        }
    };

    // Check for a simple alias macro.
    if let (false, &::ppmac::Node::Ident(ref s)) = (is_fn_macro, &exp_ast) {
        use clang::CursorKind as CK;

        let decl_cur = match name_map.get(s) {
            Some(decl_cur) => decl_cur.clone(),
            None => return Err(format!("forward-reference to name {:?}", s))
        };
        match decl_cur.kind() {
            CK::StructDecl
            | CK::UnionDecl
            | CK::EnumDecl
            | CK::TypedefDecl
            | CK::MacroDefinition
            => {
                // Need the header name and spelling to do the alias.
                let qual = mod_qual(&decl_cur);
                let decl = format!("#[doc(inline)] pub use {}{} as {};", qual, escape_ident(s.clone()), escape_ident(name.clone()));

                // We want to alias to the *original* thing, so that if someone aliases *us*, they know how to make it work.  It saves us from having to preserve this information in the name map itself.
                try!(add_to_name_map_checked(name_map, name.clone(), decl_cur));
                output.add_header_item(name, header, feat, decl, annot);
                return Ok(())
            },

            CK::FunctionDecl => {
                // We need to make sure we "inherit" the library of the symbol we're aliasing.
                let decl = format!("#[doc(inline)] pub use self::{} as {};", escape_ident(s.clone()), escape_ident(name.clone()));
                try!(add_to_name_map_checked(name_map, name.clone(), decl_cur));
                output.add_func_alias(name, s.clone(), feat, decl, annot);
                return Ok(())
            },

            _ => {
                return Err(format!("unsupported-alias-target to {:?}, {}", s, decl_cur));
            }
        }
    }

    // Check for an "inty" macro expression.
    if let Some((v, t)) = try!(try_trans_inty_macro(&exp_ast, name_map)) {
        let decl = format!("pub const {}: {} = {}; /* {:?} */", escape_ident(name.clone()), t, v, exp_ast);
        try!(add_to_name_map_checked(name_map, name.clone(), defn_cur.clone()));
        output.add_header_item(name, header, feat, decl, annot);
        return Ok(());
    }

    if EMIT_STUBS {
        let decl = format!("// #define {}{} {:?}", name, args.join(""), exp_ast);
        output.add_header_item(name, header, feat, decl, annot);
    }

    Err("unsupported-macro".into())
}

fn try_trans_inty_macro(node: &::ppmac::Node, name_map: &NameMap) -> Result<Option<(String, String)>, String> {
    use ::ppmac::{Node, Signed, Size, UnaryOp};
    use self::try_trans_inty_macro as ttim;

    debug!("try_trans_inty_macro({:?})", node);

    match *node {
        Node::Call { ref subject, ref args } => match **subject {
            Node::Ident(ref s) => match (&**s, args.len()) {
                ("TEXT", 1) => ttim(&args[0], name_map),
                (name, args) => {
                    let _decl = match name_map.get(s) {
                        Some(decl) => decl,
                        None => return Err(format!("forward-reference to name {:?}", s))
                    };
                    debug!("ttim: unknown call ident {} @ {}", name, args);
                    Ok(None)
                }
            },
            _ => {debug!("ttim: non-ident call subject"); Ok(None)}
        },
        Node::Cast { ref ty, ref value } => {
            match **ty {
                Node::Type(ref name, ptr) => {
                    // Lookup type name.
                    let ty_cur = match name_map.get(name) {
                        Some(decl) => decl,
                        None => return Err(format!("forward-reference to name {:?}", name))
                    };
                    let ptr = if ptr { "*mut " } else { "" };
                    let ty = format!("{}{}{}", ptr, mod_qual(&ty_cur), name);
                    ttim(value, name_map)
                        .ro_map(|(value, _)| (format!("{} as {}", value, ty), ty))
                },
                _ => Ok(None)
            }
        },
        Node::Ident(ref s) => {
            let _decl = match name_map.get(s) {
                Some(decl) => decl,
                None => return Err(format!("forward-reference to name {:?}", s))
            };
            // TODO
            Ok(None)
        },
        Node::Integer(v, signed, size) => {
            Ok(Some(match (signed, size) {
                (Signed::No, Size::Unknown) => (format!("0x{:x}u32", v), "u32".into()),
                (Signed::No, Size::Long) => (format!("0x{:x}u64", v), "u64".into()),
                (Signed::Yes, Size::Unknown) => (format!("0x{:x}i32", v as i64), "i32".into()),
                (Signed::Yes, Size::Long) => (format!("0x{:x}i64", v as i64), "i64".into()),
            }))
        },
        Node::String(ref s, _) => Ok(Some((format!("\"{}\"", s), "&'static str".into()))),
        Node::Unary(UnaryOp::Com, ref expr) => ttim(expr, name_map).ro_map(|(expr, ty)| (format!("!{}", expr), ty)),
        Node::Unary(UnaryOp::Neg, ref expr) => ttim(expr, name_map).ro_map(|(expr, ty)| (format!("-{}", expr), ty)),
        ref node => {
            debug!("ttim: unsupported node: {:?}", node);
            Ok(None)
        }
    }
}
