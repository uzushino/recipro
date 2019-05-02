extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = PathBuf::from(manifest_dir);

    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-std=c++17")
        .file("src/c/binding.cc")
        .file("src/c/isolate.cc")
        .file("src/c/recipro.cc")
        .include("src/c")
        .include("v8")
        .compile("binding");

    println!("cargo:rustc-link-lib=static=v8_monolith");
    println!(
        "cargo:rustc-link-search=native={}",
        src_dir.join("out/v8build/obj").display()
    );
}
