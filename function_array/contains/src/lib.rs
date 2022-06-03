use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();

    let binding = &values["results"]["bindings"][0];
    let array = binding["value_0"]["value"].as_str().unwrap();
    let value = &binding["value_1"];

    let mapping_dictionary_sqr = json!({"head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0": value}]}}).to_string();

    let sqr_ptr = unsafe { CString::from_vec_unchecked(mapping_dictionary_sqr.into_bytes()) }.into_raw();

    let id = unsafe { mapping_dictionary_add(sqr_ptr as i32) };

    let result = array.trim_matches(|c| c == '[' || c == ']').split(',').map(|t| t.trim().parse::<i64>().unwrap()).any(|i| i == id);

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": result, "datatype": "http://www.w3.org/2001/XMLSchema#boolean"}}]}
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

#[no_mangle]
pub extern fn cardinality_estimate(subject: *mut c_char) -> *mut c_char {
    return stardog_function::cardinality_estimate(subject);
}
