#[macro_use] extern crate log;
#[macro_use] extern crate rustc_serialize;
extern crate env_logger;
extern crate regex;
extern crate win32_bindgen;

use win32_bindgen::{Architecture, ExpConfig, GenConfig, NativeCallConv};

pub const USE_MINGW_HEADERS: bool = false;
pub const USE_MSVC_HEADERS: bool = false;

fn main() {
    env_logger::init().unwrap();
    let result = if USE_MINGW_HEADERS {
        try_main_mingw()
    } else if USE_MSVC_HEADERS {
        try_main_msvc()
    } else {
        try_main_json()
    };
    result.unwrap();
}

const EXP_CONFIGS: &'static [ExpConfig] = &[
    ExpConfig {
        arch: Architecture::X86_32,
        winver: ExpConfig::WINVER_WIN81,
        native_cc: NativeCallConv::Stdcall,
    },
    ExpConfig {
        arch: Architecture::X86_64,
        winver: ExpConfig::WINVER_WIN81,
        native_cc: NativeCallConv::C,
    },
    ExpConfig {
        arch: Architecture::Arm,
        winver: ExpConfig::WINVER_WIN81,
        native_cc: NativeCallConv::C,
    },
];

fn try_main_json() -> Result<(), String> {
    info!("Running with local\\config.json...");
    let json_config: Config = rustc_serialize::json::decode(&read_file("local/config.json")).unwrap();
    let header = json_config.header.clone();
    let gen_config = json_config.into_gen_config();
    win32_bindgen::process_header(&header, &gen_config);
    Ok(())
}

fn try_main_mingw() -> Result<(), String> {
    info!("Running with MinGW headers...");
    let gen_config = GenConfig {
        exp_configs: EXP_CONFIGS.iter().cloned().collect(),
        ignore_decl_spellings: vec![re("^_"), re("__")],
        ignore_file_paths: vec![
            re(r#"^$"#),
            re(r#"\\_mingw.h"#), re(r#"\\_mingw_secapi.h"#),
            re(r#"\\sdks\\_mingw_directx.h"#), re(r#"\\sdks\\_mingw_ddk.h"#),
            re(r#"\\psdk_inc\\intrin-impl.h"#),
            re(r#"\\ctype.h"#), re(r#"\\excpt.h"#), re(r#"\\string.h"#),
            re(r#"\\sec_api\\string_s.h"#),
            re(r#"\\winapifamily.h"#), re(r#"\\apiset.h"#),
            re(r#"\\sal.h"#), re(r#"\\specstrings.h"#),
        ],
        switches: vec![
        ],
    };

    win32_bindgen::process_header(
        // r#"F:\Programs\MSYS2-64\mingw32\i686-w64-mingw32\include\windows.h"#,
        r#"F:\Programs\MSYS2-64\mingw32\i686-w64-mingw32\include\winnt.h"#,
        &gen_config
    );
    Ok(())
}

fn try_main_msvc() -> Result<(), String> {
    info!("Running with Windows SDK headers...");
    let gen_config = GenConfig {
        exp_configs: EXP_CONFIGS.iter().cloned().collect(),
        ignore_decl_spellings: vec![re("^_"), re("__")],
        ignore_file_paths: vec![
            re(r#"^$"#),
            // The sanity checks are a pain to parse.
            re(r#"\\sdkddkver.h"#),
        ],
        switches: vec![
            r#"-fms-extensions"#.into(), r#"-fms-compatibility"#.into(),
            r#"-D_MSC_VER=1800"#.into(),
            r#"-D_STDCALL_SUPPORTED"#.into(),
            r#"-Ilocal/Include/shared"#.into(),
            r#"-Ilocal/Include/um"#.into(),
        ],
    };

    win32_bindgen::process_header(
        r#"local\Include\um\winnt.h"#,
        &gen_config
    );
    Ok(())
}

fn re(re: &str) -> regex::Regex {
    regex::Regex::new(re).unwrap()
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub header: String,
    pub expansion_configs: Vec<ExpansionConfig>,
    pub ignore_decl_spellings: Vec<String>,
    pub ignore_file_paths: Vec<String>,
    pub switches: Vec<String>,
}

impl Config {
    pub fn into_gen_config(self) -> GenConfig {
        GenConfig {
            exp_configs: self.expansion_configs.into_iter().map(ExpansionConfig::into_exp_config).collect(),
            ignore_decl_spellings: self.ignore_decl_spellings.into_iter().map(|s| re(&s)).collect(),
            ignore_file_paths: self.ignore_file_paths.into_iter().map(|s| re(&s)).collect(),
            switches: self.switches,
        }
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct ExpansionConfig {
    pub architecture: ConfigArch,
    pub windows_version_short: String,
    pub windows_version_full: String,
    pub native_calling_conv: ConfigNCC,
}

impl ExpansionConfig {
    pub fn into_exp_config(self) -> ExpConfig {
        ExpConfig {
            arch: self.architecture.into_architecture(),
            winver: ((wv(&self.windows_version_short) >> 16) as u16, wv(&self.windows_version_full)),
            native_cc: self.native_calling_conv.into_native_call_conv(),
        }
    }
}

fn wv(s: &str) -> u32 {
    win32_bindgen::generated::winver::WinVersion::from_name(s).unwrap() as u32
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ConfigArch {
    X86_32,
    X86_64,
    Arm,
}

impl ConfigArch {
    pub fn into_architecture(self) -> Architecture {
        use self::ConfigArch::*;
        match self {
            X86_32 => Architecture::X86_32,
            X86_64 => Architecture::X86_64,
            Arm => Architecture::Arm,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum ConfigNCC {
    C,
    Stdcall,
}

impl ConfigNCC {
    pub fn into_native_call_conv(self) -> NativeCallConv {
        use self::ConfigNCC::*;
        match self {
            C => NativeCallConv::C,
            Stdcall => NativeCallConv::Stdcall,
        }
    }
}

pub fn read_file(path: &str) -> String {
    use std::fs;
    use std::io::prelude::*;
    let mut s = String::new();
    let _ = fs::File::open(path).unwrap().read_to_string(&mut s).unwrap();
    s
}
