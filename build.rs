use std::env;

mod build_4_8;
mod build_5_3;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    if cfg!(feature = "v5_3") {
        build_5_3::build_unix(&out_dir);
        build_5_3::generate_bindings(&out_dir);
    } else if cfg!(feature = "v4_8") {
        build_4_8::build_unix(&out_dir);
        build_4_8::generate_bindings(&out_dir);
    } else {
        // default
        build_5_3::build_unix(&out_dir);
        build_5_3::generate_bindings(&out_dir);
    }
}
