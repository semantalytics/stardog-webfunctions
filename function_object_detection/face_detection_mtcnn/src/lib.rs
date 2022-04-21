use serde_json::json;
use std::env;
use std::error::Error;
use std::io::{self, Read};
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

pub use stardog_function::*;

#[no_mangle]
pub extern fn evaluate(subject: *mut c_char) -> *mut c_char {
    let arg = unsafe { CStr::from_ptr(subject).to_str().unwrap() };

    let values_json: Value = serde_json::from_str(arg).unwrap();
    let input_image = decode(values_json["results"]["bindings"][0]["value_1"]["value"].as_str().unwrap()).unwrap().parse::<f32>().unwrap();
    let detection_threshold = decode(values_json["results"]["bindings"][0]["value_2"]["value"].as_str().unwrap()).unwrap();
    let model = include_bytes!("mtcnn.pb");
    let image = image::load_from_memory(input_image).unwrap().to_rgb8();
    let img_width = image.width();
    let img_height = image.height();

    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?;

    // The `input` tensor expects BGR pixel data.
    let input = Tensor::new(&[img_height, img_width, 3]).with_values(&flattened)?;
    let min_size = Tensor::new(&[]).with_values(&[detection_threshold])?;

    // Use input params from the existing module.
    let thresholds = Tensor::new(&[3]).with_values(&[0.6f32, 0.7f32, 0.7f32])?;
    let factor = Tensor::new(&[]).with_values(&[0.709f32])?;
    let mut args = SessionRunArgs::new();

    // Load default parameters and input image.
    args.add_feed(&graph.operation_by_name_required("min_size")?, 0, &min_size);
    args.add_feed(
        &graph.operation_by_name_required("thresholds")?,
        0,
        &thresholds,
    );
    args.add_feed(&graph.operation_by_name_required("factor")?, 0, &factor);
    args.add_feed(&graph.operation_by_name_required("input")?, 0, &input);

    // Request the following outputs after the session runs.
    let bbox = args.request_fetch(&graph.operation_by_name_required("box")?, 0);
    let session = Session::new(&SessionOptions::new(), &graph)?;
    session.run(&mut args)?;

    // Our bounding box extents.
    let bbox_res: Tensor<f32> = args.fetch(bbox)?;

    // Print results.
    let mut iter = 0;
    let mut json_vec: Vec<[f32; 4]> = Vec::new();
    while (iter * 4) < bbox_res.len() {
        json_vec.push([
            bbox_res[4 * iter + 1], // x1
            bbox_res[4 * iter],     // y1
            bbox_res[4 * iter + 3], // x2
            bbox_res[4 * iter + 2], // y2
        ]);
        iter += 1;
    }
    let json_obj = json!(json_vec);
    println!("{}", json_obj.to_string());
    Ok(())

    let sparql_query_result = json!({
      "head": {"vars":["result"]}, "results":{"bindings":[{"result":{"type":"literal","value": prediction}}]}
    }).to_string();

    return unsafe { CString::from_vec_unchecked(sparql_query_result.into_bytes()) }.into_raw();
}