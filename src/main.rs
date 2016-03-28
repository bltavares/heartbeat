extern crate clap;
use clap::App;

fn main() {
  App::new("heartbeat").version("v0.1.0-beta").get_matches();
}
