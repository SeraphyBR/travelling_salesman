#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::{Error, Write, Read};
use std::str::FromStr;
use std::fmt::Display;

use rand::Rng;
use rand::thread_rng;
use rand::distributions::uniform::SampleUniform;
use num_traits::{Num, NumCast};

use crate::graph::Graph;
use crate::point::Point;

const MAX_XY: i32 = 1000;

fn gen_points<T: Num + NumCast + Default + Copy + SampleUniform>(size: usize) -> Vec<Point<T>> {
    let mut points = Vec::with_capacity(size);

    let max = T::from(MAX_XY).unwrap_or_default();

    let mut rng = thread_rng();
    loop {
        let x: T = rng.gen_range(T::one(), max);
        let y: T = rng.gen_range(T::one(), max);

        let new_p = Point::new(&points.len() + 1, (x, y));

        match points.iter().find(|p: &&Point<T>| p.coordinates() == new_p.coordinates()) {
            Some(_) => {},
            None => points.push(new_p)
        }

        if points.len() == size {
            break;
        }
    }
    points
}

fn gen_random_input<T: Num + NumCast + Default + Display + Copy + SampleUniform>(vertex_count: usize) -> Result<(), Error> {
    let points = gen_points::<T>(vertex_count);
    let mut file = File::create(format!("inputs/vertices_{}.in", vertex_count))?;

    file.write_all(format!("{}\n", vertex_count).as_bytes())?;
    for p in points {
        file.write_all(format!("{} {}\n", p.x(), p.y()).as_bytes())?;
    }
    Ok(())
}

pub fn gen_all_allowed_random_inputs<T: Num + NumCast + Default + Display + Copy + SampleUniform>(begin: usize, end: usize) {
    for i in begin..end {
        match gen_random_input::<T>(i) {
            Ok(()) => {},
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
}

pub fn read_graph<T: Num + NumCast + FromStr + Copy + Default>(input_size: usize, from_file: bool) -> Result<Graph<T>, Error> {
    let points = read_points(input_size, from_file)?;
    let mut graph = Graph::new(points.len());
    for p in points {
        graph.add_point(p);
    }

    Ok(graph)
}

pub fn read_points<T: Num + FromStr + Copy>(input_size: usize, from_file: bool) -> Result<Vec<Point<T>>, Error> {
    let mut input: Box<dyn Read> = if from_file {
        Box::new(File::open(format!("inputs/vertices_{}.in", input_size))?)
    } else {
        Box::new(io::stdin())
    };
    let mut content = String::new();

    input.read_to_string(&mut content)?;

    let values: Vec<T> = content.split_whitespace()
        .filter_map(|l| l.trim().parse().ok()).collect();

    let mut points = Vec::with_capacity(input_size);

    for i in 2..values.len() {
        points.push(Point::new(i - 2, (values[i - 1], values[i])));
    }

    Ok(points)
}