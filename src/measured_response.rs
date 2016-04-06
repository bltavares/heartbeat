use std::time::Duration;

use stopwatch::Stopwatch;

use hyper::Client;
use hyper::header::Connection;
use hyper::status::StatusCode;
use hyper::Url;

use time::Duration as TimeDuration;

#[derive(Debug)]
pub struct MeasuredResponse {
    pub time: TimeDuration,
    pub status: StatusCode,
    url: Url,
}

#[cfg(test)]
impl Default for MeasuredResponse {
    fn default() -> MeasuredResponse {
        MeasuredResponse {
            time: TimeDuration::zero(),
            status: StatusCode::Ok,
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
        self.status.is_success()
    }

    pub fn request(url: &str) -> MeasuredResponse {
        let mut client = Client::new();
        client.set_read_timeout(Some(Duration::from_secs(10)));

        let request = client.get(url)
                            .header(Connection::close());

        let stop_watch = Stopwatch::start_new();
        let response = request.send().expect("Could not make request");
        let duration = stop_watch.elapsed();

        MeasuredResponse {
            status: response.status,
            url: response.url.clone(),
            time: duration,
        }
    }

    #[cfg(test)]
    pub fn empty_failure() -> MeasuredResponse {
        let mut response = MeasuredResponse::default();
        response.status = StatusCode::InternalServerError;
        response
    }
}
