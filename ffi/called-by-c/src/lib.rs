use std::ffi::CStr;
use std::os::raw::c_char;

pub struct HelloWorld {
    who: String,
}

impl HelloWorld {
    fn new(who: String) -> Self {
        Self { who }
    }
}

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
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