use measured_response::MeasuredResponse;

pub struct Summary {
    total_requests: u64,
    total_success: u64,
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            total_requests: 0,
            total_success: 0,
        }
    }

    pub fn total_success(&self) -> u64 {
        self.total_success
    }

    pub fn total_failure(&self) -> u64 {
        self.total_requests - self.total_success
    }

    pub fn total_percentual_success(&self) -> f64 {
        let scaled_value = self.total_success() as f64 * 100.0;
        scaled_value / self.total_requests as f64
    }

    pub fn total_percentual_failure(&self) -> f64 {
        let scaled_value = self.total_failure() as f64 * 100.0;
        scaled_value / self.total_requests as f64
    }

    pub fn push(&mut self, response: MeasuredResponse) {
        self.total_requests += 1;

        if response.is_success() {
            self.total_success += 1;
        }
    }
}

#[test]
fn it_should_increase_the_count_of_requests() {
    let mut summary = Summary::new();

    summary.push(MeasuredResponse::default());
    assert_eq!(1, summary.total_requests);

    summary.push(MeasuredResponse::default());
    assert_eq!(2, summary.total_requests);
}

#[test]
fn it_should_calculate_the_failures_count() {
    let mut summary = Summary::new();

    summary.push(MeasuredResponse::default());
    assert_eq!(0, summary.total_failure());
    assert_eq!(0.0, summary.total_percentual_failure());

    summary.push(MeasuredResponse::empty_failure());
    assert_eq!(1, summary.total_failure());
    assert_eq!(50.0, summary.total_percentual_failure());
}

#[test]
fn it_should_calculate_the_success_count() {
    let mut summary = Summary::new();

    summary.push(MeasuredResponse::default());
    assert_eq!(1, summary.total_success());
    assert_eq!(100.0, summary.total_percentual_success());

    summary.push(MeasuredResponse::empty_failure());
    assert_eq!(1, summary.total_success());
    assert_eq!(50.0, summary.total_percentual_success());
}
