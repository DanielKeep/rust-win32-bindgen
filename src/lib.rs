/*!

See the [Readme](../README.md) for more details.

*/
/*

This module should only contain public-facing things that don't really belong anywhere else, as well as whatever is needed for the public interface.

*/
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
use features::Features;

#[macro_use] mod macros;
mod clang;
mod features;
mod generated;
mod process;
mod util;

pub use generated::winver::WinVersion;
pub use process::process_header;

/**
This represents a single target architecture.
*/
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

/**
This is an "expansion configuration".  It contains all the settings that may need to change between successive expansion passes.
*/
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExpConfig {
    /// Which architecture will this expansion target?
    pub arch: Architecture,

    /**
    What base version of windows will this expansion target?

    Note that the two values here are for the "short" (*i.e.* `_WIN32_WINNT`) and "full" (*i.e.* `NTDDI_VERSION`) version numbers respectively.  *Importantly*, keep in mind that as of Windows 8.1, these two numbers *do not necessarily point to the same version*.
    */
    pub winver: (u16, u32),

    /**
    What is the native calling convention for the target platform.

    > **TODO**: Just derive this from `Architecture`.
    */
    pub native_cc: NativeCallConv,
}

impl ExpConfig {
    /**
    This provides the short and full version numbers for targetting Windows 8.1.
    */
    pub const WINVER_WIN81: (u16, u32) = (0x0602, 0x06030000);

    /**
    This *only* exists so we have something to give the TU cache when we're after tokens.

    Ideally, it should match one of the other expansion configs, but it's not the end of the world if it doesn't.
    */
    const DUMMY_CFG: ExpConfig = ExpConfig {
        arch: Architecture::X86_32,
        winver: ExpConfig::WINVER_WIN81,
        native_cc: NativeCallConv::Stdcall,
    };

    /**
    The Clang switches that need to be included with every expansion.
    */
    const BASE_SWITCHES: &'static [&'static str] = &[
        "-D_WIN32",

        // This includes both Desktop and App partitions.
        "-DWINAPI_FAMILY=WINAPI_FAMILY_DESKTOP_APP"
    ];

    /// Clang switches for X86_32 passes.
    const X86_32_SWITCHES: &'static [&'static str] = &[
        "--target=i686-pc-windows-gnu", "-D__X86__", "-D_M_IX86"
    ];

    /// Clang switches for X86_64 passes.
    const X86_64_SWITCHES: &'static [&'static str] = &[
        "--target=x86_64-pc-windows-gnu", "-D_AMD64_", "-D_M_AMD64"
    ];

    /// Clang switches for Arm passes.
    const ARM_SWITCHES: &'static [&'static str] = &[
        "--target=arm-pc-windows-gnu", "-D_ARM_", "-D_M_ARM"
    ];

    /// Return the switches needed for this config.
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

/**
This provides the bits of configuration that *do not* change between passes.
*/
#[derive(Clone, Debug)]
pub struct GenConfig {
    /**
    In order to get a complete set of bindings, the Windows headers must be processed more than once.  Each one of these represents a single pass, each with different settings.
    */
    pub exp_configs: Vec<ExpConfig>,

    /**
    These are extra switches that should be added to the Clang command line.

    These are added *before* any expansion-specific switches.
    */
    pub switches: Vec<String>,

    /**
    Any declaration whose spelling matches *any* of these regular expressions will be ignored.  That is, the processor will *not* attempt to generate a Rust binding for it.
    */
    pub ignore_decl_spellings: Vec<Regex>,

    /**
    Like `ignore_decl_spellings`, except that this works based on the path to the file that contains a declaration.  This is useful for excluding "internal" headers that shouldn't be publically exposed.
    */
    pub ignore_file_paths: Vec<Regex>,
}

impl GenConfig {
    /// Return the switches that should be passed to Clang, indepedent of expansion.
    fn switches(&self) -> &[String] {
        &self.switches
    }

    /// Determines whether or not the declaration at the given `Cursor` should be ignored.
    fn should_ignore(&self, cursor: &clang::Cursor) -> bool {
        let spelling = cursor.spelling();
        if self.ignore_decl_spellings.iter().any(|r| r.is_match(&spelling)) { return true; }

        let (file, _, _, _) = cursor.location().file_location();
        let file = file.map(|f| f.to_string()).unwrap_or(String::new());
        if self.ignore_file_paths.iter().any(|r| r.is_match(&file)) { return true; }

        false
    }
}

/**
Represents the calling convention used "natively" by a target architecture.  This is what the majority of Windows API calls uses.
*/
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NativeCallConv {
    C,
    Stdcall,
}
