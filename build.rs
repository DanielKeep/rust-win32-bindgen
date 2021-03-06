/*!
The build script has two primary jobs:

1. Do code generation.  Currently, this consists of turning `data/winver.json` into an appropriate `enum`.

2. Tell Cargo to link against Clang.
*/
extern crate itertools;
extern crate serde;

use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use itertools::Itertools;

/// `pj` as in "path join".
macro_rules! pj {
    ($p0:expr, $($ps:expr),*) => {
        {
            let mut pb = PathBuf::from($p0);
            $(pb.push($ps);)*
            pb
        }
    }
}

fn main() {
    let self_path = pj!(get_manifest_dir(), "build.rs");
    let gen_path = make_gen_dir();
    let data_path = get_data_dir();

    {
        let winver_rs = pj!(&gen_path, "winver.rs");
        let winver_json = pj!(&data_path, "winver.json");
        if is_target_stale(&winver_rs, &[&self_path, &winver_json]) {
            let mut f = fs::File::create(winver_rs).ok().expect("create winver.rs");
            gen_winver_enum(&mut f, &read_file_str(winver_json));
            f.flush().unwrap();
        }
    }

    link_clang();
}

fn make_gen_dir() -> PathBuf {
    let gen_path = pj!(get_out_dir(), "src");
    let _ = fs::create_dir(&gen_path).ok();
    gen_path
}

fn get_data_dir() -> PathBuf {
    pj!(get_manifest_dir(), "data")
}

#[cfg(windows)]
fn is_target_stale<P0, P1>(target: P0, dep_paths: &[P1]) -> bool
where P0: AsRef<Path>, P1: AsRef<Path> {
    use std::os::windows::fs::MetadataExt;
    let target_ts = fs::metadata(&target).map(|md| md.last_write_time()).unwrap_or(0);
    dep_paths.iter().any(|dp| fs::metadata(dp).map(|md| md.last_write_time()).unwrap_or(1) > target_ts)
}

fn gen_winver_enum<W>(out: &mut W, json_str: &str) where W: io::Write {
    use std::collections::HashMap;
    use serde::json;

    println!("# gen_winver_enum(..)");

    let root: HashMap<String, String> = json::from_str(json_str).unwrap();
    
    // First, parse those version numbers!
    let root: HashMap<_, u32> = root.into_iter()
        .map(|(k, v)| (k, parse_int(&v)))
        .collect();

    // Make the map of "primary" names.
    let primary: HashMap<&str, u32> = root.iter()
        .filter(|&(ref k, _)| !k.starts_with("*"))
        .map(|(k, v)| (&**k, *v))
        .collect();

    // Make the reverse lookup map.
    let reverse: HashMap<u32, &str> = primary.iter().map(|(k, v)| (*v, *k)).collect();
    assert_eq!(primary.len(), reverse.len());

    // Make the map of non-primary aliases.
    let mut aliases: Vec<(&str, u32)> = root.iter().filter(|&(k, _)| k.starts_with("*"))
        .map(|(k, v)| (&k[1..], *v)).collect();
    aliases.sort();

    // Get a sorted list of versions.
    let mut vers: Vec<u32> = primary.values().cloned().collect();
    vers.sort();

    // Use that to get a "next" version map.
    let next_ver_iter = vers.iter().cloned().skip(1).chain(Some(vers.last().unwrap() + 1).into_iter());
    let next_ver: HashMap<u32, u32> = vers.iter().cloned().zip(next_ver_iter).collect();

    // Generate the enum.
    write!(out, r#"
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum WinVersion {{
{primary_variants}
}}

impl WinVersion {{
{alias_consts}

    pub const AFTER_LAST: u32 = 0x{guard_const:08x};

    pub fn from_name(name: &str) -> Option<WinVersion> {{
        match name {{
{from_names}
            _ => None
        }}
    }}

    pub fn next_version(self) -> Option<WinVersion> {{
        match self {{
{next_versions}
            _ => None
        }}
    }}

    pub fn from_u32_round_up(v: u32) -> Option<WinVersion> {{
{from_u32_round_ups}
        None
    }}
}}
"#,
        primary_variants = vers.iter().cloned()
            .map(|v| format!("    {:<8} = 0x{:08x},", reverse[&v], v))
            .join("\n"),

        alias_consts = aliases.iter()
            .map(|&(k, v)| format!("    pub const {:<7}: WinVersion = WinVersion::{};", k, reverse[&v]))
            .join("\n"),

        guard_const = next_ver[vers.last().unwrap()],

        from_names = primary.iter().map(|(&k, &v)| (k, v)).chain(aliases.iter().map(|&(k, v)| (k, v)))
            .map(|(k, v)| format!("            \"{}\" => Some(WinVersion::{}),",
                k, reverse[&v]))
            .join("\n"),

        next_versions = vers.iter().cloned()
            .filter(|&k| reverse.contains_key(&next_ver[&k]))
            .map(|k| format!("            WinVersion::{:<8} => Some(WinVersion::{}),",
                reverse[&k],
                reverse[&next_ver[&k]]))
            .join("\n"),

        from_u32_round_ups = vers.iter().cloned()
            .map(|k| format!("        if v <= 0x{:08x} {{ return Some(WinVersion::{}); }}",
                k, reverse[&k]))
            .join("\n"),
    ).unwrap();
}

fn link_clang() {
    let manifest_path = get_manifest_dir();
    let bin_dir = pj!(manifest_path, "bin", get_target());

    println!("cargo:rustc-link-lib=clang");
    println!("cargo:rustc-link-search={}", bin_dir.to_str().unwrap());
}

fn parse_int(s: &str) -> u32 {
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).unwrap()
    } else {
        s.parse().unwrap()
    }
}

fn read_file_str<P>(path: P) -> String
where P: AsRef<Path> + ::std::fmt::Debug {
    let mut s = String::new();
    let _ = fs::File::open(&path).ok().expect(&format!("open {:?}", path))
        .read_to_string(&mut s).ok().expect(&format!("read from {:?}", path));
    s
}

fn get_manifest_dir() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()).into()
}

fn get_out_dir() -> PathBuf {
    env::var("OUT_DIR").ok().expect("OUT_DIR *must* be set").into()
}

fn get_target() -> PathBuf {
    env::var("TARGET").unwrap_or_else(|_| "i686-pc-windows-gnu".into()).into()
}
