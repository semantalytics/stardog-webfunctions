use dockertest::{Composition, DockerTest};
use std::sync::{Arc, Mutex};
use std::env;
/*
fn file_test() -> Result<(), ureq::Error>{

    let mut test = DockerTest::new();

    //TODO need some way to set the stardog version to use
    // then match up the plugin to that version
    // download plugin and make available to docker
    let stardog = Composition::with_repository("stardog/stardog")
        .tag("latest")
        .bind_mount(env::var("STARDOG_LICENSE_PATH"), "/var/opt/stardog/")
        .bind_mount("$PLUGIN", "/var/opt/stardog/server/ext/$PLUGIN") //webfunciton plugin
        .with_wait_for(Box::new(MessageWait {
            message: "".to_string(),
            source: MessageSource::Stdout, //can we make this the livlyness rest endpitn??
            timeout: 10, //seconds???
        }))
        .with_container_name("stardog");

    let ipfs = Composition::with_repository("ipfs/go-ipfs")
        .tag("latest")
        .with_container_name("ipfs");

    test.add_composition(stardog);
    test.add_composition(ipfs);

    let has_ran: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let has_ran_test = has_ran.clone();
    test.run(|ops| async move {
        let _stardog_container = ops.handle("stardog/stardog");
        let _ipfs_container = ops.handle("ipfs/go-ipfs");
        //TODO need to change up ports??
        // docker run -d --name ipfs_host -v $ipfs_staging:/export -v $ipfs_data:/data/ipfs -p 4001:4001 -p 127.0.0.1:8080:8080 -p 127.0.0.1:5001:5001 ipfs/go-ipfs:latest


        //TODO need to add plugin (what version) to stardog and license
        //  docker run -it -v ~/stardog-home/:/var/opt/stardog -p 5820:5820 -e STARDOG_EXT=/var/opt/stardog-ext -v ~/stardog-ext:/var/opt/stardog-ext stardog/stardog
        // use single file voluem and env var -e STARDOG_LICENSE_PATH

        //needs to have been built and deployed to test ipfs node
        //Might need to have test peerid so that location is consistent or need to get node's default ipns id

        //Need to wait for Stardog and IPFS to come up with MessageWait
        //can also check on stardog with http aliveness check

        ureq::post(format!("http://{}:{}/", _stardog_container.ip(), 5820))
            .send_json(json!({"dbname": "test"}));

        //TODO
        let query = r#"
                PREFIX array: <file:src/main/rust/function_array/target/wasm32-unknown-unknown/release/>

                SELECT
                    ?result
                WHERE {
                    BIND(wf:call(str(array:contains.wasm), wf:call(str(array:of.wasm), "star", "dog"), "dog") AS ?result)
                }
                "#;

        let json: serde_json::Value = ureq::get(format!("http://{}:{}/", _stardog_container.ip(), 5820))
            .set("Accept", "application/sparql-results+json")
            .query("query", query)
            .call()?
            .into_json()?;

        assert_eq!(json["results"]["bindings"].as_array().unwrap().len(), 1);

        let mut ran = has_ran_test.lock().unwrap();
        //Test body
        *ran = true;

    });

    let ran = has_ran.lock().unwrap();
    assert!(*ran);
    Ok(());
}
*/