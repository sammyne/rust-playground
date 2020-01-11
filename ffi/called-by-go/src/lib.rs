#[repr(C)]
pub struct HelloWorld {
    pub value: u64,
}

impl HelloWorld {
    fn new(value: u64) -> Self {
        Self { value }
    }
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
pub extern "C" fn hello_world_new(value: u64) -> *mut HelloWorld {
    Box::into_raw(Box::new(HelloWorld::new(value)))
    //Box::into_raw(Box::new(HelloWorld { value }))
}
