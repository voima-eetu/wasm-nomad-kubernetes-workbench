use anyhow::Result;
use hyper::{Client, Uri, Request, Response, Body, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::server::Server;
use simsearch::SimSearch;
use std::collections::HashMap;
use std::str;
use url::Url;
use tokio;

fn parse_query_string(req: &Request<Body>, parameter_name: &str) -> Result<String, String> {
    let host = req.headers()
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

async fn fuzzysearch_http(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    let search_keyword = parse_query_string(&req, "search");
    if search_keyword.is_err() {
        return Ok(Response::builder()
            .status(500)
            .body(Body::from(format!("search argument is missing..."))).unwrap());
    }
    let search_keyword = search_keyword.unwrap();

    let client = Client::new();
    let response = client.get(Uri::from_static("http://10.223.6.99:8000/hamlet.txt")).await?;
    
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = str::from_utf8(&body_bytes).unwrap();

    let matches = search(body_str, search_keyword);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("{:?}", matches)))
        .unwrap())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT").unwrap_or("3000".to_string()).parse().unwrap();
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, hyper::Error>(service_fn(fuzzysearch_http))
        }
    });

    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    server.await?;

    Ok(())
}

