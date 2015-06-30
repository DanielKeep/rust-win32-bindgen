#![feature(associated_consts)]
#![feature(btree_range)]
#![feature(collections_bound)]
#![feature(cstr_to_str)]
#![feature(rc_weak)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate itertools;
extern crate libc;
extern crate num;
extern crate regex;

#[macro_use] mod macros;
pub mod clang;
pub mod features;
pub mod generated;
pub mod util;

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use itertools::Itertools;
use regex::Regex;

use clang::{
    Index, RcIndexExt,
    TranslationUnit, RcTranslationUnitExt, TranslationUnitFlags,
    Cursor, CursorKind,
};

use features::{
    Features,
    define_feature, define_feature_expr,
};

struct Cache<'a> {
    tu: TuCache<'a>,
    features: HashMap<String, BTreeMap<u32, Features>>,
}

impl<'a> Cache<'a> {
    fn new(index: Rc<Index>, gen_config: &'a GenConfig) -> Self {
        Cache {
            tu: TuCache::new(index, gen_config),
            features: HashMap::new(),
        }
    }

    fn iter_features<F>(&mut self, mut f: F)
    where F: FnMut(&str, u32, &Features) {
        for (&ref name, &ref map) in self.features.iter() {
            for (&line, &ref feat) in map.iter() {
                f(name, line, feat);
            }
        }
    }
}

pub fn process_header(path: &str, gen_config: &GenConfig) {
    let index = Index::create(
        /*exclude_declarations_from_pch*/ false,
        /*display_diagnostics*/ false,
    );

    let mut output = Output::new();
    let mut cache = Cache::new(index, gen_config);

    for exp_config in &gen_config.exp_configs {
        info!("expanding with config {:?}", exp_config);
        info!(".. switches: {:?}", exp_config.switches());
        let root_tu = cache.tu.parse_translation_unit(path, exp_config).unwrap();
        let feat_mask = exp_config.arch.to_features();

        for decl_cur in root_tu.cursor().children().into_iter() {
            if gen_config.should_ignore(&decl_cur) {
                debug!("ignoring: {}", decl_cur);
            } else {
                process_decl(decl_cur, feat_mask.clone(), exp_config.native_cc, &mut output, &mut cache);
            }
        }
    }

    info!("generating output...");
    {
        let fn_decls = output.fn_decls;
        let mut names: Vec<_> = fn_decls.keys().collect();
        names.sort();

        for (name, &ref decls) in names.into_iter().map(|k| (k, &fn_decls[k])) {
            let mut decl_set = vec![];
            for &(ref feat, ref cc, ref decl, ref annot) in decls {
                debug!(".. {} ({}): {:?}", name, annot, feat);
                decl_set.push(format!("{extern_:<15} {{ {decl} }} /* {annot} */",
                    extern_ = format!("extern {:?}", cc.as_str()),
                    annot = annot,
                    decl = decl.replace(
                        "{feat}",
                        &format!("{:<33}", feat.to_string())
                    )
                ));
            }
            decl_set.sort();
            for decl in decl_set {
                println!("{}", decl);
            }
        }
    }

    info!("sanity-checking features...");
    {
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
                    ver, generated::winver::WinVersion::from_u32_round_up(ver));
            }
        }
    }
}

fn process_decl(decl_cur: Cursor, feat_mask: Features, native_cc: NativeCallConv, output: &mut Output, cache: &mut Cache) {
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

    // This is kind of a pain, but as it turns out, different architectures can cause some things to behave in weird ways.  For example, `DWORD` might be a typedef on one arch, but a macro on another, which leads to a different expansion.  Hooray!
    let feat = match feat.and(feat_mask).check_valid() {
        Ok(feat) => feat,
        Err(err) => panic!(".. invalid feature set for {}: {}", decl_cur, err),
    };

    match decl_kind {
        CK::InclusionDirective
        | CK::MacroInstantiation
        => unreachable!(),

        CK::FunctionDecl => process_function_decl(decl_cur, output, feat, native_cc),

        kind => warn!("could-not-translate unsupported {:?} {} at {}", kind, decl_cur.spelling(), decl_loc.display_short())
    }
}

