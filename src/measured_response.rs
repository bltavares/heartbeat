use std::time::Duration;

use stopwatch::Stopwatch;

use hyper::Client;
use hyper::client::response::Response;
use hyper::header::Connection;
use hyper::status::StatusCode;

use time::Duration as TimeDuration;

#[derive(Debug)]
pub struct MeasuredResponse {
    pub time: TimeDuration,
    response: Response,
}

impl MeasuredResponse {
    pub fn status(&self) -> &StatusCode {
        &self.response.status
    }

    pub fn url(&self) -> String {
        self.response.url.serialize()
    }

    pub fn std_time(&self) -> Duration {
        self.time.to_std().expect("MeasuredResponse time should never be negative")
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
            response: response,
            time: duration,
        }
    }
}
