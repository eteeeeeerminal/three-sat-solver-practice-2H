#[macro_use]
extern crate log;
extern crate env_logger as logger;

mod literal;
mod dimacs_parser;
mod solver;

use std::{env, io::Read};
use std::fs::File;

use crate::{dimacs_parser::parse_dimacs, solver::Solver};

fn main() {
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
        println!("UNSATISFIABLE");
    }

    let st = solver.solve().unwrap();
    if st {
        println!("SATISFIABLE");
    } else {
        println!("UNSATISFIABLE");
    }

    // 充足割当の出力
}
