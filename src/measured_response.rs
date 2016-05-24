use std::fmt;
use std::time::Duration;

use hyper::Client;
use hyper::Url;
use hyper::header::Connection;
use hyper::status::StatusCode;
use hyper::client::RedirectPolicy;

use stopwatch::Stopwatch;

use time::Duration as TimeDuration;

#[derive(Debug, Eq, PartialEq)]
pub enum StatusOrError {
    Status(StatusCode),
    ResponseError,
}

impl fmt::Display for StatusOrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatusOrError::Status(status) => status.fmt(f),
            StatusOrError::ResponseError => write!(f, "Response error"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MeasuredResponse {
    pub time: TimeDuration,
    pub status: StatusOrError,
    url: Url,
}

impl Default for MeasuredResponse {
    fn default() -> MeasuredResponse {
        MeasuredResponse {
            time: TimeDuration::zero(),
            status: StatusOrError::Status(StatusCode::Ok),
            url: Url::parse("http://example.com").unwrap(),
        }
    }
}

impl MeasuredResponse {
    pub fn url(&self) -> String {
        self.url.serialize()
    }

    pub fn std_time(&self) -> Duration {
        self.time.to_std().expect("MeasuredResponse time should never be negative")
    }

    pub fn is_success(&self) -> bool {

        match self.status {
            StatusOrError::ResponseError => false,
            StatusOrError::Status(status) => status.is_success(),
        }
    }

    pub fn request(url: &str, timeout: Duration, redirect_count: u8) -> MeasuredResponse {
        let mut client = Client::new();
        client.set_read_timeout(Some(timeout));
        client.set_redirect_policy(RedirectPolicy::FollowCount(redirect_count));

        let request = client.get(url)
                            .header(Connection::close());

        let stop_watch = Stopwatch::start_new();

        match request.send() {
            Err(_) => MeasuredResponse::empty_failure(),
            Ok(response) => {
                let duration = TimeDuration::from_std(stop_watch.elapsed()).expect("Could not measure elapsed response time");

                MeasuredResponse {
                    status: StatusOrError::Status(response.status),
                    url: response.url.clone(),
                    time: duration,
                }
            }
        }
    }

    pub fn empty_failure() -> MeasuredResponse {
        let mut response = MeasuredResponse::default();
        response.status = StatusOrError::ResponseError;
        response
    }
}
