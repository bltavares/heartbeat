extern crate clap;
extern crate hyper;
extern crate stopwatch;
extern crate time;

use std::str::FromStr;
use std::time::Duration;

use time::Duration as TimeDuration;

use clap::{App, Arg};

use stopwatch::Stopwatch;

use hyper::Client;
use hyper::header::Connection;


#[derive(Debug)]
struct MeasuredResponse {
    response: hyper::client::response::Response,
    time: TimeDuration,
}

impl MeasuredResponse {
    fn status(&self) -> &hyper::status::StatusCode {
        &self.response.status
    }

    fn url(&self) -> String {
        self.response.url.serialize()
    }
}

const DEFAULT_INTERVAL_IN_SECONDS: u64 = 10;

struct ApplicationConfiguration {
    url: String,
    interval: Duration,
}

fn validate_interval_argument(arg: String) -> Result<(), String> {
    match u64::from_str(&arg) {
        Ok(_) => Ok(()),
        Err(_) => Err("The interval argument requires a number".to_string()),
    }
}

fn main() {
    let interval_help_message = format!("The interval in seconds between requests, default to {}",
                                        DEFAULT_INTERVAL_IN_SECONDS);
    let matches = App::new("heartbeat")
                      .version("v0.1.0-beta")
                      .arg(Arg::with_name("url")
                               .long("url")
                               .index(1)
                               .takes_value(true)
                               .value_name("URL")
                               .required(true))
                      .arg(Arg::with_name("interval")
                               .long("interval")
                               .takes_value(true)
                               .value_name("INTERVAL")
                               .validator(validate_interval_argument)
                               .help(&interval_help_message))
                      .get_matches();


    let interval_argument = matches.value_of("interval").and_then(|arg| u64::from_str(arg).ok());

    let application_configuration = ApplicationConfiguration {
        url: matches.value_of("url").expect("URL not present").to_string(),
        interval: Duration::from_secs(interval_argument.unwrap_or(DEFAULT_INTERVAL_IN_SECONDS)),
    };

    loop {
        let measured_response = request(&application_configuration.url);
        display(&measured_response);
        std::thread::sleep(application_configuration.interval);
    }
}

fn display(response: &MeasuredResponse) {
    let status = response.status();
    let duration = response.time;
    let url = response.url();
    println!("{} -> Status: {}, time: {}s", url, status, duration);
}

fn request(url: &str) -> MeasuredResponse {
    let mut client = Client::new();
    client.set_read_timeout(Some(Duration::from_secs(10)));

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
