use std::ffi::{CStr, CString}; 
use std::os::raw::c_char;
use serde_json::{Value, json};
use lingua::{LanguageDetector, LanguageDetectorBuilder, Language, IsoCode639_1};
use std::str::FromStr;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(args: *mut c_char) -> *mut c_char {
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap() };

    //TODO third optional argument for minimum relative distance
    let values: Value = serde_json::from_str(args_str).unwrap();
    let binding = &values["results"]["bindings"][0];
    let target_string = binding["value_0"]["value"].as_str().unwrap();
    let candidate_langs = binding["value_1"]["value"].as_str().unwrap();

    let langs : Vec<Language> = candidate_langs.trim_matches(|c| c == '[' || c == ']').split(',').map(|c| c.trim().parse::<i64>().unwrap()).map(|i| {
        let sqr = unsafe { CStr::from_ptr(mapping_dictionary_get(i)).to_str().unwrap() };
        let values: Value = serde_json::from_str(sqr).unwrap();
        let lang = values["results"]["bindings"][0]["value_0"]["value"].as_str().unwrap();
        return Language::from_iso_code_639_1(&IsoCode639_1::from_str(lang).unwrap());
    }).collect();

    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&langs).with_preloaded_language_models().build();
    let detected_language = detector.detect_language_of(target_string).unwrap();

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": detected_language.iso_code_639_1().to_string()}}]}
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


