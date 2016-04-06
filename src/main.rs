extern crate clap;
extern crate hyper;
extern crate stopwatch;
extern crate time;

mod measured_response;
mod summary;

use measured_response::MeasuredResponse;
use summary::Summary;

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

    let mut summary = Summary::new();

    loop {
        let measured_response = MeasuredResponse::request(&application_configuration.url);
        let next_tick = application_configuration.next_request_in(measured_response.std_time());

        summary.push(measured_response);

        display(&summary);
        std::thread::sleep(next_tick);
    }
}

fn display(summary: &Summary) {
    println!("Total");
    println!("Requests: {} - Success: {}/{}% - Failure: {}/{}%",
             summary.total_requests,
             summary.total_success(),
             summary.total_percentual_success(),
             summary.total_failure(),
             summary.total_percentual_failure());
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
