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

impl ApplicationConfiguration {
    fn next_request_in(&self, time_spent: Duration) -> Duration {
        if self.interval >= time_spent {
            self.interval - time_spent
        } else {
            Duration::new(0, 0)
        }
    }
}

fn main() {
    let application_configuration = parse_arguments();

    loop {
        let measured_response = MeasuredResponse::request(&application_configuration.url);
        display(&measured_response);
        std::thread::sleep(application_configuration.next_request_in(measured_response.std_time()));
    }
}

fn display(response: &MeasuredResponse) {
    let status = response.status();
    let duration = response.time;
    let url = response.url();
    println!("{} -> Status: {}, time: {}", url, status, duration);
}

fn parse_arguments() -> ApplicationConfiguration {
    let interval_help_message = format!("The interval in seconds between requests, default to {} \
                                         seconds",
                                        DEFAULT_INTERVAL_IN_SECONDS);

    let cli_arguments = App::new("heartbeat")
                            .version("v0.1.0-beta")
                            .arg(Arg::with_name("interval")
                                     .long("interval")
                                     .short("i")
                                     .takes_value(true)
                                     .value_name("INTERVAL")
                                     .help(&interval_help_message))
                            .arg(Arg::with_name("url")
                                     .long("url")
                                     .index(1)
                                     .takes_value(true)
                                     .value_name("URL")
                                     .help("The URL to monitor")
                                     .required(true))
                            .get_matches();


    let interval_argument = cli_arguments.value_of("interval").map(|arg| {
        u64::from_str(arg).expect("The interval argument requires a number")
    });

    ApplicationConfiguration {
        url: cli_arguments.value_of("url").expect("URL not present").to_string(),
        interval: Duration::from_secs(interval_argument.unwrap_or(DEFAULT_INTERVAL_IN_SECONDS)),
    }
}

#[test]
fn next_tick_should_remove_the_time_spent_on_the_request() {
    let configuration = ApplicationConfiguration {
        url: Default::default(),
        interval: Duration::from_secs(3),
    };

    let time_spent = Duration::from_secs(1);

    assert_eq!(configuration.next_request_in(time_spent),
               Duration::from_secs(2));
}

#[test]
fn next_tick_should_be_right_away_when_more_time_is_spent() {
    let configuration = ApplicationConfiguration {
        url: Default::default(),
        interval: Duration::from_secs(3),
    };

    let time_spent = Duration::from_secs(5);

    assert_eq!(configuration.next_request_in(time_spent),
               Duration::from_secs(0));
}
