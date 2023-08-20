use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
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

#[http_component]
fn prime_numbers(req: Request) -> Result<Response> {
    match parse_query_string(&req, "n") {
        Ok(e) => {
            let val = e.parse::<usize>();
            if val.is_err() {
                return Ok(http::Response::builder()
                    .status(500)
                    .body(Some(format!("n is not a number...").into()))?);
            }
            let prime_numbers_result = prime_numbers_calc(val.unwrap());
            return Ok(http::Response::builder()
                .status(200)
                .body(Some(
                    format!("The value is: {:?}", prime_numbers_result).into(),
                ))?);
        }
        Err(e) => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some(format!("{}", e).into()))?);
        }
    };
}
