#[macro_use] extern crate log;
extern crate env_logger;
extern crate regex;
extern crate win32_bindgen;

use win32_bindgen::{Architecture, ExpConfig, GenConfig, NativeCallConv};

pub const USE_MINGW_HEADERS: bool = false;

fn main() {
    env_logger::init().unwrap();
    (if USE_MINGW_HEADERS { try_main_mingw() } else { try_main_msvc() }).unwrap();
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

fn try_main_mingw() -> Result<(), String> {
    info!("Running with MinGW headers...");
    let gen_config = GenConfig {
        exp_configs: EXP_CONFIGS,
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
        switches: &[
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
        exp_configs: EXP_CONFIGS,
        ignore_decl_spellings: vec![re("^_"), re("__")],
        ignore_file_paths: vec![
            re(r#"^$"#),
            // The sanity checks are a pain to parse.
            re(r#"\\sdkddkver.h"#),
        ],
        switches: &[
            r#"-fms-extensions"#, r#"-fms-compatibility"#,
            r#"-D_MSC_VER=1800"#,
            r#"-D_STDCALL_SUPPORTED"#,
            r#"-Ilocal/Include/shared"#,
            r#"-Ilocal/Include/um"#,
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
