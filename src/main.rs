#[macro_use]
extern crate log;
extern crate env_logger as logger;

mod literal;
mod dimacs_parser;
mod solver;

use std::{env, io::Read};
use std::fs::File;
use std::time::Instant;
use solver::{Stats, Solver};

use dimacs_parser::parse_dimacs;

fn print_stats(stats: Stats, start_time: Instant) {
    let time = Instant::now().duration_since(start_time);
    let time = time.as_secs_f64();
    println!("conflicts     : {} ", stats.conflicts);
    println!("decisions     : {} ", stats.decisions);
    println!("CPU time      : {:.3} sec", time);
}

fn main() {
    let start_time = Instant::now();

    env::set_var("RUST_LOG", "info");
    logger::init();

    let args: Vec<String> = env::args().collect();

    info!("input file: {}", &args[1]);

    let mut f = File::open(&args[1]).expect("file not found");
    let mut cnf_data = String::new();
    f.read_to_string(&mut cnf_data).expect("file reading error");

    let mut solver = Solver::new();
    let st = parse_dimacs(cnf_data.as_mut_str(), &mut solver);


    if !st {
        // format 由来のエラーを検出するようにする
        println!("UNSATISFIABLE (any file error)");
        return;
    }

    let st = solver.solve().unwrap();

    print_stats(solver.stats, start_time);

    if st {
        println!("SATISFIABLE");
        print!("Satisfying solution: ");
        for (i, assign) in solver.model.iter().enumerate() {
            let assign = match assign {
                Some(assign) => *assign,
                None => false,
            };
            let assign = if assign {
                1
            } else {
                0
            };
            print!("x{}={} ", i, assign);
        }
        println!("");

    } else {
        println!("UNSATISFIABLE");
    }
}
