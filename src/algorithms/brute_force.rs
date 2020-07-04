#![allow(dead_code)]

use ndarray::Array2;

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;

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