pub type TSPResults = Vec<TSPResult>;

pub struct TSPResult {
    input_size: usize,
    min_dist: f32,
    min_path: Vec<usize>,
    time: f64
}

impl TSPResult {
    pub fn with_values(input_size: usize, min_dist: f32, min_path: &[usize], time: f64) -> TSPResult {
        TSPResult {
            input_size,
            min_dist,
            min_path: Vec::from(min_path),
            time
        }
    }
}