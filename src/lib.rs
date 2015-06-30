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

use regex::Regex;
use clang::Cursor;
use features::Features;

#[macro_use] mod macros;
pub mod clang;
pub mod features;
pub mod generated;
pub mod process;
pub mod util;

pub use generated::winver::WinVersion;
pub use process::process_header;

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NativeCallConv {
    C,
    Stdcall,
}
