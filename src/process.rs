/**
This module contains the bulk of the header-processing code, and the core structures.

Note that it specifically *does not* contain conditional expression handling, or feature set abstractions.
*/
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path;
use std::rc::Rc;
use itertools::Itertools;
use {ExpConfig, GenConfig, NativeCallConv, OutConfig, WinVersion};
use clang::{
    self,
    Index, RcIndexExt,
    TranslationUnit, TranslationUnitFlags,
    Cursor, CursorKind,
};
use features::{Features, scan_features};
use util;

const EMIT_STUBS: bool = false;

/**
This is effectively the "entry point" for processing.  Given a header and a configuration, it attempts to generate a Rust binding.
*/
pub fn process_header(path: &str, gen_config: &GenConfig, out_config: &OutConfig) {
    info!("using clang version {}", clang::version());

    let index = Index::create(
        /*exclude_declarations_from_pch*/ false,
        /*display_diagnostics*/ false,
    );

    let mut out_items = OutputItems::new();
    let mut cache = Cache::new(index, gen_config);

    // Expand once for each expansion config.
    for exp_config in &gen_config.exp_configs {
        info!("expanding with config {:?}", exp_config);
        info!(".. switches: {:?}", exp_config.switches());
        let tu = cache.tu.parse_translation_unit(path, exp_config).ok().expect("parse TU");
        let renames = scan_for_renames(tu.clone(), gen_config);
        process_decls(tu, gen_config, exp_config, &mut out_items, &mut cache, &renames);
    }

    info!("generating output...");
    let out_files = &mut OutputFiles::new(out_config);
    output_header_items(&out_items, out_files);
    output_func_items(&out_items, out_files, out_config);

    info!("sanity-checking features...");
    sanity_check_features(&mut cache);
}

/**
A helper method that yields the "next" cursor to process from both a primary sequence and a list of deferred items.
*/
fn next_from(
    dc: &mut ::std::vec::IntoIter<Cursor>,
    d: &mut Vec<Cursor>,
    di: &mut Option<::std::vec::IntoIter<Cursor>>
) -> Option<Cursor> {
    use std::mem::replace;

    if let Some(cur) = di.as_mut().and_then(|di| di.next()) {
        return Some(cur)
    }

    *di = None;

    if d.len() > 0 {
        *di = Some(replace(d, vec![]).into_iter());
        return next_from(dc, d, di);
    }

    dc.next()
}

struct Renames {
    renames: HashMap<Cursor, Cursor>,
    invalidations: HashSet<Cursor>,
}

impl Renames {
    fn add_rename(&mut self, from: Cursor, to: Cursor) {
        assert!(!self.is_renamed(&from), "definition is already renamed");
        assert!(!self.invalidations.contains(&to), "already have invalidation");
        self.renames.insert(from, to.clone());
        self.invalidations.insert(to);
    }

    fn is_invalidated(&self, decl_cur: &Cursor) -> bool {
        self.invalidations.contains(decl_cur)
    }

    fn is_renamed(&self, defn_cur: &Cursor) -> bool {
        self.renames.contains_key(defn_cur)
    }

    fn rename_decl<'a, 'b>(&'a self, decl_cur: &'b Cursor) -> Result<&'a Cursor, &'b Cursor> {
        if let Some(&ref cur) = self.renames.get(decl_cur) {
            Ok(cur)
        } else {
            Err(decl_cur)
        }
    }

    fn rename_ty(&self, ty: clang::Type) -> Result<clang::Cursor, clang::Type> {
        use clang::TypeKind as TK;

        // We're only concerned about structs, enums and unions.
        match ty.kind() {
            TK::Record | TK::Enum => {
                let ty_decl = match ty.declaration().definition() {
                    Some(cur) => cur,
                    None => return Err(ty)
                };
                if let Some(&ref cur) = self.renames.get(&ty_decl) {
                    return Ok(cur.clone());
                }
                Err(ty)
            },
            _ => Err(ty)
        }
    }
}

impl Default for Renames {
    fn default() -> Self {
        Renames {
            renames: HashMap::new(),
            invalidations: HashSet::new(),
        }
    }
}

