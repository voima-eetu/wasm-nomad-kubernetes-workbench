use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use {std::collections::HashMap, url::Url};
use hound;
use std::f32::consts::PI;
use std::i16;

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

fn make_wave(path: String) -> String {
    if path.len() > 2 {
    
        let full_path = format!("./{}", path);
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&full_path,spec).unwrap();
        for t in (0..44100).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
        return path
    }
    else {
      panic!("Pass path for destination .wav file as first argument...");
    }

}

#[http_component]
fn audio_sine_wave(req: Request) -> Result<Response> {
    //println!("{:?}", req.headers());
    match parse_query_string(&req, "path") {
        Ok(e) => {
            let result = make_wave(e);
            Ok(http::Response::builder()
                .status(200)
                .body(Some(format!("Made wave to path: {}", result).into()))?)
        }
        Err(e) => {
            Ok(http::Response::builder()
                .status(500)
                .body(Some(format!("{}", e).into()))?)
        }
    }
}

