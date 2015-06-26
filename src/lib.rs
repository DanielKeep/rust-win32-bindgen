#![feature(associated_consts)]

/*

LASTEDIT: I can't find out what the lifetime semantics for Cursors are... so, fuck it.  Just wrap fucking *everything* in Rcs and make each object hold a reference to the thing it was created from.

*/

extern crate libc;
extern crate num;

#[macro_use] mod macros;
pub mod clang;
pub mod util;

use std::collections::HashMap;
use std::rc::Rc;

use clang::{Index, TranslationUnit, TranslationUnitFlags};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Architecture {
    X86_32,
    X86_64,
}

pub struct Config;

impl Config {
    fn defines_for_arch(&self, arch: Architecture) -> Vec<String> {
        use self::Architecture::*;

        const X86_32_DEFINES: &'static [&'static str] = &["__X86__", "__i386__", "_M_IX86"];
        const X86_64_DEFINES: &'static [&'static str] = &["_WIN64", "_AMD64_", "__x86_64", "__x86_64__", "_M_AMD64", "_M_X64"];

        let defines: &[_] = match arch {
            X86_32 => X86_32_DEFINES,
            X86_64 => X86_64_DEFINES,
        };
        defines.iter().map(|&s| format!("-D{}", s)).collect()
    }
}

pub fn process_header(_path: &str, config: &Config) {
    let _output = Output::new();

    let index = Index::create(
        /*exclude_declarations_from_pch*/ false,
        /*display_diagnostics*/ false,
    );

    let mut tu_cache = TuCache::new(&index, &config);
    let _tu = tu_cache.parse_translation_unit(
        r#"F:\Programs\MSYS2-64\mingw32\i686-w64-mingw32\include\windows.h"#,
        Architecture::X86_32
    );
}

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }
}

pub struct TuCache<'c, 'i> {
    index: &'i Index,
    cache: HashMap<TuCacheKey, Rc<TranslationUnit<'i>>>,
    config: &'c Config,
}

impl<'c, 'i> TuCache<'c, 'i> {
    pub fn new(index: &'i Index, config: &'c Config) -> TuCache<'c, 'i> {
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
    ) -> Rc<TranslationUnit<'i>> {
        let index_opts = TranslationUnitFlags::None
            | TranslationUnitFlags::DetailedPreprocessingRecord
            | TranslationUnitFlags::Incomplete
            ;

        let key = TuCacheKey::new(path, arch);

        if let Some(rc_tu) = self.cache.get(&key) {
            return rc_tu.clone()
        }

        let tu = self.index.parse_translation_unit(
            path,
            &self.config.defines_for_arch(arch),
            &[],
            index_opts,
        );
        let tu = Rc::new(tu);
        self.cache.insert(key, tu.clone());
        tu
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TuCacheKey(String, Architecture);

impl TuCacheKey {
    pub fn new(path: &str, arch: Architecture) -> TuCacheKey {
        TuCacheKey(path.into(), arch)
    }
}
