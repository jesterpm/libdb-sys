extern crate bindgen;

use std::path::{PathBuf, Path};
use std::process::Command;

pub const SOURCE: &'static str = "db-5.3.21";

pub fn build_unix(out_dir: &str) {
    let copy_dir = Path::new(out_dir).join(SOURCE);
    let build_dir = copy_dir.join("build_unix");

    // copy source to out_dir
    Command::new("cp")
        .arg("-r")
        .arg(Path::new("vendor").join(SOURCE).to_str().unwrap())
        .arg(copy_dir.to_str().unwrap())
        .status()
        .unwrap();

    // build
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
    let build_dir = Path::new(out_dir).join(SOURCE).join("build_unix");
    let bindings = bindgen::Builder::default()
        .header(build_dir.join("db.h").to_str().unwrap())
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
