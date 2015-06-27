use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bin_dir = manifest_path.join("bin").join(env::var("TARGET").unwrap());

    println!("cargo:rustc-link-lib=clang");
    println!("cargo:rustc-link-search={}", bin_dir.to_str().unwrap());
}
