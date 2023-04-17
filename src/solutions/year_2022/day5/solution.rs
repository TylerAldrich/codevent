use std::collections::VecDeque;

pub fn solution_one(input: Vec<String>) {
    let mut stacks: Stacks;
    if input.get(0).unwrap().len() <= 12 {
        stacks = Stacks::new(true);
    } else {
        stacks = Stacks::new(false);
    }

    let mut instructions = Vec::new();
    for instruction_string in input {
        if !instruction_string.starts_with("move") {
            continue;
        }
        instructions.push(Instruction::new(instruction_string.split(' ').collect()));
    }

    stacks.run_9000(instructions);
    println!("{}", stacks.result());
}

pub fn solution_two(input: Vec<String>) {
    let mut stacks: Stacks;
    if input.get(0).unwrap().len() <= 12 {
        stacks = Stacks::new(true);
    } else {
        stacks = Stacks::new(false);
    }

    let mut instructions = Vec::new();
    for instruction_string in input {
        if !instruction_string.starts_with("move") {
            continue;
        }
        instructions.push(Instruction::new(instruction_string.split(' ').collect()));
    }

    stacks.run_9001(instructions);
    println!("{}", stacks.result());
}

// Note: The input stacks are hard-coded to make parsing easier
// Total line length of the input is used to figure out if
// the test or the real puzzle input is used
struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    fn new(test: bool) -> Self {
        if test {
            Stacks::new_test()
        } else {
            Stacks::new_real()
        }
    }

    /*
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3
    */
    fn new_test() -> Self {
        Stacks {
            stacks: vec![
                VecDeque::from(vec!['N', 'Z']),
                VecDeque::from(vec!['D', 'C', 'M']),
                VecDeque::from(vec!['P']),
            ],
        }
    }

    /*
    [V]         [T]         [J]
    [Q]         [M] [P]     [Q]     [J]
    [W] [B]     [N] [Q]     [C]     [T]
    [M] [C]     [F] [N]     [G] [W] [G]
    [B] [W] [J] [H] [L]     [R] [B] [C]
    [N] [R] [R] [W] [W] [W] [D] [N] [F]
    [Z] [Z] [Q] [S] [F] [P] [B] [Q] [L]
    [C] [H] [F] [Z] [G] [L] [V] [Z] [H]
    1   2   3   4   5   6   7   8   9
    */
    fn new_real() -> Self {
        Stacks {
            stacks: vec![
                VecDeque::from(vec!['V', 'Q', 'W', 'M', 'B', 'N', 'Z', 'C']),
                VecDeque::from(vec!['B', 'C', 'W', 'R', 'Z', 'H']),
                VecDeque::from(vec!['J', 'R', 'Q', 'F']),
                VecDeque::from(vec!['T', 'M', 'N', 'F', 'H', 'W', 'S', 'Z']),
                VecDeque::from(vec!['P', 'Q', 'N', 'L', 'W', 'F', 'G']),
                VecDeque::from(vec!['W', 'P', 'L']),
                VecDeque::from(vec!['J', 'Q', 'C', 'G', 'R', 'D', 'B', 'V']),
                VecDeque::from(vec!['W', 'B', 'N', 'Q', 'Z']),
                VecDeque::from(vec!['J', 'T', 'G', 'C', 'F', 'L', 'H']),
            ],
        }
    }

    // For solution 1, each box is moved 1 at a time from the from_idx to the to_idx
    fn run_9000(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            let from_vec = self.stacks.get_mut(instruction.from_idx).unwrap();
            let mut moving_stack = Vec::new();
            for _ in 0..instruction.amount {
                moving_stack.push(from_vec.pop_front().unwrap());
            }

            let to_vec = self.stacks.get_mut(instruction.to_idx).unwrap();
            for item in moving_stack {
                to_vec.push_front(item);
            }
        }
    }

    // For solution 2, each box is moved simultaneously from from_idx to to_idx as a pile
    fn run_9001(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            let from_vec = self.stacks.get_mut(instruction.from_idx).unwrap();

            // Logically this is all that needs to change between the two impls - instead of popping the
            // things to move into a normal vector, they are pushed to the front of a queue
            // so that they get added to the desired stack in reverse order
            let mut moving_stack = VecDeque::new();
            for _ in 0..instruction.amount {
                moving_stack.push_front(from_vec.pop_front().unwrap());
            }

            let to_vec = self.stacks.get_mut(instruction.to_idx).unwrap();
            for item in moving_stack {
                to_vec.push_front(item);
            }
        }
    }

    fn result(&mut self) -> String {
        let mut result_chars = Vec::new();
        for stack in self.stacks.iter_mut() {
            if let Some(ch) = stack.pop_front() {
                result_chars.push(ch);
            }
        }
        result_chars.iter().collect()
    }
}

struct Instruction {
    amount: usize,
    from_idx: usize,
    to_idx: usize,
}

impl Instruction {
    fn new(instruction: Vec<&str>) -> Self {
        let amount = instruction[1]
            .parse::<usize>()
            .expect("Invalid number in instruction");

        let from_idx = instruction[3]
            .parse::<usize>()
            .expect("Invalid number in instruction");

        let to_idx = instruction[5]
            .parse::<usize>()
            .expect("Invalid number in instruction");

        Instruction {
            amount,
            from_idx: from_idx - 1,
            to_idx: to_idx - 1,
        }
    }
}
