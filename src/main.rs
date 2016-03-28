extern crate hyper;
extern crate clap;

use std::io::Read;

use clap::{App, Arg};

use hyper::Client;
use hyper::header::Connection;

fn main() {
  let matches = App::new("heartbeat")
      .version("v0.1.0-beta")
      .arg(Arg::with_name("url")
           .long("url")
           .index(1)
           .takes_value(true)
           .value_name("URL")
           .required(true))
      .get_matches();

  request(matches.value_of("url").expect("URL not present"));
}

fn request(url: &str) {
    let client = Client::new();

    let mut response = client.get(url)
        .header(Connection::close())
        .send().expect("Could not make request");

    let mut body = String::new();
    response.read_to_string(&mut body).expect("Could not read the response into a buffer");

    println!("Response: {}", body);
}
