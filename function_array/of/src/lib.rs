use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();
    let array_ids = values["results"]["bindings"][0].as_object().unwrap().into_iter().map(|(_, value)| {
        let sparql_query_result = json!({"head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0": value}]}}).to_string();

        let sqr_ptr = unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
        let id = unsafe { mapping_dictionary_add(sqr_ptr as i32) };
        return id.to_string();
    }).collect::<Vec<String>>().join(", ");

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": format!("[{}]", array_ids), "datatype": "tag:stardog:api:array"}}]}
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