fn process_function_decl(decl_cur: Cursor, output: &mut Output, feat: Features, native_cc: NativeCallConv) {
    use clang::CallingConv as CC;
    use self::NativeCallConv as NCC;

    debug!("process_function_decl({}, _)", decl_cur);

    // Is this an inline function?
    let children = decl_cur.children();
    if children.len() > 0 && children.last().unwrap().kind() == CursorKind::CompoundStmt {
        // Err... *might be*
        warn!("could-not-translate inline-fn {}", decl_cur);
        return;
    }

    let ty = decl_cur.type_();

    let cconv = match (ty.calling_conv(), native_cc) {
        (CC::C, NCC::C) => AbsCallConv::System,
        (CC::C, _) => AbsCallConv::ExplicitlyC,
        (CC::X86StdCall, NCC::Stdcall) => AbsCallConv::System,
        (cconv, _) => {
            warn!("could-not-translate bad-cconv {} {:?}", decl_cur, cconv);
            return;
        }
    };

    match (|| -> Result<(), String> {
        let name = decl_cur.spelling();
        let res_ty = if ty.result().kind() == clang::TypeKind::Void {
            String::new()
        } else {
            format!(" -> {}", try!(trans_type(ty.result())))
        };
        let arg_tys: Vec<String> = try!(ty.args().into_iter().map(trans_type).collect());
        let arg_tys = arg_tys.connect(", ");
        let decl = format!(
            r#"{{feat}}pub fn {name}({arg_tys}){res_ty};"#,
            name = name,
            arg_tys = arg_tys,
            res_ty = res_ty,
        );
        let annot = decl_cur.location().display_short().to_string();
        output.add_func_decl(name, feat, cconv, decl, annot);
        Ok(())
    })() {
        Ok(()) => (),
        Err(err) => warn!("could-not-translate misc {} {}", decl_cur, err)
    }
}

