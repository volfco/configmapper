#[macro_use] extern crate log;
#[macro_use] extern crate handlebars;
#[macro_use] extern crate serde_json;

use std::fs;
use std::env;
use clap::{App, Arg};
use regex::Regex;
use std::process::exit;
use std::io::{Read, Write};
use std::env::args;
use base64;


use handlebars::{Handlebars, Output, RenderContext, Context, Helper, RenderError, JsonRender};

mod config;
mod modules;


fn main() {
    env_logger::init();
    let matches = App::new("configmapper")
        .version("0.2.1")
        .author("Colum McGaley <colum@volf.co>")
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

    let mut output_path = "".to_string();
    if let Some(inner) = matches.value_of("output") {
        output_path = inner.into();
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


    let mut reg = Handlebars::new();

    // register the helper functions that power the magic of this program
    reg.register_helper("env", Box::new(functions::get_env));
    reg.register_helper("consul", Box::new(functions::get_consul));

    // TODO we should allow n to be controlled via configuration
    let mut output = raw;
    for i in 0..2 {
        debug!("starting iteration {}", &i);
        let template = reg.render_template(output.as_str(), &json!());
        if template.is_err() {
            error!("unable to render the template");
            // TODO make the error handling better
            error!("{:?}", template.err().unwrap());
            exit(1);
        }

        output = template.unwrap();
    }


    info!("writing {}", output_path);
    let fs_handle = fs::File::create(output_path.clone());
    if fs_handle.is_err() {
        error!("unable to open {}. {:?}", &output_path, fs_handle.err().unwrap());
        exit(2);
    }

    let mut fs_handle = fs_handle.unwrap();
    fs_handle.write(raw.as_bytes());
}