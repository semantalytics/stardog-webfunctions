use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use voca_rs::case::camel_case;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(subject: *mut c_char) -> *mut c_char {
    let arg = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values_json: Value = serde_json::from_str(arg).unwrap();
    let value_0 = values_json["results"]["bindings"][0]["value_0"]["value"].as_str().unwrap();

    let result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": camel_case(value_0)}}]}
    }).to_string().into_bytes();

    unsafe { CString::from_vec_unchecked(result) }.into_raw()
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
