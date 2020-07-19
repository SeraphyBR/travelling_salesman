mod input_manager;
mod point;
mod travel_route;
mod graph;
mod algorithms;
mod result;

use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Traveling Salesman Solver:");
        println!("1. Run with genereted inputs");
        println!("2. Run with manual inputs");
        println!("0. Exit\n");

        match read("OP: ").trim() {
            "0" => break,
            "1" => menu1(),
            "2" => menu2(),
            _ => continue,
        }
    }
}

fn menu1() {
    loop {
        println!("1. Generate new random inputs [1..100]");
        println!("2. Brute Force through inputs");
        println!("3. Branch and Bound through inputs");
        println!("4. Dynamic through inputs");
        println!("5. Genetic through inputs");
        println!("6. Brute Force statistics");
        println!("7. Branch and Bound statistics");
        println!("8. Dynamic statistics");
        println!("9. Genetic statistics");
        println!("0. Exit\n");
        match read("OP: ").trim() {
            "0" => break,
            "1" => input_manager::gen_all_allowed_random_inputs(1, 100),
            "2" => {},
            "3" => {},
            "4" => {},
            "5" => {},
            "6" => {},
            "7" => {},
            "8" => {},
            "9" => {},
            _ => continue,
        }
    }
}

fn menu2() {
    loop {
        println!("1. Brute Force algorithm");
        println!("2. Branch and Bound algorithm");
        println!("3. Dynamic algorithm");
        println!("4. Genetic algorithm");
        println!("0. Exit\n");
        match read("OP: ").trim() {
            "0" => break,
            "1" => {},
            "2" => {},
            "3" => {},
            "4" => {},
            _ => continue,
        }
    }
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