fn trans_type(ty: clang::Type) -> Result<String, String> {
    use clang::TypeKind as TK;
    debug!("trans_type({:?} {:?})", ty.kind(), ty.spelling());

    fn mod_qual(cur: &Cursor) -> String {
        let file = cur.location().file();
        match file.map(|f| f.name()) {
            Some(name) => format!("::{}::", name),
            None => String::new()
        }
    }

    match ty.kind() {
        TK::Invalid => Err("invalid type".into()),

        TK::Unexposed => {
            let canon_ty = ty.canonical();
            match canon_ty.kind() {
                TK::Unexposed => Err(format!("recursively unexposed type {}", canon_ty.spelling())),
                _ => trans_type(canon_ty)
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
            Ok(format!("*{} {}", mut_, try!(trans_type(pointee_ty))))
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
            Ok(format!("*{} [{}; {}]", mut_, try!(trans_type(elem_ty)), len))
        },

        TK::IncompleteArray => {
            let elem_ty = ty.array_element_type();
            let mut_ = if elem_ty.is_const_qualified() { "const" } else { "mut" };
            Ok(format!("*{} {}", mut_, try!(trans_type(elem_ty))))
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

fn get_all_features<'a>(file: clang::File, cache: &'a mut Cache) -> &'a std::collections::BTreeMap<u32, Features> {
    let path = file.file_name();
    let tu_cache = &mut cache.tu;
    let fmap = cache.features.entry(path.clone()).or_insert_with(||
        get_features(get_token_lines(file, tu_cache)));

    fmap
}

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
    tu.tokenize_all_to_vec().into_iter().group_by(remap_line_number).collect()
}

fn get_features(tls: Vec<(u32, Vec<clang::Token>)>) -> BTreeMap<u32, Features> {
    debug!("get_features([..; {}])", tls.len());
    /*
    The way this works is that we have to walk through *all* the lines, looking for preprocessor conditional compilation directives.  When we find them, we interpret them and push the enabled feature tests on to the stack.  Then, when we find something that *isn't* a conditional directive *and* the features have changed since the last time we did so, we add an entry to the map.
    */
    let mut stack = vec![Features::default()];

    fn fd() -> Features { Features::default() }

    fn seq(lhs: &[String], lhs_len: usize, rhs: &[&'static str]) -> bool {
        use std::cmp::min;
        let lhs = &lhs[..min(lhs.len(), lhs_len)];
        lhs.len() == rhs.len()
            && lhs.iter().zip(rhs.iter()).all(|(l,r)| &**l == *r)
    }

    fn append(stack: &mut Vec<Features>, f: Features) {
        debug!("append([..; {}], {:?})", stack.len(), f);
        let f = stack.last().expect("non-empty stack").clone().and(f);
        stack.push(f);
    }

    fn pop(stack: &mut Vec<Features>) {
        debug!("pop([..; {}])", stack.len());
        stack.pop();
    }

    let mut map = BTreeMap::new();

    // Insert fallback.
    map.insert(0, fd());

    for (line_num, toks) in tls {
        let loc = toks[0].location();
        let ts: Vec<_> = toks.iter()
            .map(|t| t.spelling())
            .filter(|s| !(s.starts_with("/*") || s.starts_with("//")))
            .collect();
        let ts = &*ts;
        debug!("get_features(..): {}: {:?}", loc.display_short(), ts);
        // debug!(".. stack: {:?}", stack);

        if seq(&ts, 2, &["#", "if"]) {
            debug!(".. #if {:?}", &ts[2..]);
            append(&mut stack, define_feature_expr(&ts[2..], &loc));
        } else if seq(&ts, 2, &["#", "ifdef"]) && ts.len() == 3 {
            debug!(".. #ifdef {:?}", &ts[2..]);
            append(&mut stack, define_feature(&ts[2]));
        } else if seq(&ts, 2, &["#", "ifndef"]) && ts.len() == 3 {
            debug!(".. #ifndef {:?}", &ts[2..]);
            append(&mut stack, define_feature(&ts[2]).complement());
            // debug!(".. #ifndef done");
        } else if seq(&ts, 2, &["#", "elif"]) {
            debug!(".. #elif {:?}", &ts[2..]);
            pop(&mut stack);
            append(&mut stack, define_feature_expr(&ts[2..], &loc));
        } else if seq(&ts, 2, &["#", "else"]) {
            debug!(".. #else");
            pop(&mut stack);
            append(&mut stack, fd());
        } else if seq(&ts, 2, &["#", "endif"]) {
            debug!(".. #endif");
            pop(&mut stack);
        } else {
            // debug!(".. boring");
            // Work out what the last set of features we've seen is.
            let do_insert = {
                let prev_feat = map.values().next_back().expect("previous feature");
                *prev_feat != *stack.last().expect("non-empty stack")
            };
            // debug!(".. do_insert: {:?}", do_insert);
            if do_insert {
                let feat = stack.last().expect("non-empty stack").clone();
                debug!(" .. insert {}: {:?}", line_num, feat);
                map.insert(line_num, feat);
            }
        }
    }

    debug!(".. done ({} entries)", map.len());
    map
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Architecture {
    X86_32,
    X86_64,
    Arm,
}

impl Architecture {
    fn to_features(self) -> Features {
        use self::Architecture::*;
        use features::Architectures as FA;
        let arch = match self {
            X86_32 => FA::X86_32,
            X86_64 => FA::X86_64,
            Arm => FA::Arm,
        };
        Features::from(arch)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExpConfig {
    pub arch: Architecture,
    pub winver: (u16, u32),
    pub native_cc: NativeCallConv,
}

impl ExpConfig {
    pub const WINVER_WIN81: (u16, u32) = (0x0602, 0x06030000);

    /// This *only* exists so we have something to give the TU cache when we're after tokens.
    const DUMMY_CFG: ExpConfig = ExpConfig {
        arch: Architecture::X86_32,
        winver: ExpConfig::WINVER_WIN81,
        native_cc: NativeCallConv::Stdcall,
    };

    const BASE_SWITCHES: &'static [&'static str] = &[
        "-D_WIN32",

        // This includes both Desktop and App partitions.
        "-DWINAPI_FAMILY=WINAPI_FAMILY_DESKTOP_APP"
    ];

    const X86_32_SWITCHES: &'static [&'static str] = &[
        "--target=i686-pc-windows-gnu", "-D__X86__", "-D_M_IX86"
    ];
    const X86_64_SWITCHES: &'static [&'static str] = &[
        "--target=x86_64-pc-windows-gnu", "-D_AMD64_", "-D_M_AMD64" // "-D_WIN64", "-D__x86_64", "-D__x86_64__", "_M_X64"
    ];
    const ARM_SWITCHES: &'static [&'static str] = &[
        "--target=arm-pc-windows-gnu", "-D_ARM_", "-D_M_ARM" // "-D__arm__"
    ];

    fn switches(&self) -> Vec<String> {
        use self::Architecture::*;

        let mut defs: Vec<_> = Self::BASE_SWITCHES.iter().cloned().map(Into::into).collect();

        // Version switches.
        defs.push(format!("-DWINVER=0x{:04x}", self.winver.0));
        defs.push(format!("-D_WIN32_WINNT=0x{:04x}", self.winver.0));
        defs.push(format!("-DNTDDI_VERSION=0x{:08x}", self.winver.1));

        // Architecture-specific switches.
        let arch_defines: &[_] = match self.arch {
            X86_32 => Self::X86_32_SWITCHES,
            X86_64 => Self::X86_64_SWITCHES,
            Arm => Self::ARM_SWITCHES,
        };

        defs.extend(arch_defines.iter().cloned().map(Into::into));

        // Done.
        defs
    }
}

#[derive(Clone, Debug)]
pub struct GenConfig {
    pub exp_configs: Vec<ExpConfig>,
    pub switches: Vec<String>,
    pub ignore_decl_spellings: Vec<Regex>,
    pub ignore_file_paths: Vec<Regex>,
}

impl GenConfig {
    fn switches(&self) -> &[String] {
        &self.switches
    }

    fn should_ignore(&self, cursor: &Cursor) -> bool {
        let spelling = cursor.spelling();
        if self.ignore_decl_spellings.iter().any(|r| r.is_match(&spelling)) { return true; }

        let (file, _, _, _) = cursor.location().file_location();
        let file = file.map(|f| f.to_string()).unwrap_or(String::new());
        if self.ignore_file_paths.iter().any(|r| r.is_match(&file)) { return true; }

        false
    }
}

struct Output {
    // [name => [(feat, cconv, annot, decl)]]
    fn_decls: HashMap<String, Vec<(Features, AbsCallConv, String, String)>>,
}

impl Output {
    fn new() -> Self {
        Output {
            fn_decls: HashMap::new(),
        }
    }

    fn add_func_decl(&mut self, name: String, feat: Features, cconv: AbsCallConv, decl: String, annot: String) {
        use std::mem::replace;
        debug!("add_func_decl({:?}, {:?}, {:?}, {:?}, {:?})", name, feat, cconv, decl, annot);

        let decls = self.fn_decls.entry(name).or_insert(vec![]);

        // Is there already a decl which is compatible with this one?
        for &mut (ref mut df, ref dcc, ref dd, ref mut da) in decls.iter_mut() {
            if *dd == decl && *dcc == cconv {
                debug!(".. merging");
                // The decls are the same.  Just combine the feature sets together.
                debug!(".. old feats: {:?}, {:?}", df, feat);
                let new_df = replace(df, Features::default()).or(feat);
                debug!(".. new feats: {:?}", new_df);
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum AbsCallConv {
    ExplicitlyC,
    System,
}

impl AbsCallConv {
    fn as_str(self) -> &'static str {
        use self::AbsCallConv::*;
        match self {
            ExplicitlyC => "C",
            System => "system",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NativeCallConv {
    C,
    Stdcall,
}

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

    // pub fn iter_dummy_tus(&self) -> Vec<Rc<TranslationUnit>> {
    //     fn filter(&(&ref k, _): &(&TuCacheKey, &Rc<TranslationUnit>)) -> bool {
    //         k.1 == ExpConfig::DUMMY_CFG
    //     }

    //     fn map((_, &ref v): (&TuCacheKey, &Rc<TranslationUnit>)) -> Rc<TranslationUnit> {
    //         v.clone()
    //     }

    //     self.cache.iter().filter(filter).map(map).collect()
    // }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TuCacheKey(String, ExpConfig);

impl TuCacheKey {
    pub fn new(path: &str, exp_config: &ExpConfig) -> TuCacheKey {
        TuCacheKey(path.into(), exp_config.clone())
    }
}
