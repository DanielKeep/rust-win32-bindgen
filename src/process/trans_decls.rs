use std::rc::Rc;
use {ExpConfig, GenConfig, NativeCallConv};
use clang::{
    self,
    TranslationUnit,
    Cursor, CursorKind,
};
use features::Features;

use super::{EMIT_STUBS, Cache, file_stem, name_for_maybe_anon, next_from};
use super::features::get_features_at;
use super::output::{AbsCallConv, OutputItems};
use super::renames::Renames;

pub fn process_decls(
    tu: Rc<TranslationUnit>,
    gen_config: &GenConfig,
    exp_config: &ExpConfig,
    output: &mut OutputItems,
    cache: &mut Cache,
    renames: &Renames,
) {
    let feat_mask = exp_config.arch.to_features();

    let mut decl_curs = tu.cursor().children().into_iter();
    let mut deferred: Vec<Cursor> = vec![];
    let mut deferred_iter = None;

    while let Some(decl_cur) = next_from(&mut decl_curs, &mut deferred, &mut deferred_iter) {
        /*
        Something to be aware of: a symbol might match the ignore patterns, but have been renamed to something that *doesn't*.  We need to check for this.
        */
        if gen_config.should_ignore(&decl_cur) && !renames.is_renamed(&decl_cur) {
            debug!("ignoring: {}", decl_cur);
        } else if renames.is_invalidated(&decl_cur) {
            debug!("invalidated: {}", decl_cur);
        } else {
            process_decl(
                decl_cur,
                feat_mask.clone(),
                exp_config.native_cc,
                output,
                cache,
                renames,
                &mut |cur| deferred.push(cur),
            );
        }
    }
}

/**
Processes a single declaration.
*/
fn process_decl<Defer>(
    decl_cur: Cursor,
    feat_mask: Features,
    native_cc: NativeCallConv,
    output: &mut OutputItems,
    cache: &mut Cache,
    renames: &Renames,
    defer: &mut Defer,
)
where Defer: FnMut(Cursor)
{
    use clang::CursorKind as CK;

    let decl_kind = match decl_cur.kind() {
        CK::InclusionDirective
        | CK::MacroInstantiation
        => return,
        kind => kind
    };

    let decl_loc = decl_cur.location();

    debug!("process_decl: {}: {:?} {}",
        decl_loc.display_short(),
        decl_kind,
        decl_cur.spelling());

    let feat = decl_loc.file()
        .map(|file| get_features_at(file, decl_loc.line(), cache))
        .unwrap_or_else(|| Features::default());

    debug!(".. process_decl feat: {:?}", feat);

    /*
    This is kind of a pain, but as it turns out, different architectures can cause some things to behave in weird ways.  For example, `DWORD` might be a typedef on one arch, but a macro on another, which leads to a different expansion.  Hooray!
    */
    let feat = match feat.and(feat_mask).check_valid() {
        Ok(feat) => feat,
        Err(err) => {
            /*
            This is *very definitely* a problem.  This means that the pre-processor has emitted code that our feature set says we shouldn't ever reach!

            This generally means one of two things: either the feature set calculation is wrong *or* the set of pre-defined symbols is incomplete/incorrect.
            */
            panic!(".. invalid feature set for {}: {}", decl_cur, err)
        },
    };

    let decl_cur_copy = decl_cur.clone();

    let result = match decl_kind {
        CK::InclusionDirective
        | CK::MacroInstantiation
        => unreachable!(),

        CK::StructDecl => process_struct_decl(decl_cur, output, feat, renames, defer),
        CK::UnionDecl => process_union_decl(decl_cur, output, feat, renames, defer),
        CK::EnumDecl => process_enum_decl(decl_cur, output, feat, renames, defer),
        CK::FunctionDecl => process_function_decl(decl_cur, output, feat, renames, native_cc),
        CK::TypedefDecl => process_typedef_decl(decl_cur, output, feat, renames),
        CK::MacroDefinition => super::trans_macros::process_macro_defn(decl_cur, output, feat),

        kind => {
            warn!("could-not-translate unsupported {:?} {} at {}",
                kind, decl_cur.spelling(), decl_loc.display_short());
            Ok(())
        }
    };

    if let Err(err) = result {
        warn!("could-not-translate misc {}: {}", decl_cur_copy, err);
    }
}

