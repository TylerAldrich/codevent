mod parser;
mod solutions {
    pub mod year_2022;
}

use std::path::Path;

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
        log_test_solution(input_path, args.solution);
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

        (2, 1) => solutions::year_2022::day2::solution::solution_one,
        (2, 2) => solutions::year_2022::day2::solution::solution_two,

        (3, 1) => solutions::year_2022::day3::solution::solution_one,
        (3, 2) => solutions::year_2022::day3::solution::solution_two,

        (4, 1) => solutions::year_2022::day4::solution::solution_one,
        (4, 2) => solutions::year_2022::day4::solution::solution_two,

        (5, 1) => solutions::year_2022::day5::solution::solution_one,
        (5, 2) => solutions::year_2022::day5::solution::solution_two,

        (6, 1) => solutions::year_2022::day6::solution::solution_one,
        (6, 2) => solutions::year_2022::day6::solution::solution_two,

        (7, 1) => solutions::year_2022::day7::solution::solution_one,
        (7, 2) => solutions::year_2022::day7::solution::solution_two,

        (8, 1) => solutions::year_2022::day8::solution::solution_one,
        (8, 2) => solutions::year_2022::day8::solution::solution_two,

        (9, 1) => solutions::year_2022::day9::solution::solution_one,
        (9, 2) => solutions::year_2022::day9::solution::solution_two,

        (10, 1) => solutions::year_2022::day10::solution::solution_one,
        (10, 2) => solutions::year_2022::day10::solution::solution_two,
        _ => panic!("invalid day/solution! day must be between 1-25, solution must be 1-2"),
    }
}

fn log_test_solution(filename: String, solution: u8) {
    let filename = filename + "test_solution.txt";
    if Path::new(&filename).exists() {
        let test_solutions = parse_file(filename);
        if test_solutions.len() == 0 {
            // do nothing
        } else if test_solutions.len() == 1 && solution != 1 {
            // do nothing, no test data
        } else if test_solutions.len() >= 1 {
            println!(
                "Test solution is: {}",
                test_solutions.get((solution - 1) as usize).unwrap()
            );
        }
    }
}
