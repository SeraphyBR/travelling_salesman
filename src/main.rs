mod input_manager;
mod point;
mod travel_route;
mod graph;
mod algorithms;
mod result;

use std::io;
use std::io::{Error, Write, Read};
use std::ops::{Range, RangeBounds};

use human_panic::setup_panic;

use crate::point::Point;

use algorithms::{
    algorithm::Algorithm,
    brute_force::BruteForce,
    branch_bound::BranchBound,
    dynamic::Dynamic,
    genetic::Genetic
};

fn main() {
    setup_panic!();
    loop {
        println!("Traveling Salesman Solver:");
        println!("1. Run with genereted inputs");
        println!("2. Run with manual inputs");
        println!("0. Exit\n");

        match read("OP: ").trim() {
            "0" => break,
            "1" => menu_generated(),
            "2" => menu_manual(),
            _ => continue,
        }
    }
}

fn menu_generated() {
    loop {
        println!("1. Generate new random inputs [1..100]");
        println!("2. Brute Force through inputs");
        println!("3. Branch and Bound through inputs");
        println!("4. Dynamic through inputs");
        println!("5. Genetic through inputs");
        println!("0. Exit\n");
        match read("OP: ").trim() {
            "0" => break,
            "1" => input_manager::gen_all_allowed_random_inputs::<i32,_>(1..=100),
            "2" => run_algorithm_gen_range::<BruteForce,_>(1..=100),
            "3" => run_algorithm_gen_range::<BranchBound,_>(1..=100),
            "4" => run_algorithm_gen_range::<Dynamic,_>(1..=100),
            "5" => run_algorithm_gen_range::<Genetic,_>(1..=100),
            _ => continue,
        }
    }
}

fn menu_manual() {
    loop {
        println!("1. Brute Force algorithm");
        println!("2. Branch and Bound algorithm");
        println!("3. Dynamic algorithm");
        println!("4. Genetic algorithm");
        println!("0. Exit\n");

        match read("OP: ").trim() {
            "0" => break,
            "1" => run_algorithm_stdin::<BruteForce>(),
            "2" => run_algorithm_stdin::<BranchBound>(),
            "3" => run_algorithm_stdin::<Dynamic>(),
            "4" => run_algorithm_stdin::<Genetic>(),
            _ => continue,
        }
    }
}

fn run_algorithm_stdin<T: Algorithm>() {
    println!("You can start typing, press Ctrl-D when finished");
    match input_manager::read_points_stdin() {
        Ok(input) => run_algorithm::<T>(input),
        Err(e) => {
            eprintln!("A entrada está incorreta");
        }
    };
}

fn run_algorithm_gen_range<T: Algorithm, I: Iterator<Item = usize>>(size_range: I) {
    for i in size_range {
        run_algorithm_gen::<T>(i);
    }
}

fn run_algorithm_gen<T: Algorithm>(input_size: usize) {
    match input_manager::read_points_gen(input_size) {
        Ok(input) => run_algorithm::<T>(input),
        Err(e) => {
            eprintln!("Algo de errado ocorreu, voce gerou os arquivos?");
        }
    }
}

fn run_algorithm<T: Algorithm>(input: Vec<Point<f32>>) {
    let mut algorithm = T::with_input(input);
    let result = algorithm.run();
    println!("{}", result);
}

fn read(msg: &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler do teclado!");
    input
} //Fim read()
