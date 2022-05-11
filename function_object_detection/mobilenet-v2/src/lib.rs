use tract_tensorflow::prelude::*;

use std::io::BufRead;
use stringreader::StringReader;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json::{Value, json};
use base64::decode;

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(arg: *mut c_char) -> *mut c_char {
    let arg_str = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

    let values: Value = serde_json::from_str(arg_str).unwrap();
    let binding = &values["results"]["bindings"][0];
    let input_image = decode(binding["value_0"]["value"].as_str().unwrap()).unwrap();


    let prediction =  match get_prediction(&input_model, &input_image) {
      Ok(p) => p,
      Err(e) => e.to_string(),
    };

    let sparql_query_result = json!({
      "head": {"vars":["value_0"]}, "results":{"bindings":[{"value_0":{"type":"literal","value": prediction}}]}
    }).to_string();

    return unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
}

fn get_prediction(model_bytes: &[u8], image_bytes: &[u8]) -> Result<String, anyhow::Error> {

    let results = infer(model_bytes, image_bytes);
	return get_label(results as usize);
}



/// Perform the inference given the contents of the model and the image, and
/// return the index of the predicted class.
///
/// Adapted from https://github.com/sonos/tract/tree/main/examples/tensorflow-mobilenet-v2 and
/// using the TensorFlow Mobilenet V2 model.
/// See https://github.com/tensorflow/models/tree/master/research/slim/nets/mobilenet
fn infer(model_bytes: &[u8], image_bytes: &[u8]) -> i32 {
    let mut model = std::io::Cursor::new(model_bytes);
    let model = tract_tensorflow::tensorflow()
        .model_for_read(&mut model)
        .unwrap()
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
        )
        .unwrap()
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap();

    let image = image::load_from_memory(image_bytes).unwrap().to_rgb8();
    // The model was trained on 224 x 224 RGB images, so we are resizing the input image to this dimension.
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();

    let result = model.run(tvec!(image)).unwrap();
    let best = result[0]
        .to_array_view::<f32>()
        .unwrap()
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    return best.unwrap().1;
}

/// Get the human-readable label of a prediction
/// from the MobileNet V2 labels file
fn get_label(num: usize) -> Result<String, anyhow::Error> {
    // The result of executing the inference is the predicted class,
    // which also indicates the line number in the (1-indexed) labels file.

    let labels = include_str!("labels.txt");

    let l = StringReader::new(labels);
    let content = std::io::BufReader::new(l);
    return content
        .lines()
        .nth(num - 1)
        .expect("cannot get prediction label")
        .map_err(|err| anyhow::Error::new(err))
}
