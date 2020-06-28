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

pub struct Dynamic {
    graph: Graph,
    path: Matrix<i32>,
    dp: Matrix<f32>,
    min_path: f32,
    size: usize,
    pos: usize,
    visited: usize
}