use std::ffi;
use std::os::raw;

type Callback = extern "C" fn(v: i32) -> i32;

pub extern "C" fn increment(v: i32) -> i32 {
    v + 1
}

extern "C" {
    fn add_one(arr: *mut u8, arr_len: usize);
    fn say_hello(who: *const raw::c_char);

    fn free_hello(h: *mut raw::c_void);
    fn new_hello() -> *mut raw::c_void;
    fn print_hello(h: *const raw::c_void);

    fn call_callback_from_rust(v: i32, callback: Callback) -> i32;
}

fn main() {
    let who = ffi::CString::new("world").unwrap();
    unsafe {
        say_hello(who.as_ptr());
    }

    let arr: &mut [u8] = &mut [1, 2, 3, 4];
    unsafe {
        add_one(arr.as_mut_ptr(), arr.len());
    }
    println!("arr = {:?}", arr);

    unsafe {
        let v = new_hello();
        print_hello(v);
        free_hello(v);
        print_hello(v);
    }

    unsafe {
        let v = call_callback_from_rust(123, increment);
        assert_eq!(v, 124, "call_callback_from_rust");
        println!("call_callback_from_rust is ok");
    }
}
