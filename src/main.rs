extern crate clap;
extern crate hyper;
extern crate stopwatch;
extern crate time;

use clap::{App, Arg};

use stopwatch::Stopwatch;
use time::Duration;

use hyper::Client;
use hyper::header::Connection;


#[derive(Debug)]
struct MeasuredResponse {
    response: hyper::client::response::Response,
    time: Duration,
}

impl MeasuredResponse {
    fn status(&self) -> &hyper::status::StatusCode {
        &self.response.status
    }
}

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

    let measured_response = request(matches.value_of("url").expect("URL not present"));
    display(&measured_response);
}

fn display(response: &MeasuredResponse) {
    let status = response.status();
    let duration = response.time;
    println!("Status: {}, time: {}s", status, duration);
}

fn request(url: &str) -> MeasuredResponse {
    let mut client = Client::new();
    client.set_read_timeout(Some(std::time::Duration::from_secs(10)));

    let request = client.get(url)
                        .header(Connection::close());

    let stop_watch = Stopwatch::start_new();
    let response = request.send().expect("Could not make request");
    let duration = stop_watch.elapsed();

    MeasuredResponse {
        response: response,
        time: duration,
    }
}