/**
Process a single structure declaration.
*/
fn process_struct_decl<Defer>(
    decl_cur: Cursor,
    output: &mut OutputItems,
    feat: Features,
    renames: &Renames,
    defer: &mut Defer,
) -> Result<(), String>
where Defer: FnMut(Cursor)
{
    use clang::CursorKind as CK;

    debug!("process_struct_decl({}, ..)", decl_cur);

    let (name, header) = match renames.rename_decl(&decl_cur) {
        Ok(cur) => {
            debug!(".. was renamed to {}", cur);
            (cur.spelling(), file_stem(&cur))
        },
        Err(cur) => (try!(name_for_maybe_anon(&cur)), file_stem(&cur))
    };
    let annot = decl_cur.location().display_short().to_string();

    match (decl_cur.is_definition(), decl_cur.definition().is_none()) {
        (false, false) => {
            debug!(".. skipping forward declaration");
            return Ok(());
        },
        (false, true) => {
            // There *is no* definition!
            debug!(".. no definition found");
            let decl = format!("#[repr(C)] pub struct {};", name);
            output.add_header_item(name, header, feat, decl, annot);
            return Ok(())
        },
        (true, _) => ()
    }

    let mut fields = vec![];

    for child_cur in decl_cur.children() {
        match child_cur.kind() {
            CK::StructDecl
            | CK::UnionDecl
            => {
                // Defer.
                defer(child_cur);
            },

            CK::FieldDecl => {
                let name = child_cur.spelling();
                let ty = match trans_type(child_cur.type_(), renames) {
                    Ok(ty) => ty,
                    Err(err) => {
                        if EMIT_STUBS {
                            // TODO: just stub for now.
                            let decl = format!("#[repr(C)] pub struct {}; /* ERR STUB! */", name);
                            output.add_header_item(name, header, feat, decl, annot);
                        }
                        return Err(err);
                    }
                };
                fields.push(format!("{}: {}", name, ty));
            },

            CK::UnexposedAttr => {
                // Skip.
            },

            other => panic!("nyi {:?}", other)
        }
    }

    let decl = match fields.len() {
        // Why did this have to be special-cased? :(
        0 => format!(
            "#[repr(C)] pub struct {name};",
            name = name,
        ),
        _ => format!(
            "#[repr(C)] pub struct {name} {{ {fields} }}",
            name = name,
            fields = fields.connect(", "),
        )
    };

    output.add_header_item(name, header, feat, decl, annot);
    Ok(())
}

/**
Process a single union declaration.
*/
fn process_union_decl<Defer>(
    decl_cur: Cursor,
    output: &mut OutputItems,
    feat: Features,
    _renames: &Renames,
    _defer: &mut Defer,
) -> Result<(), String>
where Defer: FnMut(Cursor)
{
    debug!("process_union_decl({}, ..)", decl_cur);

    let name = try!(name_for_maybe_anon(&decl_cur));
    let header = file_stem(&decl_cur);
    let annot = decl_cur.location().display_short().to_string();

    if EMIT_STUBS {
        let decl = format!(
            "#[repr(C)] pub /*union*/ struct {name}; /* STUB! */",
            name = name,
        );

        output.add_header_item(name, header, feat, decl, annot);
    }
    Ok(())
}

/**
Process a single enum declaration.
*/
fn process_enum_decl<Defer>(
    decl_cur: Cursor,
    output: &mut OutputItems,
    feat: Features,
    _renames: &Renames,
    _defer: &mut Defer,
) -> Result<(), String>
where Defer: FnMut(Cursor)
{
    debug!("process_enum_decl({}, ..)", decl_cur);

    let name = try!(name_for_maybe_anon(&decl_cur));
    let header = file_stem(&decl_cur);
    let annot = decl_cur.location().display_short().to_string();

    if EMIT_STUBS {
        let decl = format!(
            "#[repr(C)] pub enum {name}; /* STUB! */",
            name = name,
        );

        output.add_header_item(name, header, feat, decl, annot);
    }
    Ok(())
}

/**
Process a single function declaration.
*/
fn process_function_decl(
    decl_cur: Cursor,
    output: &mut OutputItems,
    feat: Features,
    renames: &Renames,
    native_cc: NativeCallConv
) -> Result<(), String> {
    use clang::CallingConv as CC;
    use ::NativeCallConv as NCC;

    debug!("process_function_decl({}, _)", decl_cur);

    // Is this an inline function?
    let children = decl_cur.children();
    if children.len() > 0 && children.last().unwrap().kind() == CursorKind::CompoundStmt {
        // Err... *might be*
        return Err("inline-fn".into());
    }

    let ty = decl_cur.type_();

    let cconv = match (ty.calling_conv(), native_cc) {
        (CC::C, NCC::C) => AbsCallConv::System,
        (CC::C, _) => AbsCallConv::ExplicitlyC,
        (CC::X86StdCall, NCC::Stdcall) => AbsCallConv::System,
        (cconv, _) => {
            return Err(format!("bad-cconv {:?}", cconv));
        }
    };

    let name = decl_cur.spelling();

    let res_ty = if ty.result().kind() == clang::TypeKind::Void {
        String::new()
    } else {
        format!(" -> {}", try!(trans_type(ty.result(), renames)))
    };

    let arg_tys: Vec<String> = try!(ty.args().into_iter().map(|ty| trans_type(ty, renames)).collect());
    let arg_tys = arg_tys.connect(", ");

    let decl = format!(
        r#"pub fn {name}({arg_tys}){res_ty};"#,
        name = name,
        arg_tys = arg_tys,
        res_ty = res_ty,
    );

    let annot = decl_cur.location().display_short().to_string();
    output.add_func_item(name, feat, cconv, decl, annot);
    Ok(())
}

