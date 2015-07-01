#[macro_use] extern crate log;
#[macro_use] extern crate rustc_serialize;
extern crate env_logger;
extern crate regex;
extern crate win32_bindgen;

use std::collections::HashMap;
use win32_bindgen as bg;

fn main() {
    env_logger::init().unwrap();
    let result = try_main();
    result.unwrap();
}

fn try_main() -> Result<(), String> {
    info!("Running with local\\config.json...");
    let json_config: Config = rustc_serialize::json::decode(&read_file("local/config.json")).unwrap();
    let header = json_config.header;
    let gen_config = json_config.generation.into_gen_config();
    let out_config = json_config.output.into_out_config();
    bg::process_header(&header, &gen_config, &out_config);
    Ok(())
}

fn re(re: &str) -> regex::Regex {
    regex::Regex::new(re).unwrap()
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub header: String,
    pub generation: GenConfig,
    pub output: OutConfig,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct GenConfig {
    pub expansion_configs: Vec<ExpConfig>,
    pub ignore_decl_spellings: Vec<String>,
    pub ignore_file_paths: Vec<String>,
    pub switches: Vec<String>,
    pub non_canonical_tag_names: Vec<String>,
}

impl GenConfig {
    pub fn into_gen_config(self) -> bg::GenConfig {
        bg::GenConfig {
            exp_configs: self.expansion_configs.into_iter().map(ExpConfig::into_exp_config).collect(),
            ignore_decl_spellings: self.ignore_decl_spellings.into_iter().map(|s| re(&s)).collect(),
            ignore_file_paths: self.ignore_file_paths.into_iter().map(|s| re(&s)).collect(),
            switches: self.switches,
            non_canonical_tag_names: self.non_canonical_tag_names.into_iter().map(|s| re(&s)).collect(),
        }
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct OutConfig {
    pub output_dir: String,
    pub header_path: String,
    pub library_path: String,
    pub function_library_map: String,
    pub function_library_fallback: String,
}

impl OutConfig {
    pub fn into_out_config(self) -> bg::OutConfig {
        bg::OutConfig {
            output_dir: self.output_dir,
            header_path: self.header_path,
            library_path: self.library_path,
            function_library_map: read_symbol_list(&self.function_library_map),
            function_library_fallbacks: vec![self.function_library_fallback],
        }
    }
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct ExpConfig {
    pub architecture: Architecture,
    pub windows_version_short: String,
    pub windows_version_full: String,
    pub native_calling_conv: NativeCallConv,
}

impl ExpConfig {
    pub fn into_exp_config(self) -> bg::ExpConfig {
        bg::ExpConfig {
            arch: self.architecture.into_architecture(),
            winver: ((wv(&self.windows_version_short) >> 16) as u16, wv(&self.windows_version_full)),
            native_cc: self.native_calling_conv.into_native_call_conv(),
        }
    }
}

fn wv(s: &str) -> u32 {
    bg::WinVersion::from_name(s).unwrap() as u32
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum Architecture {
    X86_32,
    X86_64,
    Arm,
}

impl Architecture {
    pub fn into_architecture(self) -> bg::Architecture {
        use self::Architecture::*;
        match self {
            X86_32 => bg::Architecture::X86_32,
            X86_64 => bg::Architecture::X86_64,
            Arm => bg::Architecture::Arm,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub enum NativeCallConv {
    C,
    Stdcall,
}

impl NativeCallConv {
    pub fn into_native_call_conv(self) -> bg::NativeCallConv {
        use self::NativeCallConv::*;
        match self {
            C => bg::NativeCallConv::C,
            Stdcall => bg::NativeCallConv::Stdcall,
        }
    }
}

pub fn read_symbol_list(path: &str) -> HashMap<String, Vec<String>> {
    use std::fs;
    use std::io;
    use std::io::prelude::*;
    let mut map = HashMap::new();
    for line in io::BufReader::new(fs::File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 || line.starts_with("#") { continue; }

        let mut parts = line.splitn(2, ":");
        let sym_name = parts.next().unwrap();
        let sym_libs = parts.next().unwrap();

        let sym_libs = sym_libs.split_whitespace().map(|s| s.into()).collect();

        map.insert(sym_name.into(), sym_libs);
    }
    map
}

pub fn read_file(path: &str) -> String {
    use std::fs;
    use std::io::prelude::*;
    let mut s = String::new();
    let _ = fs::File::open(path).unwrap().read_to_string(&mut s).unwrap();
    s
}
