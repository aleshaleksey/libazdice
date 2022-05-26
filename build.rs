extern crate cbindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = PathBuf::from(&crate_dir).join("cbindgen.toml");
    let config = cbindgen::Config::from_file(path)
        .expect("Config parsing failed, because");

    cbindgen::generate_with_config(crate_dir, config)
      .expect("Unable to generate bindings")
      .write_to_file("include/libazdice.h");
}
