use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use simsearch::SimSearch;
use std::str;
use {std::collections::HashMap, url::Url};

fn parse_query_string(req: &Request, parameter_name: &str) -> Result<String, String> {
    let binding = req.headers();
    let host = binding
        .get("host")
        .unwrap()
        .to_str()
        .unwrap();
    let uri = req.uri().to_string();
    let full_url = format!("http://{}{}", host, uri);
    let parsed_url = Url::parse(&full_url).or_else(|_e| {
        return Err("cannot parse the url...");
    });
    let hash_query: HashMap<_, _> = parsed_url.unwrap().query_pairs().into_owned().collect();
    let val = hash_query.get(parameter_name);
    if val.is_none() {
        return Err(
            format!("{parameter_name} parameter in the query string is missing...").to_string(),
        );
    }
    return Ok(val.unwrap().to_string());
}

fn search(file_content: &str, search_pattern: String) -> Vec<u32> {
    let mut counter = 0;
    let mut engine: SimSearch<u32> = SimSearch::new();

    for line in file_content.split("\n") {
        engine.insert(counter, &line);
        counter += 1;
    }

    let mut results: Vec<u32> = engine.search(&search_pattern);
    results.sort();
    return results;
}

/// A simple Spin HTTP component.
#[http_component]
fn fuzzysearch_http(req: Request) -> Result<Response> {

    let search_keyword = parse_query_string(&req, "search");
    if search_keyword.is_err() {
        return Ok(http::Response::builder()
            .status(500)
            .body(Some(format!("search argument is missing...").into()))?);
    }
    let search_keyword = search_keyword.unwrap();

    let response = spin_sdk::http::send(
        http::Request::builder()
            .method("GET")
            .uri("http://10.223.6.99:8000/hamlet.txt")
            .body(None)?,
    )?;

    let body = response.into_body();
    let body_bytes = body.unwrap();
    let body_str = str::from_utf8(&body_bytes).unwrap();

    let matches = search(body_str, search_keyword);

    Ok(http::Response::builder()
        .status(200)
        .body(Some(format!("{:?}", matches).into()))?)
}
