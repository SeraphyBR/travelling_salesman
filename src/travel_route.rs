#![allow(dead_code)]

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
}