use anyhow::Result;
use {std::collections::HashMap, url::Url};
use hound;
use std::f32::consts::PI;
use std::i16;
use std::io::Write;
use hound::Sample;

fn parse_query_string(url: &str, parameter_name: &str) -> Result<String, String> {
    let parsed_url = Url::parse(&url).or_else(|_e| {
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

fn make_wave() {
    
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
            let v = spec.into_header_for_infinite_file();

    let so = std::io::stdout();
    let mut so = so.lock();
    so.write_all(&v[..]).unwrap();
        //let mut writer = hound::WavWriter::create(&full_path,spec).unwrap();
        for t in (0..44100).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            let x : i16 = (sample * amplitude) as i16;
            if x.write(&mut so, 16).is_err() {
                println!("Err");
            }
            //writer.write_sample((sample * amplitude) as i16).unwrap();
        }

}

fn main() {
    println!("Content-Type: audio/wav\n");
    make_wave();
    //println!("{:?}", req.headers());
    //
    /*let mut full_url = String::new();
    std::env::vars().for_each(|v| {
        if v.0 == "X_FULL_URL" {
             full_url = v.1;
        }
    });
    

    match parse_query_string(full_url.as_str(), "path") {
        Ok(e) => {
            let result = make_wave(e);
            println!("Status: 200");
        },
        Err(e) => {
            println!("Status: 500");
        }
    }*/
}
