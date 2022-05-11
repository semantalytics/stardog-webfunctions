use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use md5::{Md5, Digest};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();

    let value = values["results"]["bindings"][0]["value_0"]["value"].as_str().unwrap();

    let hash = Md5::digest(value);

    let hex_hash = base16ct::lower::encode_string(&hash);

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": hex_hash}}]}
    }).to_string();

    return unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
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