### Advent of Code 2022+ 

A little CLI framework I made to run my [Advent of Code](https://adventofcode.com) solutions. All solutions are in the `solutions` folder, broken down by year/day, so avoid that if you're trying to avoid spoilers.

The solutions layout required to run properly is:
```
solutions/
    year_{YEAR}/
        day1/
            solution.rs
            input.txt  <-- The main puzzle input
            test_input.txt  <-- The test input used in the puzzle explanation
            test_solution.txt  <-- The test input solution used in the puzzle explanation
        day2/
        ... etc
```

`solution.rs` must contain two public functions:

```rust
pub fn solution_one(input: Vec<String>) {}
pub fn solution_two(input: Vec<String>) {}
```

Solutions can be run with:
```sh
cargo run -- -y YEAR -d DAY [-t]  # Include -t when you want to run using the  test input
cargo run --release -- -y YEAR -d DAY [-t]  # --release to optimize and remove debug symbols and such
```

Note that currently, solution files are compiled along with the main CLI, so there's not much point in installing and running the executable