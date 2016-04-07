use arrayvec::ArrayVec;
use measured_response::MeasuredResponse;

const LAST_REQUEST_STORAGE_SIZE: usize = 10;

pub struct Summary {
    pub total_requests: u64,
    total_success: u64,
    last_requests: ArrayVec<[MeasuredResponse; LAST_REQUEST_STORAGE_SIZE]>,
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            total_requests: 0,
            total_success: 0,
            last_requests: ArrayVec::<[MeasuredResponse; LAST_REQUEST_STORAGE_SIZE]>::new(),
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

        self.last_requests.insert(0, response);
    }

    pub fn last_requests(&self) -> &[MeasuredResponse] {
        &self.last_requests
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

#[test]
fn it_should_store_the_last_few_requests() {
    let mut summary = Summary::new();

    for _ in 0..10 {
        summary.push(MeasuredResponse::default());
    }
    summary.push(MeasuredResponse::empty_failure());

    assert_eq!(summary.last_requests(),
               &[MeasuredResponse::empty_failure(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default(),
                 MeasuredResponse::default()]);
}
