use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use lingua::Language;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(_args: *mut c_char) -> *mut c_char {

    let array_ids: Vec<String> = Language::all_with_devanagari_script().iter().map(|lang| {
        let sparql_query_result = json!({"head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0": {"type":"literal", "value": lang.iso_code_639_1().to_string()}}]}}).to_string();

        let sqr_ptr = unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
        let id = unsafe { mapping_dictionary_add(sqr_ptr as i32) };
        return id.to_string();
    }).collect();

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": format!("[{}]", array_ids.join(", ")), "datatype": "tag:stardog:api:array"}}]}
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
    let subject = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values: Value = serde_json::from_str(subject).unwrap();
    let estimate = values["results"]["bindings"][0]["value_0"]["value"].as_str().unwrap();

    let result = json!({
      "head": {"vars":["value_0", "value_1"]}, "results":{"bindings":[
            {"value_0":{"type":"literal","value": estimate}, "value_1":{"type":"literal","value": "ACCURATE"}}
        ]}
    }).to_string().into_bytes();

    unsafe { CString::from_vec_unchecked(result) }.into_raw()
}