fn scan_for_renames(tu: Rc<TranslationUnit>, gen_config: &GenConfig) -> Renames {
    /*
    The goal here is to find two kinds of things:

    1. Types which have *no* name in tag space, but are given one via typedef.
    2. Types which *have* a name in tag space, but the *canonical* name is given via typedef.

    We handle both of these by scanning through all the typedefs.  If we find one whose subject is one of the above types, we record the subject's cursor and the *new* name, as well as an "invalidation" of the typedef's cursor.
    */
    info!("scanning for renames...");
    let mut renames = Renames::default();
    let mut decl_curs = tu.cursor().children().into_iter();
    let mut deferred: Vec<Cursor> = vec![];
    let mut deferred_iter = None;

    while let Some(decl_cur) = next_from(&mut decl_curs, &mut deferred, &mut deferred_iter) {
        if !gen_config.should_ignore(&decl_cur) {
            let rename = scan_decl_for_rename(decl_cur, gen_config, &renames, &mut |cur| deferred.push(cur));
            if let Some((from, to)) = rename {
                renames.add_rename(from, to);
            }
        }
    }

    renames
}

fn scan_decl_for_rename<Defer>(
    decl_cur: Cursor,
    gen_config: &GenConfig,
    renames: &Renames,
    mut defer: Defer,
) -> Option<(Cursor, Cursor)>
where Defer: FnMut(Cursor) {
    use clang::CursorKind as CK;
    use clang::TypeKind as TK;

    match decl_cur.kind() {
        CK::TypedefDecl => (),
        _ => return None
    }

    debug!("scan_decl_for_rename({}, ..)", decl_cur);

    let ty = decl_cur.typedef_decl_underlying_type();

    // Resolve unexposed types if possible.
    let ty = match ty.kind() {
        TK::Unexposed => ty.canonical(),
        _ => ty
    };

    // We don't want to even look at things that aren't just direct ADT typedefs.
    match ty.kind() {
        TK::Record | TK::Enum => (),
        other => {
            debug!(".. ignoring indirect typedef: {:?}", other);
            return None;
        }
    }

    // Get the original type definition.
    let ty_defn = ty.declaration().definition();
    let ty_defn = match ty_defn {
        Some(c) => c,
        None => {
            debug!(".. ignoring; has no definition");
            return None;
        }
    };

    // Double-check that this is really what we think it is.
    match ty_defn.kind() {
        CK::StructDecl | CK::EnumDecl | CK::UnionDecl => (),
        other => {
            debug!(".. ignoring non-adt target: {:?}", other);
            return None;
        }
    };

    // Defer all the child cursors.
    for child in ty_defn.children() { defer(child); }

    // Check if it's already canonical.
    let ty_name = ty_defn.spelling();
    if !gen_config.is_tag_name_non_canonical(&ty_name) {
        debug!(".. ignoring canonical name {:?}", ty_name);
        return None;
    }

    // Finally, make sure we aren't renaming it more than once.
    if renames.is_renamed(&ty_defn) {
        debug!(".. ignoring; already renamed");
        return None;
    }

    // Do the rename.
    debug!(".. renaming {} to {}", ty_defn, decl_cur);
    Some((ty_defn, decl_cur))
}

