use hound;
use std::f32::consts::PI;
use std::i16;

use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

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

fn make_wave(path: String) -> String {

    if path.len() > 2 {
      let spec = hound::WavSpec {
          channels: 1,
          sample_rate: 44100,
          bits_per_sample: 16,
          sample_format: hound::SampleFormat::Int,
      };
      let mut writer = hound::WavWriter::create(&path, spec).unwrap();
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

fn audio_sine_wave(req: Request<String>) -> bytecodec::Result<Response<String>> {
    match parse_query_string(&req, "path") {
        Ok(e) => {
            let result = make_wave(e);
            return Ok(Response::new(
                      HttpVersion::V1_0,
                      StatusCode::new(200)?,
                      ReasonPhrase::new("")?,
                      format!("Made wave to path: {}", result).into(),
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
        Ok(req) => audio_sine_wave(req),
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
