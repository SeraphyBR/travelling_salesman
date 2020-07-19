#![allow(dead_code)]


use std::fs::File;
use std::io::{Error, prelude::*};

use rand::Rng;
use rand::thread_rng;
use num_traits::{pow, Float, Num, NumCast, cast};

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;

type Graph = graph::Graph<f32>;

const MAX_XY: i32 = 1000;

fn gen_points(size: usize) -> Vec<Point<i32>> {
    let mut points = Vec::with_capacity(size);

    let mut rng = thread_rng();
    loop {
        let x: i32 = rng.gen_range(1, MAX_XY);
        let y: i32 = rng.gen_range(1, MAX_XY);

        let new_p = Point::new(&points.len() + 1, (x, y));

        match points.iter().find(|p: &&Point<i32>| p.coordinates() == new_p.coordinates()) {
            Some(_) => {},
            None => points.push(new_p)
        }

        if points.len() == size {
            break;
        }
    }
    points
}

fn gen_random_input(vertex_count: usize) -> Result<(), Error> {
    let points = gen_points(vertex_count);
    let mut file = File::create(format!("inputs/vertices_{}.in", vertex_count))?;

    file.write_all(format!("{}\n", vertex_count).as_bytes())?;
    for p in points {
        file.write_all(format!("{} {}\n", p.x(), p.y()).as_bytes())?;
    }
    Ok(())
}

pub fn gen_all_allowed_random_inputs(begin: usize, end: usize) {
    for i in begin..end {
        match gen_random_input(i) {
            Ok(()) => {},
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
}

pub fn read_graph_in_file() {

}

pub fn read_cities_in_file() {

}