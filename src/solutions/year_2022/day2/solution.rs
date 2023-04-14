pub fn solution_one(input: Vec<String>) {
    let mut score = 0;
    for game in input {
        let slices: Vec<&str> = game.split(" ").collect();
        score += Game::new(slices[0], slices[1]).result();
    }

    println!("Solution: {}", score)
}

pub fn solution_two(input: Vec<String>) {
    let mut score = 0;
    for game in input {
        let slices: Vec<&str> = game.split(" ").collect();
        score += Game::new_2(slices[0], slices[1]).result();
    }

    println!("Solution: {}", score)
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct Game {
    them: Shape,
    us: Shape,
}

static WIN_POINTS: usize = 6;
static DRAW_POINTS: usize = 3;
static LOSE_POINTS: usize = 0;

impl Game {
    fn new(them: &str, us: &str) -> Self {
        let them = match them {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid move for them"),
        };

        let us = match us {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Invalid move for us"),
        };
        Game { them, us }
    }

    fn new_2(them: &str, us: &str) -> Self {
        let them = match them {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid move for them"),
        };

        let us = match us {
            "X" => {
                // Need to lose
                match them {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            }
            "Y" => {
                // Need to draw
                them
            }
            "Z" => {
                // Need to win
                match them {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            }
            _ => panic!("Invalid move for us"),
        };
        Game { them, us }
    }

    fn result(self) -> usize {
        let our_points = self.us.points();

        match (self.them, self.us) {
            (Shape::Rock, Shape::Paper) => our_points + WIN_POINTS,
            (Shape::Rock, Shape::Scissors) => our_points + LOSE_POINTS,
            (Shape::Paper, Shape::Scissors) => our_points + WIN_POINTS,
            (Shape::Paper, Shape::Rock) => our_points + LOSE_POINTS,
            (Shape::Scissors, Shape::Rock) => our_points + WIN_POINTS,
            (Shape::Scissors, Shape::Paper) => our_points + LOSE_POINTS,
            (_, _) => our_points + DRAW_POINTS,
        }
    }
}
