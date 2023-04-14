pub fn solution_one(input: Vec<String>) {
    let elves = elf_calories(input);

    let mut max = 0;
    for elf in elves {
        if elf > max {
            max = elf
        }
    }

    println!("Solution: {}", max)
}

pub fn solution_two(input: Vec<String>) {
    let mut elves = elf_calories(input);
    elves.sort_by(|a, b| b.cmp(a));

    if elves.len() < 3 {
        println!("Unable to compute answer - amount of elves is less than 3.");
    } else {
        println!("Solution: {}", elves[0] + elves[1] + elves[2]);
    }
}

fn elf_calories(input: Vec<String>) -> Vec<usize> {
    let mut elves: Vec<usize> = Vec::new();
    let mut total_calories = 0;
    for value in input {
        if value == "" {
            elves.push(total_calories);
            total_calories = 0;
        } else {
            let cals = value.parse::<usize>().expect("Invalid number found");
            total_calories += cals;
        }
    }
    elves.push(total_calories);
    elves
}
