use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json::{Value, json};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(subject: *mut c_char) -> *mut c_char {
    let subject = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values: Value = serde_json::from_str(subject).unwrap();
    let value_1 = values["results"]["bindings"][0]["value_1"]["value"].as_str().unwrap();

    let result = value_1.to_uppercase();

    let result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": result}}]}
    }).to_string().into_bytes();

    unsafe { CString::from_vec_unchecked(result) }.into_raw()

}
