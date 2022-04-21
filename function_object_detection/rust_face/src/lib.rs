use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use rustface::{Detector, FaceInfo, ImageData};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();
    let binding = &values["results"]["bindings"][0];
    let input_image = decode(binding["value_1"]["value"].as_str().unwrap()).unwrap();

    let model = include_bytes!("seeta_fc_frontal_v1.0.bin");
    let mut detector = rustface::create_detector().unwrap();

    let mut image = ImageData::new(input_image, width, height);
    detection_ids = detector.dectect(&mut image).into_iter().map(|d| {

        let sparql_query_result = json!({"head": {"vars":["result"]}, "results":{"bindings":[{"result": value}]}}).to_string();

        let sqr_ptr = unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
        let id = unsafe { mappingDictionaryAdd(sqr_ptr as i32) };
        return id.to_string();
        //TODO this might be better of as a service

    }).collect::<Vec<String>>().join(", ");

    let sparql_query_result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": format!("[{}]", detection_ids), "datatype": "tag:stardog:api:array"}}]}
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
