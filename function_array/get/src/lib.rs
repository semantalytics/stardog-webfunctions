use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::Value;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();

    let binding = &values["results"]["bindings"][0];
    let array = binding["value_0"]["value"].as_str().unwrap();
    let index = binding["value_1"]["value"].as_str().unwrap().parse::<usize>().unwrap();

    let index_value_id = array.trim_matches(|c| c == '[' || c == ']').split(',').nth(index).unwrap().trim().parse::<i64>().unwrap();

    return unsafe { mappingDictionaryGet(index_value_id) };
}

#[no_mangle]
pub extern fn doc() -> *mut c_char {

    let output = b"
function description here

arguemnts:
    value_0:literal first string to compare
    value_1:literal second string to compare

	".to_vec();

    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}