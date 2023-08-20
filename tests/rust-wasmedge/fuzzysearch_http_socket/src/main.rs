use anyhow::Result;

use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

use simsearch::SimSearch;
use std::str;
use {std::collections::HashMap, url::Url};

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


fn fuzzysearch_http(req: Request<String>) -> bytecodec::Result<Response<String>> {

    let search_keyword = parse_query_string(&req, "search");
    if search_keyword.is_err() {
        return Ok(Response::new(
                  HttpVersion::V1_0,
                  StatusCode::new(500)?,
                  ReasonPhrase::new("")?,
                  format!("search argument is missing...").into()
        ))
    }
    let search_keyword = search_keyword.unwrap();

    // Establish a connection
    let mut stream = TcpStream::connect("www.buildingjavaprograms.com:80").unwrap();

    // Send an HTTP GET request
    let request = format!(
        "GET /code_files/3ed/ch06/hamlet.txt HTTP/1.1\r\n\
         Host: www.buildingjavaprograms.com\r\n\
         Connection: close\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).unwrap();

    // Read the response
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();

    // Extract the body from the response (for simplicity, this assumes headers and body are separated by two CRLFs)
    let body_start = String::from_utf8_lossy(&response).find("\r\n\r\n").unwrap() + 4;
    let body = &response[body_start..];
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    // Assuming you have a function `search` that does the searching
    let matches = search(&body_str, search_keyword);

    Ok(Response::new(
      HttpVersion::V1_0,
      StatusCode::new(200)?,
      ReasonPhrase::new("")?,
      format!("{:?}", matches).into()
    ))
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
        Ok(req) => fuzzysearch_http(req),
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
