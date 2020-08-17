pub use crate::result::{TSPResult, Instant};
use crate::point::Point;


pub trait Algorithm {
    fn with_input(input: Vec<Point<f32>>) -> Self;
    fn run(&mut self) -> TSPResult;
}