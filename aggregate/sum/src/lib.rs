use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};

pub use stardog_function::*;

static mut STATE: i64 = 0;

#[no_mangle]
pub extern fn get_value() -> *mut c_char {
    let result = unsafe { json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": STATE }}]}
    }).to_string().into_bytes() };

    unsafe { CString::from_vec_unchecked(result) }.into_raw()
}

#[no_mangle]
pub extern fn aggregate(subject: *mut c_char) -> *mut c_char {
    let arg = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values_json: Value = serde_json::from_str(arg).unwrap();
    let value_1 = values_json["results"]["bindings"][0]["value_1"]["value"].as_str().unwrap();
    let multiplicity = values_json["results"]["bindings"][0]["multiplicity"]["value"].as_str().unwrap();

    unsafe { STATE += value_1.parse::<i64>().unwrap() * multiplicity.parse::<i64>().unwrap() };

    let result = unsafe { json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": STATE}}]}
    }).to_string().into_bytes() };

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
