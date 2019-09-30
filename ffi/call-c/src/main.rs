use std::ffi;
use std::os::raw;

extern {
    fn add_one(arr: *mut u8, arr_len: usize);
    fn say_hello(who: *const raw::c_char);
}

fn main() {
    let who = ffi::CString::new("world").unwrap();
    unsafe {
        say_hello(who.as_ptr());
    }

    let arr: &mut [u8] = &mut [1,2,3,4];
    unsafe{
        add_one(arr.as_mut_ptr(), arr.len());
    }
    println!("arr = {:?}",arr);
}