/**
Process a single structure declaration.
*/
fn process_typedef_decl(
    decl_cur: Cursor,
    output: &mut OutputItems,
    feat: Features,
    renames: &Renames,
) -> Result<(), String> {
    debug!("process_typedef_decl({}, ..)", decl_cur);
    let name = decl_cur.spelling();
    let header = file_stem(&decl_cur);

    let ty = decl_cur.typedef_decl_underlying_type();
    let ty = try!(trans_type(ty, renames));

    let decl = format!("pub type {} = {};", name, ty);

    let annot = decl_cur.location().display_short().to_string();
    output.add_header_item(name, header, feat, decl, annot);
    Ok(())
}

/**
Translate a type into an equivalent Rust type reference.

Note that this **is not** for translating type declarations; you cannot just pass a structure definition.
*/
fn trans_type(ty: clang::Type, renames: &Renames) -> Result<String, String> {
    use clang::TypeKind as TK;
    debug!("trans_type({:?} {:?}, _)", ty.kind(), ty.spelling());

    /**
    This works out the module qualifier for a given type.  This is intended to let you put types into files based on their source header.
    */
    fn mod_qual(cur: &Cursor) -> String {
        let file = cur.location().file();
        match file.map(|f| f.name()) {
            Some(name) => format!("::{}::", name),
            None => String::new()
        }
    }

    let ty = match renames.rename_ty(ty) {
        Ok(cur) => {
            // Use whatever we've been given and don't look too closely...
            let qual = mod_qual(&cur);
            return Ok(format!("{}{}", qual, cur.spelling()));
        },
        Err(ty) => ty
    };

    match ty.kind() {
        TK::Invalid => Err("invalid type".into()),

        TK::Unexposed => {
            let canon_ty = ty.canonical();
            match canon_ty.kind() {
                TK::Unexposed => Err(format!("recursively unexposed type {}", canon_ty.spelling())),
                _ => trans_type(canon_ty, renames)
            }
        },

        // Basic types.
        TK::Void => Ok("libc::c_void".into()),
        TK::Char_U | TK::UChar => Ok("libc::c_uchar".into()),
        TK::Char16 => Ok("u16".into()),
        // **Note**: *not* `char` because C++ doesn't appear to guarantee that a value of type char32_t is a valid UTF-32 code unit.
        TK::Char32 => Ok("u32".into()),
        TK::UShort => Ok("libc::c_ushort".into()),
        TK::UInt => Ok("libc::c_uint".into()),
        TK::ULong => Ok("libc::c_ulong".into()),
        TK::ULongLong => Ok("libc::c_ulonglong".into()),
        TK::Char_S => Ok("libc::c_schar".into()),
        TK::SChar => Ok("libc::c_schar".into()),
        TK::WChar => Ok("libc::wchar_t".into()),
        TK::Short => Ok("libc::c_short".into()),
        TK::Int => Ok("libc::c_int".into()),
        TK::Long => Ok("libc::c_long".into()),
        TK::LongLong => Ok("libc::c_longlong".into()),
        TK::Float => Ok("libc::c_float".into()),
        TK::Double => Ok("libc::c_double".into()),
        TK::NullPtr => Ok("*mut libc::c_void".into()),

        // Constructed types.
        TK::Pointer => {
            // We want to know whether the thing we're pointing to is const or not.
            let pointee_ty = ty.pointee();
            let mut_ = if pointee_ty.is_const_qualified() { "const" } else { "mut" };
            Ok(format!("*{} {}", mut_, try!(trans_type(pointee_ty, renames))))
        },

        TK::Record
        | TK::Enum
        | TK::Typedef
        => {
            // **Note**: use the decl to avoid const-qualification.  This might not be correct.
            let decl_cur = ty.declaration();
            Ok(format!("{}{}", mod_qual(&decl_cur), decl_cur.spelling()))
        },

        TK::ConstantArray => {
            let elem_ty = ty.array_element_type();
            let mut_ = if elem_ty.is_const_qualified() { "const" } else { "mut" };
            let len = ty.array_size();
            Ok(format!("*{} [{}; {}]", mut_, try!(trans_type(elem_ty, renames)), len))
        },

        TK::IncompleteArray => {
            let elem_ty = ty.array_element_type();
            let mut_ = if elem_ty.is_const_qualified() { "const" } else { "mut" };
            Ok(format!("*{} {}", mut_, try!(trans_type(elem_ty, renames))))
        },

        // **Note**: This isn't currently in `libc`, and does *not* have a platform-independent definition.
        TK::Bool
        | TK::UInt128
        | TK::Int128
        | TK::LongDouble
        | TK::Overload
        | TK::Dependent
        | TK::ObjCId
        | TK::ObjCClass
        | TK::ObjCSel
        | TK::Complex
        | TK::BlockPointer
        | TK::LValueReference
        | TK::RValueReference
        | TK::ObjCInterface
        | TK::ObjCObjectPointer
        | TK::FunctionNoProto
        | TK::FunctionProto
        | TK::Vector
        | TK::VariableArray
        | TK::DependentSizedArray
        | TK::MemberPointer
        => Err(format!("unsupported type {:?}", ty.kind()))
    }
}
