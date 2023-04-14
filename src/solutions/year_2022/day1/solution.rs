pub fn solution_one(input: Vec<String>) {
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

    let mut max = 0;
    for elf in elves {
        if elf > max {
            max = elf
        }
    }

    println!("Solution: {}", max)
}

pub fn solution_two(input: Vec<String>) {
    println!("{:?}", input);
}
