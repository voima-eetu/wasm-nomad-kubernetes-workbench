use anyhow::Result;
use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, http::Error as HyperError};
use tokio::net::TcpListener;
use {std::collections::HashMap, url::Url};

fn prime_numbers_calc(max: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if max >= 2 {
        result.push(2)
    }
    for i in (3..max + 1).step_by(2) {
        let stop: usize = (i as f64).sqrt() as usize + 1;
        let mut status: bool = true;

        for j in (3..stop).step_by(2) {
            if i % j == 0 {
                status = false;
                break;
            }
        }
        if status {
            result.push(i)
        }
    }
    result
}

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

async fn prime_numbers(req: Request<Body>) -> Result<Response<Body>, HyperError> {
    match parse_query_string(req, "n") {
        Ok(e) => {
            let val = e.parse::<usize>();
            if val.is_err() {
                return Ok(http::Response::builder()
                    .status(500)
                    .body(format!("n is not a number...").into())?);
            }
            let prime_numbers_result = prime_numbers_calc(val.unwrap());
            return Ok(http::Response::builder()
                .status(200)
                .body(
                    format!("The value is: {:?}", prime_numbers_result).into(),
                )?);
        }
        Err(e) => {
            return Ok(http::Response::builder()
                .status(500)
                .body(format!("{}", e).into())?);
        }
    };
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
        if let Err(err) = http.serve_connection(stream, service_fn(prime_numbers)).await {
            println!("Error serving connection: {:?}", err);
        }
    });
  }
}
