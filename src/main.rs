mod input_manager;
mod point;
mod travel_route;
mod graph;
mod algorithms;

use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Traveling Salesman Solver:");
        println!("1. Gen random inputs [1..100]");
        println!("2. Brute Force through inputs[1..100]");
        println!("3. Branch and Bound through inputs[1..100]");
        println!("4. Dynamic through inputs[1..100]");
        println!("5. Genetic through inputs[1..100]");
        println!("6. Brute Force statistics[1..100]");
        println!("7. Branch and Bound statistics[1..100]");
        println!("8. Dynamic statistics[1..100]");
        println!("9. Genetic statistics[1..100]");
        println!("0. Exit\n");

        match read("OP: ").trim() {
            "0" => break,
            "1" => {},
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

fn read(msg: &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler do teclado!");
    input
} //Fim read()
