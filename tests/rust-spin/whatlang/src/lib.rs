use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use whatlang::detect;
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

#[http_component]
fn whatlang(req: Request) -> Result<Response> {
    match parse_query_string(&req, "text") {
        Ok(v) => {
            let language = detect(&v).unwrap().lang();
            return Ok(http::Response::builder()
                .status(200)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(Some(format!("<i>{v}</i> = <b>{language}</b>").into()))?);
        }
        Err(e) => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some(format!("{e}").into()))?);
        }
    }
}
