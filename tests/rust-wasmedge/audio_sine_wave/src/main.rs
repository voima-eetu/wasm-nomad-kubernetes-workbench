use hound;
use std::f32::consts::PI;
use std::i16;

use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

use hound::Sample;

fn make_wave(mut stream: &TcpStream) -> bytecodec::Result<Response<String>> {

      let spec = hound::WavSpec {
          channels: 1,
          sample_rate: 44100,
          bits_per_sample: 16,
          sample_format: hound::SampleFormat::Int,
      };
    let v = spec.into_header_for_infinite_file();
//    let so = std::io::stdout();
//    let mut so = so.lock();
//    so.write_all(&v[..]).unwrap();
        //let mut writer = hound::WavWriter::create(&full_path,spec).unwrap();
        //stream.write("Content-Type: text/plain\n".as_bytes())?;
         let mut headers = ["HTTP/1.1 200 OK", "content-type: audio/wav","content-length: 88244", "\r\n"].join("\r\n").to_string().into_bytes();
        headers.extend(&v);
        stream.write_all(&headers[..]).unwrap();
        for t in (0..44100).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            let x : i16 = (sample * amplitude) as i16;
            if x.write(&mut stream, 16).is_err() {
                println!("Err");
            }
            //writer.write_sample((sample * amplitude) as i16).unwrap();
        }
            return Ok(Response::new(
                      HttpVersion::V1_0,
                      StatusCode::new(200)?,
                      ReasonPhrase::new("")?,
                      format!("Made wave" ).into(),
            ))
}

/*fn audio_sine_wave(req: Request<String>) -> bytecodec::Result<Response<String>> {
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
}*/

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
        Ok(req) => make_wave(&stream),
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
