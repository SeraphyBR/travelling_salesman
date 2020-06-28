#![allow(dead_code)]

use crate::graph;
use crate::point::Point;
use crate::travel_route::TravelRoute;

use rand::Rng;
use rand::thread_rng;

type City = Point<i32>;
type Route = Vec<usize>;
type Cities = Vec<City>;
type Graph = graph::Graph<f32>;
type Population = Vec<TravelRoute>;

pub struct Genetic {
    mutation_rate: f32,
    tournament_size: usize,
    population_size: usize,
    generations: usize,
    elitism: bool,
}

impl Genetic {
    pub fn new() -> Genetic {
        Genetic {
            mutation_rate: 0.0,
            tournament_size: 0,
            population_size: 0,
            generations: 1,
            elitism: false,
        }
    }

    pub fn mutation_rate(&mut self, rate: f32) -> &mut Self {
        self.mutation_rate = rate;
        self
    }

    pub fn tournament_size(&mut self, size: usize) -> &mut Self {
        self.tournament_size = size;
        self
    }

    pub fn population_size(&mut self, size: usize) -> &mut Self {
        self.population_size = size;
        self
    }

    pub fn generations(&mut self, generations: usize) -> &mut Self {
        self.generations = generations;
        self
    }

    pub fn with_elitism(&mut self) -> &mut Self {
        self.elitism = true;
        self
    }


    pub fn evolve_population(&self, p: Population) -> Population {
        let mut next_gen = Population::with_capacity(p.len());
        let mut elitism_offset = 0;

        if self.elitism {
            next_gen[0] = self.get_best_route(p);
            elitism_offset = 1;
        }

        for i in elitism_offset..p.len() {
            let p1 = self.tournament_selection(p);
            let p2 = self.tournament_selection(p);
            let child = self.crossover(p1, p2);
            next_gen[i] = child;
        }

        for i in elitism_offset..next_gen.len() {
            self.mutate(next_gen.get_mut(i).unwrap());
        }

        next_gen
    }

    fn crossover(&self, a: TravelRoute, b: TravelRoute) -> TravelRoute {
        let mut child_cities = Cities::with_capacity(a.size());

        // Obtem uma posicao inicial e final de uma sub rota de 'a'
        let mut rng = thread_rng();
        let start_pos = rng.gen_range(1, a.size() - 1);
        let end_pos = rng.gen_range(1, a.size() - 1);

        for i in 0..child_cities.len() {
            if start_pos < end_pos && i > start_pos && i < end_pos {
                child_cities[i] = a.get_city(i);
            }
            else if start_pos > end_pos {
                if !(i < start_pos && i > end_pos) {
                    child_cities[i] = a.get_city(i);
                }
            }
        }

        for i in 0..b.size() {
             if !child_cities.contains(&b.get_city(i)) {
                 for j in 0..child_cities.len() {

                 }
             }
        }

        TravelRoute::with_cities(child_cities)
    }

    fn mutate(&self, tr: &mut TravelRoute) {
        let mut rng = thread_rng();
        for route_pos1 in 1..tr.size() {
            if rng.gen::<f32>() < self.mutation_rate {
                let route_pos2 = rng.gen_range(1, tr.size() - 1);
                tr.swap_cities(route_pos1, route_pos2);
            }
        }
    }

    fn tournament_selection(&self, p: Population) -> TravelRoute {

    }

    fn get_best_route(&self, p: Population) -> TravelRoute {
        let mut best = p[0];
        for route in p {
            if route.get_fitness() >= best.get_fitness() {
                best = route;
            }
        }
        best
    }
}