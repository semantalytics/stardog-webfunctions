use dockertest::{Composition, DockerTest};
use std::sync::{Arc, Mutex};

#[test]
fn file_test() -> Result<(), ureq::Error>{

    let mut test = DockerTest::new();

    let hello = Composition::with_repository("hello-world");

    test.add_composition(hell);

    let has_ran: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let has_ran_test = has_ran.clone();
    test.run(|ops| async move {
        let _container = ops.handle("hello-world");

        let mut ran = has_ran_test.lock().unwrap();
        *ran = true;
    });

    let ran = has_ran.lock().unwrap();
    assert!(*ran);

    let query = r#"
                PREFIX array: <file:src/main/rust/function_array/target/wasm32-unknown-unknown/release/>

                SELECT
                    ?result
                WHERE {
                    BIND(wf:call(str(array:contains.wasm), wf:call(str(array:of.wasm), "star", "dog"), "dog") AS ?result)
                }
                "#;

    let json: serde_json::Value = ureq::get(query)
        .set("Accept", "application/sparql-results+json")
        .query("query", query)
        .call()?
        .into_json()?;

    assert_eq!(json["results"]["bindings"].as_array().unwrap().len(), 1);
    Ok(())
}

#[test]
fn http_test() {

}

#[test]
fn ipfs_test() {

}