fn process_decls(
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
        if gen_config.should_ignore(&decl_cur) {
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

fn output_header_items(items: &OutputItems, output: &mut OutputFiles) {
    let mut lines = vec![];
    for (_, decls) in &items.header_items {
        for &(ref header, ref feat, ref decl, ref annot) in decls {
            lines.push((header, feat, decl, annot));
        }
    }
    lines.sort();

    for (header, feat, decl, annot) in lines {
        output.emit_to_header(header, feat, decl, annot);
    }
}

fn output_func_items(items: &OutputItems, output: &mut OutputFiles, out_config: &OutConfig) {
    let mut lines = vec![];
    for (name, decls) in &items.fn_items {
        for &(ref feat, ref cconv, ref decl, ref annot) in decls {
            lines.push((name, feat, cconv, decl, annot));
        }
    }
    lines.sort();

    for (name, feat, cconv, decl, annot) in lines {
        let libs = out_config.get_fn_libs(name);
        for &ref lib in libs {
            output.emit_to_library(lib, feat, cconv, decl, annot);
        }
    }
}

fn sanity_check_features(cache: &mut Cache) {
    use std::collections::BTreeSet;

    let mut weird_vers = BTreeSet::new();

    cache.iter_features(|path, line, &ref feat| {
        use features::Partitions;

        /*
        What we're looking for are any features that might mess up the expansion.  This currently means:

        - Features with upper limits on versions.
        - Features that *do not* target the desktop.
        */

        let mut suspect = vec![];

        if let Some(ref parts) = feat.parts {
            if (parts.clone() & Partitions::DesktopApp).is_empty() {
                suspect.push("non-desktop-app");
            }
        }

        if let Some(ref winver) = feat.winver {
            if !winver.is_simple() {
                for &ref range in winver.ranges() {
                    weird_vers.insert(range.end);
                }
                suspect.push("complex-winver");
            }
        }

        if suspect.len() != 0 {
            warn!("suspect feature set: {}:{}: {} {:?}",
                path, line, suspect.connect(", "), feat);
        }
    });

    if weird_vers.len() > 0 {
        warn!("suspect versions:");
        for ver in weird_vers {
            warn!(".. 0x{:08x} - {:?}",
                ver, WinVersion::from_u32_round_up(ver));
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
        CK::MacroDefinition => process_macro_defn(decl_cur, output, feat),

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

fn file_stem(cur: &Cursor) -> String {
    cur.location().file().expect("valid file for file_stem").name()
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
Works out a name for the given structure, even if it doesn't otherwise *have* one.
*/
fn name_for_maybe_anon(decl_cur: &Cursor) -> Result<String, String> {
    // TODO: Use clang_Cursor_isAnonymous once its released.
    let name = decl_cur.spelling();
    if name == "" {
        /*
        This is *probably* an anonymous type.  We need to give it a name that will be both reasonable *and* stable across invocations.
        */
        return Err(format!("anonymous-struct {}", decl_cur));
    }
    Ok(name)
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
Process a single macro definition.
*/
fn process_macro_defn(defn_cur: Cursor, _output: &mut OutputItems, _feat: Features) -> Result<(), String> {
    debug!("process_macro_defn({}, ..)", defn_cur);

    let name = defn_cur.spelling();
    let _annot = defn_cur.location().display_short().to_string();
    let mut toks: Vec<_> = defn_cur.tokenize().into_iter().skip(1).map(|t| t.spelling()).collect();
    toks.pop(); // Drop last because it's just a newline.

    if EMIT_STUBS {
        println!("// #define {} {:?}", name, toks);
    }

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

/**
Calculate the feature set map for a given file.
*/
fn get_all_features<'a>(file: clang::File, cache: &'a mut Cache) -> &'a BTreeMap<u32, Features> {
    let path = file.file_name();
    let tu_cache = &mut cache.tu;
    let fmap = cache.features.entry(path.clone()).or_insert_with(||
        scan_features(get_token_lines(file, tu_cache)));

    fmap
}

/**
Calculate the feature set at a given line.
*/
fn get_features_at(file: clang::File, line: u32, cache: &mut Cache) -> Features {
    use std::collections::Bound;

    debug!("get_features_at({:?}, {}, _)", file.file_name(), line);

    let fmap = get_all_features(file, cache);
    fmap.range(Bound::Unbounded, Bound::Included(&line)).next_back()
        .map(|(i, v)| {
            debug!(".. found: {}: {:?}", i, v);
            v.clone()
        })
        .unwrap_or_else(|| Features::default())
}

/**
Returns a given file as a sequence of `(line_number, tokens)` pairs.
*/
fn get_token_lines(file: clang::File, tu_cache: &mut TuCache) -> Vec<(u32, Vec<clang::Token>)> {
    debug!("get_token_lines({:?}, _)", file.file_name());
    let path = file.file_name();

    // Architecture shouldn't matter since we just want the tokens.
    let tu = tu_cache.parse_translation_unit(&path, &ExpConfig::DUMMY_CFG).unwrap();

    // Get the set of line numbers which *contain* a line continuation.
    let cont_lines: Vec<_> = util::read_lines(&path)
        .map(|rs| rs.unwrap())
        .enumerate()
        .filter(|&(_, ref s)| s.trim_right().ends_with("\\"))
        // +1 because enumerate is 0-based, line numbers are 1-based
        .map(|(i, _)| (i + 1) as u32) // TODO: checked
        .collect();

    // Work out the starting line for continued lines.
    let mut line_starts = HashMap::new();
    for i in cont_lines {
        let start = *line_starts.get(&i).unwrap_or(&i);
        line_starts.insert(i+1, start);
    }

    // Change each line's line number to be the *first* line the (possibly continued) line starts on.
    let remap_line_number = |tok: &clang::Token| {
        let l = tok.location().line();
        *line_starts.get(&l).unwrap_or(&l)
    };

    // Grab all the tokens, then re-group them by logical line.
    tu.tokenize().into_iter().group_by(remap_line_number).collect()
}

/**
Bundles together any caches we need for efficiency.
*/
struct Cache<'a> {
    /// Parsed Clang `TranslationUnit`s.
    tu: TuCache<'a>,

    /// Evaluated per-line feature sets.
    features: HashMap<String, BTreeMap<u32, Features>>,
}

impl<'a> Cache<'a> {
    fn new(index: Rc<Index>, gen_config: &'a GenConfig) -> Self {
        Cache {
            tu: TuCache::new(index, gen_config),
            features: HashMap::new(),
        }
    }

    /**
    Iterates over all feature sets.

    Note that this works by iterating over the underlying feature set maps.  What this gives you *in effect* is the evaluated feature set of *every* conditional compilation branch.
    */
    fn iter_features<F>(&mut self, mut f: F)
    where F: FnMut(&str, u32, &Features) {
        for (&ref name, &ref map) in self.features.iter() {
            for (&line, &ref feat) in map.iter() {
                f(name, line, feat);
            }
        }
    }
}

/**
Used to centralise how output of translated items is done.

One of the major reasons for this is to consolidate disparate bindings.  That is, if the output for both x86 and x86-64 are the same, then they should use a *single* declaration with an appropriate `#[cfg]` attribute.

Note that `annot` is used for "annotations", which are free-form strings that may be emitted as comments in the output.  These are handy for identifying, for example, *where* a declaration originally came from, for debugging purposes.
*/
struct OutputItems {
    /// `[name => [(feat, cconv, decl, annot)]]`
    fn_items: HashMap<String, Vec<(Features, AbsCallConv, String, String)>>,

    /// `[name => [(header, feat, decl, annot)]]`
    header_items: HashMap<String, Vec<(String, Features, String, String)>>,
}

impl OutputItems {
    fn new() -> Self {
        OutputItems {
            fn_items: HashMap::new(),
            header_items: HashMap::new(),
        }
    }

    /**
    Adds a function declaration.

    If the given `decl` matches an already existing `decl` with the same `name`, the existing entry will have its feature set unioned with `feat`, and `annot` appended to its annotation.
    */
    fn add_func_item(&mut self, name: String, feat: Features, cconv: AbsCallConv, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_func_item({:?}, {:?}, {:?}, {:?}, {:?})", name, feat, cconv, decl, annot);

        let decls = self.fn_items.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (ref mut df, ref dcc, ref dd, ref mut da) in decls.iter_mut() {
            if *dd == decl && *dcc == cconv {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                let new_df = replace(df, Features::default()).or(feat);
                *df = new_df;
                da.push_str(", ");
                da.push_str(&annot);
                return;
            }
        }

        // Add it to the set of decls.
        debug!(".. adding");
        decls.push((feat, cconv, decl, annot));
    }

    /**
    Adds a header declaration.

    If the given `decl` matches an already existing `decl` with the same `name`, the existing entry will have its feature set unioned with `feat`, and `annot` appended to its annotation.
    */
    fn add_header_item(&mut self, name: String, header: String, feat: Features, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_header_item({:?}, {:?}, {:?}, {:?}, {:?})", header, name, feat, decl, annot);

        let decls = self.header_items.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (ref dh, ref mut df, ref dd, ref mut da) in decls.iter_mut() {
            if *dh == header && *dd == decl {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                let new_df = replace(df, Features::default()).or(feat);
                *df = new_df;
                da.push_str(", ");
                da.push_str(&annot);
                return;
            }
        }

        // Add it to the set of decls.
        debug!(".. adding");
        decls.push((header, feat, decl, annot));
    }
}

/**
This cache owns the output files and saves us from constantly opening and closing them.
*/
struct OutputFiles<'a> {
    out_config: &'a OutConfig,
    files: HashMap<path::PathBuf, fs::File>,
}

impl<'a> OutputFiles<'a> {
    fn new(out_config: &'a OutConfig) -> Self {
        OutputFiles {
            out_config: out_config,
            files: HashMap::new(),
        }
    }

    fn emit_to_header(&mut self, name: &str, feat: &Features, decl: &str, annot: &str) {
        use std::io::prelude::*;
        let file = self.get_file(name, &self.out_config.header_path);
        writeln!(file, "{}{} /* {} */", feat, decl, annot).unwrap();
    }

    fn emit_to_library(&mut self, name: &str, feat: &Features, cconv: &AbsCallConv, decl: &str, annot: &str) {
        use std::io::prelude::*;
        let file = self.get_file(name, &self.out_config.library_path);
        writeln!(file, "{}extern {:?} {{ {} /* {} */ }}", feat, cconv.as_str(), decl, annot).unwrap();
    }

    fn get_file<'b>(&'b mut self, name: &str, pattern: &str) -> &'b mut fs::File {
        use std::path::PathBuf;
        let mut path = PathBuf::from(&self.out_config.output_dir);
        path.push(pattern.replace("{}", name));
        self.files.entry(path.clone()).or_insert_with(|| fs::File::create(path).unwrap())
    }
}

/**
An "abstract" calling convention.

This is to answer the question: "if a function uses the C calling convention, is that the same thing as `"system"`, or do I have to *actually* say `"C"`?"

Without this, almost every Windows API call would need two decls: one with `extern "C"`, and one with `extern "stdcall"`.  Yuck.
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum AbsCallConv {
    ExplicitlyC,
    System,
}

impl AbsCallConv {
    /// Gets the calling convention as a string, suitable for use with Rust's `extern`.
    fn as_str(self) -> &'static str {
        use self::AbsCallConv::*;
        match self {
            ExplicitlyC => "C",
            System => "system",
        }
    }
}

/**
A `TranslationUnit` cache.
*/
pub struct TuCache<'a> {
    index: Rc<Index>,
    cache: HashMap<TuCacheKey, Rc<TranslationUnit>>,
    gen_config: &'a GenConfig,
}

impl<'a> TuCache<'a> {
    pub fn new(index: Rc<Index>, gen_config: &'a GenConfig) -> TuCache<'a> {
        TuCache {
            index: index,
            cache: HashMap::new(),
            gen_config: gen_config,
        }
    }

    /**
    Parse a translation unit with the given expansion config.

    Unsurprisingly, this will return a cached TU if one has already been parsed.
    */
    pub fn parse_translation_unit(
        &mut self,
        path: &str,
        exp_config: &ExpConfig,
    ) -> Result<Rc<TranslationUnit>, clang::ErrorCode> {
        let index_opts = TranslationUnitFlags::None
            | TranslationUnitFlags::DetailedPreprocessingRecord
            | TranslationUnitFlags::Incomplete
            ;

        let key = TuCacheKey::new(path, exp_config);
        info!("parsing tu {:?} with {:?} ({:?})", path, exp_config, key);

        if let Some(rc_tu) = self.cache.get(&key) {
            info!(".. already in cache");
            return Ok(rc_tu.clone())
        }

        let switches: Vec<String> = self.gen_config.switches().iter().map(|s| s.clone())
            .chain(exp_config.switches().into_iter())
            .collect();

        let tu = try!(self.index.parse_translation_unit(
            path,
            &switches,
            &[],
            index_opts,
        ));
        self.cache.insert(key, tu.clone());
        Ok(tu)
    }
}

/**
This is the unique key for each entry in the `TuCache`.
*/
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TuCacheKey(String, ExpConfig);

impl TuCacheKey {
    pub fn new(path: &str, exp_config: &ExpConfig) -> TuCacheKey {
        TuCacheKey(path.into(), exp_config.clone())
    }
}
