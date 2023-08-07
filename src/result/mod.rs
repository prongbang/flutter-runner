use std::time::{SystemTime, Duration};

pub struct Result {
    pub test: String,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub status: String,
    pub error: String,
}

impl Result {
    pub fn new() -> Self {
        Self {
            test: String::new(),
            start_time: SystemTime::now(),
            end_time: SystemTime::now(),
            status: String::new(),
            error: String::new(),
        }
    }

    pub fn get_time_usage(&self) -> i64 {
        let duration = self.end_time.duration_since(self.start_time).unwrap_or(Duration::from_secs(0));
        let milliseconds = duration.as_millis() as i64;
        let seconds = milliseconds / 1000;
        seconds
    }

    pub fn get_time_minute(&self) -> f32 {
        let time_usage = self.get_time_usage();
        let time_minute = time_usage as f32 / 60.0;
        time_minute
    }
}