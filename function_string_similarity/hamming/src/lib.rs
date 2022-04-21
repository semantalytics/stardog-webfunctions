use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json::{Value, json};
use strsim::hamming;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(subject: *mut c_char) -> *mut c_char {
    let subject = unsafe { CStr::from_ptr(subject).to_str().unwrap() };
    
    let values: Value = serde_json::from_str(subject).unwrap();

    let binding = &values["results"]["bindings"][0];

    let string1 = binding["value_1"]["value"].as_str().unwrap();
    let string2 = binding["value_2"]["value"].as_str().unwrap();

    let result = hamming(string1, string2);

    let sparql_query_result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": result.ok()}}]}
    }).to_string();

    return unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
}

#[no_mangle]
pub extern fn doc() -> *mut c_char {

    let output = b"
Compute the Levenshtein distance between two strings.

arguemnts:
    value[0]:literal first string to compare
    value[1]:literal second string to compare

	".to_vec();

    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}