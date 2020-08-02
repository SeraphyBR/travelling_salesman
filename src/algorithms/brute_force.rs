#![allow(dead_code)]

use ndarray::Array2;

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;
use super::algorithm::{Algorithm, TSPResult, Instant};

type City = Point<i32>;
type Route = Vec<usize>;
type Cities = Vec<City>;
type Graph = graph::Graph<f32>;
type Population = Vec<TravelRoute>;
type Matrix<T> = Array2<T>;

pub struct BruteForce {
    graph: Graph,
    min_dist: f32,
    min_path: Route
}

impl BruteForce {
    fn permutation(&mut self, arr: &mut [usize], i: usize) {
        let n = arr.len();
        if i == n {
            let w = self.weight_of_path(arr);
            if w <= self.min_dist {
                self.min_path = Route::from(arr);
                self.min_dist = w;
            }
            return;
        }

        for j in i..n {
            arr.swap(i, j);
            self.permutation(arr, i + 1);
            arr.swap(i, j);
        }
    }

    fn weight_of_path(&self, path: &[usize]) -> f32 {
        let mut weight = 0.0;
        for i in 1..path.len() {
            weight += self.graph.get_connection(path[i], path[i - 1]);
        }
        weight += self.graph.get_connection(path[path.len() - 1], 0);
        weight
    }
}

impl Algorithm for BruteForce {

    fn with_input(input: Vec<Point<f32>>) -> Self {
        let mut graph = Graph::new(input.len());
        for p in input {
            graph.add_point(p);
        }

        Self {
            graph,
            min_dist: f32::MAX,
            min_path: Vec::with_capacity(input.len()),
        }
    }

    fn run(&mut self) -> TSPResult {
        let input_size = self.graph.size();
        let mut graph_path: Route = (0..input_size).collect();
        let now = Instant::now();
        self.permutation(graph_path.as_mut_slice(), 1);
        TSPResult::with_values(input_size, self.min_dist, self.min_path.as_slice(), now.elapsed())
    }
}