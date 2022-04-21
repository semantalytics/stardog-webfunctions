
#[cfg(test)]
mod tests {

    use ureq::Error;
    use std::env;

    #[test]
    fn test_append_over_http() {
        let query = r#"
            prefix array: <http://wf.semantalytics.com/ipns/k51qzi5uqu5dlx0ttqevj64d3twk31y7hsgnofkqkjaiv11k98lj2rx60kjgv5/stardog/function/array/>
            prefix wf: <http://semantalytics.com/2021/03/ns/stardog/webfunction/latest/>

            SELECT
                ?result
            WHERE {
                UNNEST(wf:call(array:of, "star", "dog") AS ?result)
            }
            "#;

        let url = format!("http://{}:{}/{}/query", env::var("STARDOG_HOST").unwrap().as_str(),
                                                   env::var("STARDOG_PORT").unwrap().as_str(),
                                                   env::var("STARDOG_DB").unwrap().as_str());

        let response = ureq::get(&url)
            .set("Accept", "application/sparql-results+json")
            .set("Authorization", &get_basic_auth_header("admin", "admin"))
            .query("query", query)
            .call();

        match response {
            Ok(r) => {
                let json: serde_json::Value = r.into_json().unwrap();

                let bindings = &json["results"]["bindings"];
                assert_eq!(bindings.as_array().unwrap().len(), 2);
                let binding = bindings[0];
                assert_eq!(binding["value_0"]["value"], "star");
                assert_eq!(binding["value_1"]["value"], "dog");
            },
            Err(Error::Status(code, response)) => {
                println!("Code: {} Response: {}", code, response.into_string().unwrap());
                assert!(false);
            }
            Err(_) => {}
        }
    }

    fn get_basic_auth_header( user: &str, pass: &str ) -> String {
        let usrpw = String::from( user ) + ":" + pass;
        String::from( "Basic " ) + &base64::encode( usrpw.as_bytes())
    }
}