extern crate bindgen;

use std::path::{PathBuf, Path};
use std::process::Command;

pub const SOURCE_DIR: &'static str = "db-5.3.21";

pub fn build_unix(out_dir: &str) {
    let build_dir = Path::new(SOURCE_DIR).join("build_unix");
    Command::new("../dist/configure")
        .arg(&format!("--prefix={}", out_dir))
        .arg("--with-gnu-ld")
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

pub fn generate_bindings(out_dir: &str) {
    let bindings = bindgen::Builder::default()
        .header("db-5.3.21/build_unix/db.h")
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(&out_dir);
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");
}
