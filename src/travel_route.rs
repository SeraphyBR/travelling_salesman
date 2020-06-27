#![allow(dead_code)]

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::graph;
use crate::point::Point;

type City = Point<i32>;
type Route = Vec<usize>;
type Cities = Vec<City>;
type Graph = graph::Graph<f32>;

pub struct TravelRoute {
    cities_dist: Graph,
    route: Route,
    cities: Cities,
    fitness: f32,
    distance: f32,
}

impl TravelRoute {
    pub fn empty() -> TravelRoute {
        TravelRoute {
            cities_dist: Graph::new(0),
            route: Route::new(),
            cities: Cities::new(),
            fitness: 0.0,
            distance: 0.0,
        }
    }

    pub fn with_cities(cities: Cities) -> TravelRoute {
        let mut travel_route = TravelRoute {
            cities_dist: Graph::new(cities.len()),
            route: Route::with_capacity(cities.len()),
            cities,
            fitness: 0.0,
            distance: 0.0
        };
        travel_route.inicialize();

        travel_route
    }

    fn inicialize(&mut self) {
        for i in 0..self.cities.len() {
            let city_a = &self.cities[i];
            self.route[i] = city_a.id();
            for j in 0..self.cities.len() {
                if i != j {
                    let city_b = &self.cities[j];
                    self.cities_dist.set_connection(i, j, city_a.dist_to(city_b));
                }
            }
        }
    }

    pub fn get_distance(&mut self) -> f32 {
        if self.distance == 0.0 {
            if !self.route.is_empty() {
                for i in 0..(self.route.len() - 1) {
                    let ri = self.route[i];
                    let rni = self.route[i + 1];
                    self.distance += self.cities_dist.get_connection(ri, rni);
                }
                let first = *self.route.first().unwrap_or(&0);
                let last = *self.route.last().unwrap_or(&0);

                self.distance += self.cities_dist.get_connection(first, last);
            }
        }
        self.distance
    }

    pub fn get_fitness(&mut self) -> f32 {
        if self.fitness == 0.0 {
            let distance = self.get_distance();
            if distance > 0.0 {
                self.fitness = 1.0 / distance;
            }
        }
        return self.fitness;
    }

    pub fn generate_individual(&mut self) {
        self.route.shuffle(&mut thread_rng());
        self.distance = 0.0;
        self.fitness = 0.0;
    }

    pub fn contains_city(&self, city: &City) -> bool {
        self.cities.contains(city)
    }

    pub fn get_city(&self, position: usize) -> City {
        self.cities[position].clone()
    }

    pub fn size(&self) -> usize {
        self.route.len()
    }

    pub fn swap_cities(&mut self, pos_a: usize, pos_b: usize) {
        self.route.swap(pos_a, pos_b);
        self.distance = 0.0;
        self.fitness = 0.0;
    }

    pub fn get_route(&self) -> Route {
        self.route.clone()
    }
}