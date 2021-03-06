#![allow(dead_code)]

use crate::point::Point;
use ndarray::Array2;
use num_traits::{Num, Float, zero, NumCast, cast};

/// Describes a graph
#[derive(Clone)]
pub struct Graph<T: Num> {
    size: usize,
    matrix: Array2<T>,
    vertex_count: usize,
    points: Vec<Point<T>>
}

impl<T: Num + Copy> Graph<T> {
    pub fn new(size: usize) -> Graph<T> {
        Graph {
            size,
            matrix: Array2::zeros((size, size)),
            vertex_count: 0,
            points: Vec::new()
        }
    }

    pub fn set_connection(&mut self, v1: usize, v2: usize, value: T) {
        if v1 < self.size && v2 < self.size {
            self.matrix[[v1,v2]] = value;
            self.matrix[[v2,v1]] = value;
        }
    }

    pub fn del_connection(&mut self, v1: usize, v2: usize){
        if v1 < self.size && v2 < self.size {
            self.matrix[[v1,v2]] = zero();
            self.matrix[[v2,v1]] = zero();
        }
    }

    pub fn get_connection(&self, v1: usize, v2: usize) -> T {
        self.matrix[[v1,v2]]
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }
}

impl<T: Num + NumCast + Copy + Default> Graph<T> {
    pub fn add_point(&mut self, point: Point<T>) {
        for i in 0..self.vertex_count {
            let dist: f64 = point.dist_to(&self.points[i]);
            self.set_connection(i, self.vertex_count, cast(dist).unwrap_or_default());
        }

        self.points.push(point);
        self.vertex_count += 1;
    }
}
