use std::collections::HashSet;

pub fn solution_one(input: Vec<String>) {
    let mut result = 0;

    for item in input {
        let (first, last) = item.split_at(item.len() / 2);
        let char = single_char_intersection(first, last);
        result += char_value(&char);
    }

    println!("Solution: {}", result);
}

pub fn solution_two(input: Vec<String>) {
    let result: u32 = input
        .chunks(3)
        .map(|input| {
            let (pack1, pack2, pack3) = (
                input[0].to_owned(),
                input[1].to_owned(),
                input[2].to_owned(),
            );

            let first_set = pack1.chars().collect::<HashSet<_>>();
            let second_set = pack2.chars().collect::<HashSet<_>>();
            let third_set = pack3.chars().collect::<HashSet<_>>();

            let first_intersection = first_set
                .intersection(&second_set)
                .cloned()
                .collect::<HashSet<_>>();
            let second_intersection = first_intersection.intersection(&third_set);
            let value = second_intersection
                .into_iter()
                .next()
                .expect("No char intersection exists!");
            char_value(value)
        })
        .sum();

    println!("Solution: {}", result)
}

fn single_char_intersection(first: &str, last: &str) -> char {
    let first_set = first.chars().collect::<HashSet<_>>();
    let last_set = last.chars().collect::<HashSet<_>>();

    let intersection = first_set.intersection(&last_set);
    intersection
        .into_iter()
        .next()
        .expect("No char intersection for sets")
        .to_owned()
}

fn char_value(char: &char) -> u32 {
    let mut result = char.to_digit(36).unwrap() - 9;
    if char.is_uppercase() {
        result += 26;
    }
    result
}
