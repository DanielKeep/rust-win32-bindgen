extern crate env_logger;
extern crate regex;
extern crate win32_bindgen;

fn main() {
    env_logger::init().unwrap();
    try_main().unwrap();
}

fn try_main() -> Result<(), String> {
    let config = win32_bindgen::Config {
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
    };

    win32_bindgen::process_header(
        r#"F:\Programs\MSYS2-64\mingw32\i686-w64-mingw32\include\windows.h"#,
        &config
    );
    Ok(())
}

fn re(re: &str) -> regex::Regex {
    regex::Regex::new(re).unwrap()
}
