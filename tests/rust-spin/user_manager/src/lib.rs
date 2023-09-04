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
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
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

fn insert_record(person: &Person) -> String {
    let body = serde_json::to_string(&person).unwrap();
    let body_bytes = body.as_bytes();
    let body_bytes = Bytes::copy_from_slice(&body_bytes);

    let res = spin_sdk::http::send(
        http::Request::builder()
            .method("POST")
            .uri("http://10.223.6.99:3000/prest/public/users?_page_size=10&_page=1")
            .header("Content-Type", "application/json")
            .body(Some(body_bytes))
            .unwrap(),
    )
    .unwrap();

    return res.status().as_str().to_string();
}

/// A simple Spin HTTP component.
#[http_component]
fn user_manager(req: Request) -> Result<Response> {
    match parse_query_string(&req, "entries") {
        Ok(e) => {
            let iterations = e.parse::<usize>();
            if iterations.is_err() {
                return Ok(http::Response::builder()
                    .status(500)
                    .body(Some(format!("entries is not a number...").into()))?);
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

            return Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Iterated {} times", iterations).into()))?);
        }
        Err(e) => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some(format!("{}", e).into()))?);
        }
    };
}
