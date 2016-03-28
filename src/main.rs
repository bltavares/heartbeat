extern crate clap;
use clap::{App, Arg};

fn main() {
  App::new("heartbeat")
      .version("v0.1.0-beta")
      .arg(Arg::with_name("url")
           .long("url")
           .index(1)
           .takes_value(true)
           .value_name("URL")
           .required(true))
      .get_matches();
}
