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

pub struct BranchBound {
    graph: Graph,
    lower_bound: f32,
    min_dist: f32,
    min_path: Route
}

impl BranchBound {
    fn permutation(&mut self, arr: &mut [usize], i: usize) {
        let n = arr.len();
        if i == n {
            let weight = self.weight_of_path(arr);
            if weight <= self.min_dist {
                self.min_path = Route::from(arr);
                self.min_dist = weight;
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
            if weight > self.lower_bound && weight > self.min_dist {
                return f32::MAX;
            }
        }
        weight += self.graph.get_connection(path[path.len() - 1], 0);
        weight
    }

    fn calculate_lower_bound(&mut self) {
        let input_size = self.graph.get_vertex_count();
        let mut sum = 0.0;
        if input_size > 1 {
            for i in 0..input_size {
                let tmp = self.graph.get_connection(i, i);
                self.graph.set_connection(i, i, f32::MAX);

                let mut min1 = self.graph.get_connection(i, 0);
                let mut min2 = self.graph.get_connection(i, 1);

                if min2 < min1 {
                    min1 = self.graph.get_connection(i, 1);
                    min2 = self.graph.get_connection(i, 0);
                }

                for j in 2..input_size {
                    let aux = self.graph.get_connection(j, j);
                    if tmp < min1 {
                        min2 = min1;
                        min1 = aux;
                    }
                    else if tmp < min2 {
                        min2 = aux;
                    }
                }
                self.graph.set_connection(i, i, tmp);
                sum += min1 + min2;
            }
        }
        self.lower_bound = sum;
    }
}

impl Algorithm for BranchBound {
    fn run(&mut self, input_size: usize) -> TSPResult {
        let mut graph_path: Route = (0..input_size).collect();
        self.min_dist = f32::MAX;
        // self.graph =
        self.calculate_lower_bound();
        self.permutation(graph_path.as_mut_slice(), 1);
        TSPResult::with_values(input_size, self.min_dist, self.min_path.as_slice(), 0.0)
    }
}