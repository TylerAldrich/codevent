mod parser;
mod solutions {
    pub mod year_2022;
}

use std::{fs::File, io::Read};

use clap::Parser;
use parser::parse_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year of solution to run
    #[arg(short, long, default_value_t = 1)]
    year: u16,

    /// Day of solution to run
    #[arg(short, long)]
    day: u8,

    /// Solution number [1 or 2]
    #[arg(short, long, default_value_t = 1)]
    solution: u8,

    /// Should this use the test input?
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    // println!(
    //     "Running year {} day {} - solution {} ({})",
    //     args.year, args.day, args.solution, args.test
    // );

    let func = get_solution_fn(args.day, args.solution);
    // TODO: This path should be relative from main.rs, so it doesnt need to include src/
    // Currently fails if you're not running from the root of the project
    let input_path = format!("src/solutions/year_{}/day{}/", args.year, args.day);
    if args.test {
        func(parse_file(input_path.clone() + "test_input.txt"));
        log_test_solution(input_path);
    } else {
        func(parse_file(input_path + "input.txt"));
    }
}

// TODO: Year is ignored. Fix in future years.
// TODO: Could probably write this as a proc macro instead of manually inputting everything
fn get_solution_fn(day: u8, solution: u8) -> impl Fn(Vec<String>) {
    match (day, solution) {
        (1, 1) => solutions::year_2022::day1::solution::solution_one,
        (1, 2) => solutions::year_2022::day1::solution::solution_two,
        _ => panic!("invalid day/solution! day must be between 1-25, solution must be 1-2"),
    }
}

fn log_test_solution(filename: String) {
    let filename = filename + "test_solution.txt";
    let mut file = File::open(filename).expect("test_solution.txt does not exist");

    let mut result = String::new();
    file.read_to_string(&mut result)
        .expect("Failed to read file to string");
    println!("Test solution is: {}", result);
}
