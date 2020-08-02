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
    fn with_input(input: Vec<Point<f32>>) -> Self {
        let size = input.len();
        let mut graph = Graph::new(size);
        for p in input {
            graph.add_point(p);
        }

        Self {
            graph,
            path: Matrix::from_shape_fn((size, size), |(_,_)| -1),
            dp: Matrix::from_shape_fn((size, size), |(_,_)| -1.0),
            min_path: 0.0,
            size,
            pos: 0,
            visited: 0
        }
    }

    fn run(&mut self) -> TSPResult {
        todo!();
    }
}
