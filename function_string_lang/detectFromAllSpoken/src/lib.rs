use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use lingua::{LanguageDetector, LanguageDetectorBuilder};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(args: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap() };

    let values: Value = serde_json::from_str(args_str).unwrap();
    let value_1 = values["results"]["bindings"][0]["value_1"]["value"].as_str().unwrap();

    let detector: LanguageDetector = LanguageDetectorBuilder::from_all_spoken_languages().with_preloaded_language_models().build();
    let detected_language = detector.detect_language_of(value_1).unwrap();

    let sparql_query_result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": value_1, "xml:lang": detected_language.iso_code_639_1().to_string()}}]}
    }).to_string();

    return unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();

}

