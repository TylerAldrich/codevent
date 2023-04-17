pub fn solution_one(input: Vec<String>) {
    let mut result = 0;

    for pair in input {
        let ranges: Vec<&str> = pair.split(",").collect();
        let range1 = Range::new(ranges[0]);
        let range2 = Range::new(ranges[1]);

        if range1.fully_contains(&range2) || range2.fully_contains(&range1) {
            result += 1;
        }
    }

    println!("Solution: {}", result);
}

pub fn solution_two(input: Vec<String>) {
    let mut result = 0;

    for pair in input {
        let ranges: Vec<&str> = pair.split(",").collect();
        let range1 = Range::new(ranges[0]);
        let range2 = Range::new(ranges[1]);

        if range1.overlaps(&range2) || range2.overlaps(&range1) {
            result += 1;
        }
    }

    println!("Solution: {}", result);
}

struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn new(range: &str) -> Self {
        let range_split: Vec<&str> = range.split("-").collect();
        Range {
            low: range_split[0]
                .parse::<usize>()
                .expect("Invalid number in range"),
            high: range_split[1]
                .parse::<usize>()
                .expect("Invalid number in range"),
        }
    }

    fn fully_contains(&self, other_range: &Range) -> bool {
        other_range.low >= self.low && other_range.high <= self.high
    }

    fn overlaps(&self, other_range: &Range) -> bool {
        other_range.low >= self.low && other_range.low <= self.high
            || other_range.high >= self.low && other_range.high <= self.high
    }
}
