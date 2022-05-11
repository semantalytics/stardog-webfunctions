use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use voca_rs::case::capitalize;
use std::str::FromStr;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(subject: *mut c_char) -> *mut c_char {
    let arg = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values: Value = serde_json::from_str(arg).unwrap();
    let binding = &values["results"]["bindings"][0];
    let value_1 = binding["value_0"]["value"].as_str().unwrap();
    let value_2 = binding["value_1"]["value"].as_str().unwrap_or("true");

    let result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": capitalize(value_1, FromStr::from_str(value_2).unwrap())}}]}
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
