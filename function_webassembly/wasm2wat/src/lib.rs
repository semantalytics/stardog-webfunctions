use serde_json::{Value, json};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

pub use stardog_function::*;

#[no_mangle]
pub extern "C" fn evaluate(subject: *mut c_char) -> *mut c_char {
    let arg = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values_json: Value = serde_json::from_str(arg).unwrap();
    let wasm = values_json["results"]["bindings"][0]["value_1"]["value"].as_str().unwrap();

    let wat = wasmprinter::print_bytes(base64::decode(wasm).unwrap()).unwrap();

    let sparql_query_result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": wat}}]}
    }).to_string();

    return unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
}
