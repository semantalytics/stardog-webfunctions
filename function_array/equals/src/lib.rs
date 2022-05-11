use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();

    let binding = &values["results"]["bindings"][0];
    let array = &binding["value_1"]["value"].as_str().unwrap();

    let mut array_literal_ids: Vec<i64> = array.trim_matches(|c| c == '[' || c == ']').split(',').map(|i| i.trim().parse::<i64>().unwrap()).dedupe().collect();

    let dedupe_array_literal_ids: Vec<String> = array_literal_ids.iter().map(ToString::to_string).collect();

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": format!("[{}]", dedupe_array_literal_ids.join(", ")), "datatype": "tag:stardog:api:array"}}]}
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