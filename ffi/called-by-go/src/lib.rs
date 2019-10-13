

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hello_world(c_who: *const c_char) {
    let who = unsafe { CStr::from_ptr(c_who).to_str() };

    println!("hello {}, welcome to rust's world", who.unwrap());
}