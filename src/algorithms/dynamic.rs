#![allow(dead_code)]

use ndarray::Array2;

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;
use super::algorithm::{Algorithm, TSPResult};

type City = Point<i32>;
type Route = Vec<usize>;
type Cities = Vec<City>;
type Graph = graph::Graph<f32>;
type Population = Vec<TravelRoute>;
type Matrix<T> = Array2<T>;

pub struct Dynamic {
    graph: Graph,
    path: Matrix<i32>,
    dp: Matrix<f32>,
    min_path: f32,
    size: usize,
    pos: usize,
    visited: usize
}

impl Dynamic {
    fn tsd(&mut self, mask: i32, source: i32) -> f32 {
        todo!();
    }
}

impl Algorithm for Dynamic {
    fn run(&mut self, input_size: usize) -> TSPResult {
        todo!();
    }
}