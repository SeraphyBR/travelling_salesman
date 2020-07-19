pub use crate::result::{TSPResult, TSPResults};


pub trait Algorithm {
    fn run(&mut self, input_size: usize) -> TSPResult;

    fn run_in_range_statistic(&mut self, begin: usize, end: usize){
        todo!()
    }

    fn run_in_range(&mut self, begin: usize, end: usize) -> TSPResults {
        let mut results = TSPResults::with_capacity(end - begin);
        for input_size in begin..end {
            results.push(self.run(input_size));
        }
        results
    }
}
