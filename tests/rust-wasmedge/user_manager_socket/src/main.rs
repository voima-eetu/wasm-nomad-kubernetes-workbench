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

use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

use reqwest::Client;
use http::HeaderMap;

//use http_req::{request::{Request as ReqRequest, Method}, uri::Uri};
//use std::convert::TryFrom;

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

fn parse_query_string(req: &Request<String>, parameter_name: &str) -> Result<String, String> {
    let binding = req.header();
    let host = binding
        .get_field("host")
        .unwrap();
    let uri = req.request_target().as_str();
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

async fn insert_record(person: &Person) -> String {
    let body = serde_json::to_string(&person).unwrap();
    let body_bytes = body.as_bytes();
    let body_bytes = Bytes::copy_from_slice(&body_bytes);
    
/*    
      let mut writer = Vec::new();
      let uri = Uri::try_from("http://192.168.85.153:3000/prest/public/users?_page_size=10&_page=1").unwrap();
      let res = ReqRequest::new(&uri)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(&body_bytes.to_vec())
            .send(&mut writer)
            .unwrap();
      return res.status_code().to_string();
*/      
      
      let mut headers = HeaderMap::new();
      headers.insert(http::header::CONTENT_TYPE, "application/json".parse().unwrap());

      let client = Client::new();
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

fn user_manager(req: Request<String>) -> bytecodec::Result<Response<String>> {
    match parse_query_string(&req, "entries") {
        Ok(e) => {
            let iterations = e.parse::<usize>();
            if iterations.is_err() {
                return Ok(Response::new(
                      HttpVersion::V1_0,
                      StatusCode::new(500)?,
                      ReasonPhrase::new("")?,
                      format!("entries is not a number...").into(),
                ))
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
                insert_record(&person);
            }

            return Ok(Response::new(
                      HttpVersion::V1_0,
                      StatusCode::new(200)?,
                      ReasonPhrase::new("")?,
                      format!("Iterated {} times", iterations).into()
                      ))
        }
        Err(e) => {
            return Ok(Response::new(
                      HttpVersion::V1_0,
                      StatusCode::new(500)?,
                      ReasonPhrase::new("")?,
                      format!("{}", e).into(),
            ))
        }
    };
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => user_manager(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}
