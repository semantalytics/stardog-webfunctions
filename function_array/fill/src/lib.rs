use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();

    let binding = &values["results"]["bindings"][0];
    let value = &binding["value_0"]["value"].as_str().unwrap();
    let size = &binding["value_1"].as_str().unwrap().parse::<i32>().unwrap();

    let mapping_dictionary_add_value = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0": value}]}
    }).to_string().into_bytes();

    let mapping_dictionary_add_ptr = unsafe { CString::from_vec_unchecked(mapping_dictionary_add_value) }.into_raw();

    let id = unsafe { mappingDictionaryAdd(mapping_dictionary_add_ptr as i32) };

    let filled_vec = iter::repeat(id).take(size).collect();

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": format!("[{}]", filled_vec.join(", ")), "datatype": "tag:stardog:api:array"}}]}
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