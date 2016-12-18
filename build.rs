extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

pub const SOURCE_DIR: &'static str = "db-4.8.30";

fn bindgen(dir: &str) {
    let db_h_file = Path::new(dir).join("include").join("db.h");
    let mut bindings = bindgen::Builder::new(db_h_file.to_string_lossy().into_owned());
    bindings.link("db", bindgen::LinkType::Dynamic);
    bindings.builtins();
    bindings.convert_macros(true);
    bindings.macro_int_types(["uint", "uint", "uint", "ulong", "sint", "sint", "sint", "slong"].iter().map(|r| *r));

    let generated_bindings = bindings.generate().expect("Failed to generate bindings");
    let mut file = File::create("src/ffi.rs").expect("Failed to open file");
    file.write(generated_bindings.to_string().as_bytes()).unwrap();
}

fn build_unix(out_dir: &str) {
    let build_dir = Path::new(SOURCE_DIR).join("build_unix");
    Command::new("../dist/configure")
        .arg(&format!("--prefix={}", out_dir))
        .arg("--with-pic")
        .current_dir(&build_dir)
        .status()
        .unwrap();
    Command::new("make")
        .current_dir(&build_dir)
        .status()
        .unwrap();
    Command::new("make")
        .arg(&"install")
        .current_dir(&build_dir)
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", Path::new(&out_dir).join("lib").to_string_lossy());
    println!("cargo:rustc-link-lib=static=db");
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    build_unix(&out_dir);
    bindgen(&out_dir);
}