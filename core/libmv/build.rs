extern crate cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // Load the configuration to ensure namespaces and prefixes are applied
    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Failed to read cbindgen.toml");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/libmv_generated.h");
}