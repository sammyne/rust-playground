use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // rebuild if `src/lib.rs` changed
    println!("cargo:rerun-if-changed=src/lib.rs"); // <- NEW!

    // The cbindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    //let bindings = cbindgen::Builder::new()
    //    .with_config(cbindgen::Config::from_root_or_default(&crate_dir))
    //    .with_crate(&crate_dir)
    //    .generate()
    //    // Unwrap the Result and panic on failure.
    //    .expect("Unable to generate bindings");
    // or shortcut as
    let bindings = cbindgen::generate(&crate_dir).expect("Unable to generate bindings");

    // Write the bindings to the $crate_dir/go/hello.h file.
    let out_path = PathBuf::from(&crate_dir).join("go").join("hello.h");
    let _ = fs::remove_file(&out_path);
    if !bindings.write_to_file(out_path) {
        panic!("Couldn't write bindings!");
    }
}
