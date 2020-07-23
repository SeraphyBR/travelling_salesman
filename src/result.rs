pub use std::time::{Duration, Instant};

pub type TSPResults = Vec<TSPResult>;

pub struct TSPResult {
    input_size: usize,
    min_dist: f32,
    min_path: Vec<usize>,
    time: Duration
}

impl TSPResult {
    pub fn with_values(input_size: usize, min_dist: f32, min_path: &[usize], time: Duration) -> TSPResult {
        TSPResult {
            input_size,
            min_dist,
            min_path: Vec::from(min_path),
            time
        }
    }
}