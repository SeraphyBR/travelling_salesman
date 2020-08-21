pub use std::time::{Duration, Instant};
use std::fmt;

pub struct TSPResult {
    algorithm_name: String,
    input_size: usize,
    min_dist: f32,
    min_path: Vec<usize>,
    time: Duration
}

impl TSPResult {
    pub fn with_values(algorithm_name: String, input_size: usize, min_dist: f32, min_path: &[usize], time: Duration) -> TSPResult {
        TSPResult {
            algorithm_name,
            input_size,
            min_dist,
            min_path: Vec::from(min_path),
            time
        }
    }
}

impl fmt::Display for TSPResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Algorithm: {}", self.algorithm_name)?;
        writeln!(f, "Input size: {}", self.input_size)?;
        writeln!(f, "Min dist: {}", self.min_dist)?;
        writeln!(f, "Min path: {:?}", self.min_path)?;
        writeln!(f, "Time taken: {:?}", self.time)
    }
}