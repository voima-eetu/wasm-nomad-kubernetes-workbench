use anyhow::Result;
use bytes::Bytes;
use fake::faker::{
    address::en::{BuildingNumber, CityName, CountryName, StreetName, ZipCode},
    internet::en::{UserAgent, Username, IP},
    name::raw::*,
};
use fake::locales::*;
use fake::Fake;
use serde::{Deserialize, Serialize};

use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, http::Error as HyperError};
use tokio::net::TcpListener;
use http::HeaderMap;

use {std::collections::HashMap, url::Url};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    username: String,
    ip_address: String,
    user_agent: String,
    country: String,
    city: String,
    street_name: String,
    zip_code: String,
    building_number: String,
}

fn parse_query_string(req: &Request<Body>, parameter_name: &str) -> Result<String, String> {
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

async fn insert_record(person: &Person) -> String{
    let body = serde_json::to_string(&person).unwrap();
    let body_bytes = body.as_bytes();
    let body_bytes = Bytes::copy_from_slice(&body_bytes);
    
      let mut headers = HeaderMap::new();
      headers.insert(http::header::CONTENT_TYPE, "application/json".parse().unwrap());

      let client = reqwest::Client::new();
      match client
          .request(http::Method::POST, "http://192.168.85.153:3000/prest/public/users?_page_size=10&_page=1")
          .headers(headers)
          .body(body_bytes.to_vec())
          .send()
          .await {
            Ok(res) => {
              if res.status().is_success() {
                res.status().as_str().to_string()
              } else {
                format!("Failed with status: {}", res.status().as_str())
              }
            },
            Err(err) => {
              format!("Error during request: {:?}", err)
            }
          }
}

async fn user_manager(req: Request<Body>) -> Result<Response<Body>, HyperError> {
    match parse_query_string(&req, "entries") {
        Ok(e) => {
            let iterations = e.parse::<usize>();
            if iterations.is_err() {
                return Ok(http::Response::builder()
                    .status(500)
                    .body(format!("entries is not a number...").into())?);
            }

            let iterations = iterations.unwrap();

            for _ in 0..iterations {
                let name = Name(EN).fake();
                let username = Username().fake();
                let ip_address = IP().fake();
                let user_agent = UserAgent().fake();
                let country = CountryName().fake();
                let city = CityName().fake();
                let street_name = StreetName().fake();
                let zip_code = ZipCode().fake();
                let building_number = BuildingNumber().fake();
                let person = Person {
                    name,
                    username,
                    ip_address,
                    user_agent,
                    country,
                    city,
                    street_name,
                    zip_code,
                    building_number,
                };
                insert_record(&person).await;
            }

            return Ok(http::Response::builder()
                .status(200)
                .body(format!("Iterated {} times", iterations).into())?);
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
  let port: u16 = std::env::var("PORT").unwrap_or("3000".to_string()).parse().unwrap();
  let addr = SocketAddr::from(([0, 0, 0, 0], port));

  let listener = TcpListener::bind(&addr).await?;
  println!("Listening on http://{}", addr);
  loop {
    let (stream, _) = listener.accept().await?;
    
    tokio::task::spawn(async move {
        let http = Http::new();
        if let Err(err) = http.serve_connection(stream, service_fn(user_manager)).await {
            println!("Error serving connection: {:?}", err);
        }
    });
  }
}
