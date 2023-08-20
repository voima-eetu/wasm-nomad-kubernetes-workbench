use aes::Aes128;
use anyhow::Result;
use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};
use {std::collections::HashMap, url::Url};

use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cfb};
use rand::Rng;

type Aes128Cfb = Cfb<Aes128, Pkcs7>;

const LETTER_BYTES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const KEY: &[u8; 16] = b"\xa1\xf6%\x8c\x87}_\xcd\x89dHE8\xbf\xc9,";


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

fn random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..LETTER_BYTES.len());
            LETTER_BYTES.chars().nth(idx).unwrap()
        })
        .collect()
}

fn encrypt(key: &[u8], message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let iv = &key[..aes::BLOCK_SIZE]; // For simplicity, using the key as the IV as well
    let cipher = Aes128Cfb::new_from_slices(key, iv)?;
    let cipher_text = cipher.encrypt_vec(message.as_bytes());
    Ok(base64::encode(&cipher_text))
}

fn decrypt(key: &[u8], encrypted: &str) -> Result<String, Box<dyn std::error::Error>> {
    let iv = &key[..aes::BLOCK_SIZE]; // The same IV used for encryption
    let cipher_text = base64::decode(encrypted)?;
    let cipher = Aes128Cfb::new_from_slices(key, iv)?;
    let decrypted = cipher.decrypt_vec(&cipher_text)?;
    Ok(String::from_utf8(decrypted)?)
}

fn aes(req: Request<String>) -> bytecodec::Result<Response<String>> {
    let length_keyword = parse_query_string(&req, "length");
    if length_keyword.is_err() {
        return Ok(Response::new(
                  HttpVersion::V1_0,
                  StatusCode::new(500)?,
                  ReasonPhrase::new("")?,
                  format!("lenght argument is missing...").into()
        ))
    }
    let length : usize = length_keyword.ok().unwrap().parse().unwrap_or(0);

    let iterations_keyword = parse_query_string(&req, "iterations");
    if iterations_keyword.is_err() {
        return Ok(Response::new(
                  HttpVersion::V1_0,
                  StatusCode::new(500)?,
                  ReasonPhrase::new("")?,
                  format!("iteration argument is missing...").into()
        ))
    }
    let iterations : usize = iterations_keyword.ok().unwrap().parse().unwrap_or(0);
    
    let message = random_string(length);
    let mut response = format!("CLEAR TEXT MESSAGE: {}\n\n", message);
    let mut status = 500;

    for _ in 0..iterations {
        match encrypt(KEY, &message) {
            Ok(encrypted) => {
                response.push_str(&format!("\tENCRYPTED: {}\n", encrypted));
                match decrypt(KEY, &encrypted) {
                    Ok(decrypted) =>  {
                        response.push_str(&format!("\tDECRYPTED: {}\n\n", decrypted));
                        status = 200;
                    },
                    Err(e) => {
                        response.push_str("Error in decryption\n\n");
                        status = 500;
                    }
                }
            },
            Err(e) => {
                response.push_str("Error in encryption\n\n");
                status = 500;
            }
        }
    }
    Ok(Response::new(
      HttpVersion::V1_0,
      StatusCode::new(status)?,
      ReasonPhrase::new("")?,
      format!("{:?}", response).into()
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
        Ok(req) => aes(req),
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
