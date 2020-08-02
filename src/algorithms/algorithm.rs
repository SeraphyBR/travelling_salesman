pub use crate::result::{TSPResult, TSPResults, Instant};
use num_traits::{pow, Float, Num, NumCast, cast};
use crate::point::Point;


pub trait Algorithm {
    fn with_input(input: Vec<Point<f32>>) -> Self;
    fn run(&mut self) -> TSPResult;
}