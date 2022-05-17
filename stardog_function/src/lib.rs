use std::mem;
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern fn malloc(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern fn free(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

extern {
    pub fn mapping_dictionary_add(buf_addr: i32) -> i64;
}

extern {
    pub fn mapping_dictionary_get(buf_addr: i64) -> *mut c_char;
}
