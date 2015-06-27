#![feature(associated_consts)]
#![feature(cstr_to_str)]
#![feature(rc_weak)]

#[macro_use] extern crate log;
extern crate libc;
extern crate num;
extern crate regex;

#[macro_use] mod macros;
pub mod clang;
pub mod util;

use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;

use clang::{Index, RcIndexExt, TranslationUnit, TranslationUnitFlags, Cursor};

pub fn process_header(path: &str, config: &Config) {
    let _output = Output::new();

    let index = Index::create(
        /*exclude_declarations_from_pch*/ false,
        /*display_diagnostics*/ false,
    );

    let mut tu_cache = TuCache::new(index, &config);
    let root_tu = tu_cache.parse_translation_unit(
        path,
        Architecture::X86_32
    ).unwrap();

    for decl_cur in root_tu.cursor().children().into_iter().filter(|cur| !config.should_ignore(cur)).take(50) {
        process_decl(decl_cur, config);
    }
}

pub fn process_decl(decl_cur: Cursor, _config: &Config) {
    debug!("process_decl: {}: {}", decl_cur.location(), decl_cur.spelling());
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
