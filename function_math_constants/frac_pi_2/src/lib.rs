use serde_json::json;
use std::f64::consts;
use std::ffi::CString;
use std::os::raw::c_char;

pub use stardog_function::*;

#[no_mangle]
pub extern "C" fn evaluate(_subject: *mut c_char) -> *mut c_char {
    let result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": consts::FRAC_PI_2}}]}
    }).to_string().into_bytes();

    return unsafe { CString::from_vec_unchecked(result) }.into_raw();
}
