use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("$OUT_DIR not set. Please build with cargo");

    let lib_hello = "hello";
    cc::Build::new().file("hello.s").compile(lib_hello);

    // rebuild if `hello.s` changed
    println!("cargo:rerun-if-changed=hello.s"); // <- NEW!

    // extend the library search path
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib={}", lib_hello);
}
