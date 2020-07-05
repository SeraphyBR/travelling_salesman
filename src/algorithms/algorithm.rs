
pub trait Algorithm {
    fn run(&mut self, input_size: usize);
    fn run_in_range(&mut self, begin: usize, end: usize);
    fn run_in_range_statistic(&mut self, begin: usize, end: usize);
}