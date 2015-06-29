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
    Cursor,
};

use features::{
    Features,
    define_feature, define_feature_expr,
};

struct Cache<'config> {
    tu: TuCache<'config>,
    features: HashMap<String, BTreeMap<u32, Features>>,
}

impl<'config> Cache<'config> {
    fn new(index: Rc<Index>, config: &'config Config) -> Self {
        Cache {
            tu: TuCache::new(index, config),
            features: HashMap::new(),
        }
    }
}

pub fn process_header(path: &str, config: &Config) {
    let _output = Output::new();

    let index = Index::create(
        /*exclude_declarations_from_pch*/ false,
        /*display_diagnostics*/ false,
    );

    let mut cache = Cache::new(index, &config);
    let root_tu = cache.tu.parse_translation_unit(
        path,
        Architecture::X86_32
    ).unwrap();

    for decl_cur in root_tu.cursor().children().into_iter().filter(|cur| !config.should_ignore(cur)) {
        process_decl(decl_cur, &mut cache, Architecture::X86_32);
    }
}

fn process_decl(decl_cur: Cursor, cache: &mut Cache, _arch: Architecture) {
    use clang::CursorKind as CK;

    let decl_kind = match decl_cur.kind() {
        Some(CK::InclusionDirective)
        | Some(CK::MacroInstantiation)
        | None => return,
        Some(kind) => kind
    };

    let decl_loc = decl_cur.location();

    debug!("process_decl: {}: {:?} {}",
        decl_loc.display_short(),
        decl_kind,
        decl_cur.spelling());

    let feat = decl_loc.file()
        .map(|file| get_features_at(file, decl_loc.line(), cache))
        .unwrap_or_else(|| Features::default());

    debug!("process_decl feat: {:?}", feat);
}

fn get_features_at(file: clang::File, line: u32, cache: &mut Cache) -> Features {
    use std::collections::Bound;

    debug!("get_features_at({:?}, {}, _)", file.file_name(), line);
    let path = file.file_name();

    let tu_cache = &mut cache.tu;

    let fmap = cache.features.entry(path.clone()).or_insert_with(||
        get_features(get_token_lines(file, tu_cache))
    );

    fmap.range(Bound::Included(&line), Bound::Unbounded).next()
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| Features::default())
}

fn get_token_lines(file: clang::File, tu_cache: &mut TuCache) -> Vec<(u32, Vec<clang::Token>)> {
    debug!("get_token_lines({:?}, _)", file.file_name());
    let path = file.file_name();

    // Architecture shouldn't matter since we just want the tokens.
    let tu = tu_cache.parse_translation_unit(&path, Architecture::X86_32).unwrap();

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
                map.insert(line_num, stack.last().expect("non-empty stack").clone());
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
}

pub struct Config {
    pub ignore_decl_spellings: Vec<Regex>,
    pub ignore_file_paths: Vec<Regex>,
}

impl Config {
    fn defines_for_arch(&self, arch: Architecture) -> Vec<String> {
        use self::Architecture::*;

        const ALL: &'static [&'static str] = &[
            // Because always.
            "_WIN32",

            // Windows 8.1.
            "WINVER=0x0602",
            "_WIN32_WINNT=0x0602",
            "NTDDI_VERSION=0x06030000",

            // Both Desktop and App partitions.
            "WINAPI_FAMILY=WINAPI_FAMILY_DESKTOP_APP",
        ];

        const X86_32_DEFINES: &'static [&'static str] = &["__X86__", "__i386__", "_M_IX86"];
        const X86_64_DEFINES: &'static [&'static str] = &["_WIN64", "_AMD64_", "__x86_64", "__x86_64__", "_M_AMD64", "_M_X64"];

        let arch_defines: &[_] = match arch {
            X86_32 => X86_32_DEFINES,
            X86_64 => X86_64_DEFINES,
        };

        fn as_def(s: &&str) -> String { format!("-D{}", s) }
        ALL.iter().map(as_def).chain(arch_defines.iter().map(as_def)).collect()
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

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }
}

pub struct TuCache<'c> {
    index: Rc<Index>,
    cache: HashMap<TuCacheKey, Rc<TranslationUnit>>,
    config: &'c Config,
}

impl<'c> TuCache<'c> {
    pub fn new(index: Rc<Index>, config: &'c Config) -> TuCache<'c> {
        TuCache {
            index: index,
            cache: HashMap::new(),
            config: config,
        }
    }

    pub fn parse_translation_unit(
        &mut self,
        path: &str,
        arch: Architecture,
    ) -> Result<Rc<TranslationUnit>, clang::ErrorCode> {
        let index_opts = TranslationUnitFlags::None
            | TranslationUnitFlags::DetailedPreprocessingRecord
            | TranslationUnitFlags::Incomplete
            ;

        let key = TuCacheKey::new(path, arch);

        if let Some(rc_tu) = self.cache.get(&key) {
            return Ok(rc_tu.clone())
        }

        let tu = try!(self.index.parse_translation_unit(
            path,
            &self.config.defines_for_arch(arch),
            &[],
            index_opts,
        ));
        self.cache.insert(key, tu.clone());
        Ok(tu)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TuCacheKey(String, Architecture);

impl TuCacheKey {
    pub fn new(path: &str, arch: Architecture) -> TuCacheKey {
        TuCacheKey(path.into(), arch)
    }
}
