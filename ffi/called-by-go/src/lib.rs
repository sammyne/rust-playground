use std::ffi::{CStr, CString};
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct HelloWorld {
    who: String,
}

impl HelloWorld {
    fn new(who: String) -> Self {
        Self { who }
    }
}

#[no_mangle]
pub extern "C" fn hi_new() -> Hi {
    Hi { x: 123, y: 456 }
}

#[no_mangle]
pub extern "C" fn cstring_free(c_str_ptr: *mut c_char) {
    if c_str_ptr.is_null() {
        return;
    }

    unsafe {
        CString::from_raw(c_str_ptr);
    }
}

#[no_mangle]
pub extern "C" fn cstring_new() -> *mut c_char {
    let s = CString::new("Hello World").unwrap();

    s.into_raw()
}

#[no_mangle]
pub extern "C" fn hello_world(c_who: *const c_char) {
    let who = unsafe { CStr::from_ptr(c_who).to_str() };

    println!("hello {}, welcome to rust's world", who.unwrap());
}

#[no_mangle]
pub extern "C" fn hello_world_free(ptr: *mut HelloWorld) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn hello_world_new(c_who: *const c_char) -> *mut HelloWorld {
    let who = unsafe { CStr::from_ptr(c_who).to_str().unwrap() };

    Box::into_raw(Box::new(HelloWorld::new(who.to_string())))
}

#[no_mangle]
pub extern "C" fn hello_world_say(ptr: *const HelloWorld) {
    let h = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    println!("hello world, {}", &h.who);
}
