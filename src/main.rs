#[macro_use] extern crate log;

use consul;
use std::fs;
use std::env;
use clap::{App, Arg};
use regex::Regex;
use std::process::exit;
use std::io::{Read, Write};
use consul::kv::KV;
use std::env::args;
use base64;

fn get_env(key: &str) -> Result<String, ()> {
    match env::var(key) {
        Ok(value)    => Ok(value),
        _                   => {
            warn!("the environment does not contain the key {}", key);
            Err(())
        }
    }
}

fn get_consul(key: &str) -> Result<String, ()> {
    let config = consul::Config::new();
    let mut config = config.unwrap();
    let client = consul::Client::new(config);

    let lookup = client.get(key, None);
    if lookup.is_err() {
        warn!("error occurred during consul lookup. {:?}", lookup.err().unwrap());
        return Err(());
    }
    let kv = lookup.unwrap().0;
    let inner = if kv.is_none() {
        return Err(())
    } else {
        kv.unwrap().Value
    };

    let decoder = base64::decode(&inner);
    if decoder.is_err() {

    }

    let decoded = String::from_utf8(decoder.unwrap());

    return Ok(decoded.unwrap());
}

fn main() {
    env_logger::init();
    let matches = App::new("configmapper")
        .version("0.1.3")
        .author("Colum")
        .about("Does awesome things")
        .arg(Arg::with_name("input").short("i").long("input").help("Input file").required(true).takes_value(true))
        .arg(Arg::with_name("output").short("o").long("output").help("Output file").required(true).takes_value(true))
        .get_matches();

    let mut input = "".to_string();
    if let Some(inner) = matches.value_of("input") {
        input = inner.into();
    } else {
        error!("-i/--input required");
        exit(2);
    }

    let mut output = "".to_string();
    if let Some(inner) = matches.value_of("output") {
        output = inner.into();
    } else {
        error!("-o/--output required");
        exit(2);
    }

    let fs_handle = fs::File::open(input.clone());
    if fs_handle.is_err() {
        error!("unable to open {}. {:?}", &input, fs_handle.err().unwrap());
        exit(2);
    }

    let mut raw = String::new();
    let file_read = fs_handle.unwrap().read_to_string(&mut raw);
    if file_read.is_err() {
        error!("unable to load {}. {:?}", &input, file_read.err().unwrap());
        exit(2);
    }

    let re = Regex::new("\\{\\{(.*?)\\|(.*?)}}").unwrap();
    let iraw = raw.clone();
    for inner in re.find_iter(&iraw) {
        // we can assume because we're here, the capture is valid
        let captures = re.captures(inner.as_str()).unwrap();
        let source = captures.get(1).map_or("", |m| m.as_str());
        let value = captures.get(2).map_or("", |m| m.as_str());

        let result = match source {
            "ENV"           => get_env(value),
            "CONSUL_KV"     => get_consul(value),
            _               => {
                warn!("undefined search type: {}", source);
                Err(())
            }
        };

        if result.is_err() {
            error!("an error occured while matching {}", inner.as_str());
            exit(1);
        }

        let result = result.unwrap();
        raw = raw.replace(inner.as_str(), result.as_str());

    }

    info!("writing {}", output);
    let fs_handle = fs::File::create(output.clone());
    if fs_handle.is_err() {
        error!("unable to open {}. {:?}", &output, fs_handle.err().unwrap());
        exit(2);
    }

    let mut fs_handle = fs_handle.unwrap();
    fs_handle.write(raw.as_bytes());
}