extern crate arrayvec;
extern crate clap;
extern crate hyper;
extern crate screenprints;
extern crate stopwatch;
extern crate time;
extern crate url;

mod measured_response;
mod summary;

use measured_response::MeasuredResponse;
use summary::Summary;

use std::io::{stdout, Write};
use std::str::FromStr;
use std::time::Duration;

use clap::{App, Arg};

use screenprints::Printer;

use url::Url;

const DEFAULT_INTERVAL_IN_SECONDS: u64 = 10;
const DEFAULT_TIMEOUT_IN_SECONDS: u64 = 10;
const DEFAULT_REDIRECT_LIMIT_COUNT: u8 = 5;

struct ApplicationConfiguration {
    url: String,
    interval: Duration,
    timeout: Duration,
    redirect_limit: u8,
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

fn url_validator(arg: String) -> Result<(), String> {
    match Url::parse(&arg) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err("The url argument must be complete, specifying the protocol as well. For example: \
                 http://example.com"
                    .to_string())
        }
    }
}

fn main() {
    let application_configuration = parse_arguments();

    let mut summary = Summary::new();
    let mut printer = Printer::new(stdout(), Duration::from_millis(10));

    loop {
        let measured_response = MeasuredResponse::request(&application_configuration.url,
                                                          application_configuration.timeout,
                                                          application_configuration.redirect_limit);
        let next_tick = application_configuration.next_request_in(measured_response.std_time());

        summary.push(measured_response);

        display(&summary, &mut printer);
        std::thread::sleep(next_tick);
    }
}

fn display(summary: &Summary, printer: &mut Write) {
    let requests = summary.last_requests()
                          .iter()
                          .map(|req| {
                              format!("{} -> Status: {}, Response Time: {}",
                                      req.url(),
                                      req.status,
                                      req.time)
                          })
                          .collect::<Vec<_>>();

    let _ = write!(printer,
                   "Total\r\nRequests: {} - Success: {}/{:.1}% - Failure: {}/{:.1}%\r\n\r\nLast \
                    requests\r\n{}",
                   summary.total_requests,
                   summary.total_success(),
                   summary.total_percentual_success(),
                   summary.total_failure(),
                   summary.total_percentual_failure(),
                   requests.join("\r\n"));
}

fn parse_arguments() -> ApplicationConfiguration {
    let interval_help_message = format!("The interval in seconds between requests, default to {} \
                                         seconds",
                                        DEFAULT_INTERVAL_IN_SECONDS);
    let timeout_help_message = format!("The timeout in seconds to wait for a response, default \
                                        to {} seconds",
                                       DEFAULT_TIMEOUT_IN_SECONDS);

    let redirect_limit_message = format!("The amount of redirects to follow, defaults to {} \
                                          redirects",
                                         DEFAULT_REDIRECT_LIMIT_COUNT);

    let cli_arguments = App::new("heartbeat")
                            .version("v0.1.0-beta")
                            .arg(Arg::with_name("interval")
                                     .long("interval")
                                     .short("i")
                                     .takes_value(true)
                                     .value_name("INTERVAL")
                                     .help(&interval_help_message))
                            .arg(Arg::with_name("timeout")
                                     .long("timeout")
                                     .short("t")
                                     .takes_value(true)
                                     .value_name("TIMEOUT")
                                     .help(&timeout_help_message))
                            .arg(Arg::with_name("redirect_limit")
                                     .long("follow")
                                     .short("F")
                                     .takes_value(true)
                                     .value_name("FOLLOW COUNT")
                                     .help(&redirect_limit_message))
                            .arg(Arg::with_name("url")
                                     .long("url")
                                     .index(1)
                                     .takes_value(true)
                                     .value_name("URL")
                                     .help("The URL to monitor")
                                     .validator(url_validator)
                                     .required(true))
                            .get_matches();


    let interval_argument = cli_arguments.value_of("interval").map(|arg| {
        u64::from_str(arg).expect("The interval argument requires a number")
    });

    let timeout_argument = cli_arguments.value_of("timeout").map(|arg| {
        u64::from_str(arg).expect("The timeout argument requires a number")
    });

    let redirect_limit_argument = cli_arguments.value_of("redirect_limit").map(|arg| {
        u8::from_str(arg).expect("The redirect limit requires a number")
    });

    ApplicationConfiguration {
        url: cli_arguments.value_of("url").expect("URL not present").to_string(),
        interval: Duration::from_secs(interval_argument.unwrap_or(DEFAULT_INTERVAL_IN_SECONDS)),
        timeout: Duration::from_secs(timeout_argument.unwrap_or(DEFAULT_TIMEOUT_IN_SECONDS)),
        redirect_limit: redirect_limit_argument.unwrap_or(DEFAULT_REDIRECT_LIMIT_COUNT),
    }
}

#[test]
fn next_tick_should_remove_the_time_spent_on_the_request() {
    let configuration = ApplicationConfiguration {
        url: Default::default(),
        interval: Duration::from_secs(3),
        timeout: Duration::from_secs(0),
        redirect_limit: 1,
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
        timeout: Duration::from_secs(0),
        redirect_limit: 1,
    };

    let time_spent = Duration::from_secs(5);

    assert_eq!(configuration.next_request_in(time_spent),
               Duration::from_secs(0));
}
