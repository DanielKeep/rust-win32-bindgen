#![feature(associated_consts)]
#![feature(cstr_to_str)]
#![feature(rc_weak)]

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
    Features, PrimaryBranch,
    has_important_defines,
    define_feature, define_feature_complement, define_feature_expr,
};

struct Cache<'config> {
    tu: TuCache<'config>,
}

impl<'config> Cache<'config> {
    fn new(index: Rc<Index>, config: &'config Config) -> Self {
        Cache {
            tu: TuCache::new(index, config),
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

fn process_decl(decl_cur: Cursor, _cache: &mut Cache, _arch: Architecture) {
    use clang::CursorKind as CK;

    let decl_kind = match decl_cur.kind() {
        Some(CK::InclusionDirective)
        | Some(CK::MacroInstantiation)
        | None => return,
        Some(kind) => kind
    };

    debug!("process_decl: {}: {:?} {}",
        decl_cur.location().display_short(),
        decl_kind,
        decl_cur.spelling());

    // LASTEDIT: feat = get_features_at(decl.location.file.name, decl.location.line)
}

fn _get_token_lines(file: clang::File, cache: &mut Cache) -> Vec<(u32, Vec<clang::Token>)> {
    use std::collections::HashMap;

    let path = file.file_name();

    // Architecture shouldn't matter since we just want the tokens.
    let tu = cache.tu.parse_translation_unit(&path, Architecture::X86_32).unwrap();

    // Get the set of line numbers which *contain* a line continuation.
    let cont_lines: Vec<_> = util::read_lines(&path)
        .map(|rs| rs.unwrap())
        .enumerate()
        .filter(|&(_, ref s)| s.trim_right().ends_with("\\"))
        .map(|(i, _)| i as u32) // TODO: checked
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

fn _get_features(tls: Vec<(u32, Vec<clang::Token>)>) -> BTreeMap<u32, Features> {
    /*
    The way this works is that we have to walk through *all* the lines, looking for preprocessor conditional compilation directives.  When we find them, we interpret them and push the enabled feature tests on to the stack.  Then, when we find something that *isn't* a conditional directive *and* the features have changed since the last time we did so, we add an entry to the map.
    */
    let mut stack = vec![Features::default()];

    fn fd() -> Features { Features::default() }

    fn seq(lhs: &[String], rhs: &[&'static str]) -> bool {
        lhs.len() == rhs.len()
            && lhs.iter().zip(rhs.iter()).all(|(l,r)| &**l == *r)
    }

    fn append(_stack: &Vec<Features>, _f: Features) {
        unimplemented!()
    }

    let mut map = BTreeMap::new();

    // Insert fallback.
    map.insert(0, fd());

    for (line_num, toks) in tls {
        let loc = toks[0].location();
        let ts: Vec<_> = toks.iter().map(|t| t.spelling()).collect();

        if seq(&ts[..2], &["#", "if"]) {
            if has_important_defines(&ts[2..]) {
                append(&mut stack, define_feature_expr(&ts[2..], PrimaryBranch::Yes, &loc).unwrap_or(fd()));
            } else {
                append(&mut stack, fd());
            }
        } else if seq(&ts[..2], &["#", "ifdef"]) && ts.len() == 3 {
            if has_important_defines(&ts[2..]) {
                append(&mut stack, define_feature(&ts[2]).unwrap_or(fd()));
            } else {
                append(&mut stack, fd());
            }
        } else if seq(&ts[..2], &["#", "ifndef"]) && ts.len() == 3 {
            if has_important_defines(&ts[2..]) {
                append(&mut stack, define_feature_complement(&ts[2]).unwrap_or(fd()));
            } else {
                append(&mut stack, fd());
            }
        } else if seq(&ts[..2], &["#", "elif"]) {
            stack.pop();
            if has_important_defines(&ts[2..]) {
                append(&mut stack, define_feature_expr(&ts[2..], PrimaryBranch::Yes, &loc).unwrap_or(fd()));
            } else {
                append(&mut stack, fd());
            }
        } else if seq(&ts[..2], &["#", "else"]) {
            stack.pop();
            append(&mut stack, fd());
        } else if seq(&ts[..2], &["#", "endif"]) {
            stack.pop();
        } else {
            // Work out what the last set of features we've seen is.
            let do_insert = {
                let prev_feat = map.values().next_back().unwrap();
                *prev_feat != *stack.last().unwrap()
            };
            if do_insert {
                map.insert(line_num, stack.last().unwrap().clone());
            }
        }
    }

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
