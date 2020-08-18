#![allow(dead_code)]

use ndarray::Array2;

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;
use super::algorithm::{Algorithm, TSPResult};

use graph::Graph;

type City = Point<i32>;
type Route = Vec<usize>;
type Cities = Vec<City>;
type Population = Vec<TravelRoute>;
type Matrix<T> = Array2<T>;

pub struct Dynamic {
    graph: Graph<f32>,
    path: Matrix<Option<usize>>,
    dp: Matrix<f32>,
    min_path: f32,
    size: usize,
    pos: usize,
    visited: usize
}

impl Dynamic {
    fn tsd(&mut self, mask: usize, source: usize) -> f32 {
        if mask == self.visited {
            return self.graph.get_connection(source, self.pos);
        }
        else if self.dp[(mask, source)] != -1.0 {
            return self.dp[(mask, source)];
        }

        let mut ans = f32::MAX;
        let mut min_ans = 0.0;
        let mut k = None;
        for i in 0..self.size {
            if mask & (1 << i) == 0 {
                min_ans = self.graph.get_connection(source, i) + self.tsd(mask | (1 << i), i);

                if ans > min_ans {
                    ans = min_ans;
                    k = Some(i);
                }
            }
        }

        self.path[(mask, source)] = k;
        self.dp[(mask, source)] = ans;
        self.dp[(mask, source)]
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
            path: Matrix::default((size, size)),
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
