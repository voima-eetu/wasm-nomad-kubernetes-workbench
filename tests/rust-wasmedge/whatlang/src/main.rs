use anyhow::Result;

use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, http::Error as HyperError};
use tokio::net::TcpListener;

use whatlang::detect;
use {std::collections::HashMap, url::Url};

fn parse_query_string(req: Request<Body>, parameter_name: &str) -> Result<String, String> {
    let host = req
        .headers()
        .get("host")
        .unwrap().to_str().unwrap();
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

async fn whatlang(req: Request<Body>) -> Result<Response<Body>, HyperError> {
    match parse_query_string(req, "text") {
        Ok(v) => {
            let language = detect(&v).unwrap().lang();
            return Ok(Response::builder()
                .status(200)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(format!("<i>{v}</i> = <b>{language}</b>").into())?);
        }
        Err(e) => {
            return Ok(Response::builder()
                .status(500)
                .body(format!("{e}").into())?);
        }
    }
}

//Simple WasmEdge hyper_wasi HTTP server
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

  let listener = TcpListener::bind(&addr).await?;
  loop {
    let (stream, _) = listener.accept().await?;
    
    tokio::task::spawn(async move {
        let http = Http::new();
        if let Err(err) = http.serve_connection(stream, service_fn(whatlang)).await {
            println!("Error serving connection: {:?}", err);
        }
    });
  }
}
