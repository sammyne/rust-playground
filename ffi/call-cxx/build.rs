fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/hello.cc")
        .compile("hello")
}
