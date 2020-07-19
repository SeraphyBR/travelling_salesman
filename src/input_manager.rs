#![allow(dead_code)]

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;

use rand::Rng;
use rand::thread_rng;
use num_traits::{pow, Float, Num, NumCast, cast};

type Graph = graph::Graph<f32>;

const MAX_XY: usize = 1000;

fn gen_points(size: usize) -> Vec<Point<i32>> {
    let mut points = Vec::with_capacity(size);

    let mut rng = thread_rng();
    loop {
        let x: i32 = rng.gen_range(1, MAX_XY);
        let y: i32 = rng.gen_range(1, MAX_XY);

        let new_p = Point::new(points.len() + 1, (x, y));

        match points.into_iter().find(|p| p.coordinates() == new_p.coordinates()) {
            Some(_) => {},
            None => points.push(new_p)
        }

        if points.len() == size {
            break;
        }
    }
    points
}

fn gen_random_input(){

}

pub fn gen_all_allowed_random_inputs() {

}

pub fn read_graph_in_file() {

}

pub fn read_cities_in_file() {

}