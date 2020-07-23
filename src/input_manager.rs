#![allow(dead_code)]


use std::fs::File;
use std::io::{Error, BufReader, prelude::*};
use std::str::FromStr;

use rand::Rng;
use rand::thread_rng;
use num_traits::{pow, Float, Num, NumCast, cast};

use crate::graph::Graph;
use crate::point::Point;

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

pub fn read_graph_in_file<T: Num + NumCast + FromStr + Copy>(input_size: usize) -> Result<Graph<T>, Error> {
    let points = read_points_in_file(input_size)?;
    let mut graph = Graph::new(points.len());
    for p in points {
        graph.add_point(p);
    }

    Ok(graph)
}

pub fn read_points_in_file<T: Num + FromStr + Copy>(input_size: usize) -> Result<Vec<Point<T>>, Error> {
    let mut file = File::open(format!("inputs/vertices_{}.in", input_size))?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let values: Vec<T> = content.split_whitespace()
        .filter_map(|l| l.trim().parse().ok()).collect();

    let mut points = Vec::with_capacity(input_size);

    for i in 2..values.len() {
        points.push(Point::new(i - 2, (values[i - 1], values[i])));
    }

    Ok(points)
}