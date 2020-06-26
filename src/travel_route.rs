#![allow(dead_code)]

use num_traits::{pow, Float, Num};
use crate::graph::Graph;
use crate::point::Point;

type City = Point<i32>;
pub struct TravelRoute {
    cities_dist: Graph<f32>,
    route: Vec<usize>,
    cities: Vec<City>,
    fitness: f32,
    distance: f32,
}