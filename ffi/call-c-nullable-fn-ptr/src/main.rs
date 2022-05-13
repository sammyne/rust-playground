type Callback = extern "C" fn();

pub extern "C" fn helloworld() {
    println!("hello world");
}

extern "C" {
    // @ref: https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html
    // @ref: https://stackoverflow.com/a/54575841/10878967
    fn call_nullable_callback_from_rust(callback: Option<Callback>);
}

fn main() {
    unsafe {
        call_nullable_callback_from_rust(None);
        println!("---");
        call_nullable_callback_from_rust(Some(helloworld));
    }
}
