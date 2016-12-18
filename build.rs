use std::env;
use std::path::Path;
use std::process::Command;

pub const SOURCE_DIR: &'static str = "db-4.8.30";

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
}
