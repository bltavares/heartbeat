extern crate clap;
extern crate hyper;
extern crate stopwatch;
extern crate time;

mod measured_response;

use measured_response::MeasuredResponse;

use std::str::FromStr;
use std::time::Duration;

use clap::{App, Arg};

const DEFAULT_INTERVAL_IN_SECONDS: u64 = 10;

struct ApplicationConfiguration {
    url: String,
    interval: Duration,
}

fn main() {
    let interval_help_message = format!("The interval in seconds between requests, default to {} \
                                         seconds",
                                        DEFAULT_INTERVAL_IN_SECONDS);
    let matches = App::new("heartbeat")
                      .version("v0.1.0-beta")
                      .arg(Arg::with_name("url")
                               .long("url")
                               .index(1)
                               .takes_value(true)
                               .value_name("URL")
                               .help("The URL to monitor")
                               .required(true))
                      .arg(Arg::with_name("interval")
                               .long("interval")
                               .short("i")
                               .takes_value(true)
                               .value_name("INTERVAL")
                               .help(&interval_help_message))
                      .get_matches();


    let interval_argument = matches.value_of("interval")
                                   .map(|arg| {
                                       u64::from_str(arg)
                                           .expect("The interval argument requires a number")
                                   });

    let application_configuration = ApplicationConfiguration {
        url: matches.value_of("url").expect("URL not present").to_string(),
        interval: Duration::from_secs(interval_argument.unwrap_or(DEFAULT_INTERVAL_IN_SECONDS)),
    };

    loop {
        let measured_response = MeasuredResponse::request(&application_configuration.url);
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
