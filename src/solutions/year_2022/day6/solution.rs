use std::collections::HashSet;

pub fn solution_one(input: Vec<String>) {
    let input = input.get(0).unwrap();
    let mut signilator = Signilator::new(input);

    while !signilator.check_signal() {
        signilator.next_signal();
    }

    println!("Solution: {}", signilator.get_value())
}

pub fn solution_two(input: Vec<String>) {
    let input = input.get(0).unwrap();
    let mut signilator = SignilatorV2::new(input);

    while !signilator.check_signal() {
        signilator.next_signal();
    }

    println!("Solution: {}", signilator.get_value())
}

struct Signilator<'a> {
    // Total chars looked at so far
    value: usize,
    // Last 4 chars - when all unique, we're happy
    last_chars: [char; 4],
    // Last index a char was inserted
    last_char_idx: usize,
    // Signal still needing to be processed
    signal: std::str::Chars<'a>,
}

impl<'a> Signilator<'a> {
    fn new(signal: &'a String) -> Self {
        let mut last_chars: [char; 4] = ['?'; 4];

        if signal.len() < 4 {
            panic!("Invalid signal length, less than 4 -- cannot init");
        }
        let mut signal = signal.chars();
        for i in 0..4 {
            last_chars[i] = signal.nth(0).unwrap();
        }

        Signilator {
            value: 4,
            last_chars,
            signal,
            last_char_idx: 0,
        }
    }

    fn check_signal(&self) -> bool {
        self.last_chars[0] != self.last_chars[1]
            && self.last_chars[0] != self.last_chars[2]
            && self.last_chars[0] != self.last_chars[3]
            && self.last_chars[1] != self.last_chars[2]
            && self.last_chars[1] != self.last_chars[3]
            && self.last_chars[2] != self.last_chars[3]
    }

    fn get_value(&self) -> usize {
        self.value
    }

    fn next_signal(&mut self) {
        self.value += 1;
        self.last_chars[self.last_char_idx] = self.signal.nth(0).unwrap();
        self.last_char_idx = (self.last_char_idx + 1) % 4;
    }
}

struct SignilatorV2<'a> {
    // Total chars looked at so far
    value: usize,
    // Last 14 chars - when all unique, we're happy
    last_chars: [char; 14],
    // Last index a char was inserted
    last_char_idx: usize,
    // Signal still needing to be processed
    signal: std::str::Chars<'a>,
}

impl<'a> SignilatorV2<'a> {
    fn new(signal: &'a String) -> Self {
        let mut last_chars: [char; 14] = ['?'; 14];

        if signal.len() < 14 {
            panic!("Invalid signal length, less than 14 -- cannot init");
        }
        let mut signal = signal.chars();
        for i in 0..14 {
            last_chars[i] = signal.nth(0).unwrap();
        }

        SignilatorV2 {
            value: 14,
            last_chars,
            signal,
            last_char_idx: 0,
        }
    }

    fn check_signal(&self) -> bool {
        let mut signal_set = HashSet::new();
        for i in 0..14 {
            if signal_set.contains(&self.last_chars[i]) {
                return false;
            }
            signal_set.insert(self.last_chars[i]);
        }
        true
    }

    fn get_value(&self) -> usize {
        self.value
    }

    fn next_signal(&mut self) {
        self.value += 1;
        self.last_chars[self.last_char_idx] = self.signal.nth(0).unwrap();
        self.last_char_idx = (self.last_char_idx + 1) % 14;
    }
}
