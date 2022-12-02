use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Shape {
    fn score(&self) -> i32 {
        1 + (*self as i32)
    }
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        let mut input = c as u8 - ('A' as u8);

        if input > 2 {
            // Shift char so it always starts from 'A'
            // X -> A
            // Y -> B
            // Z -> C
            input -= 'X' as u8 - 'A' as u8;
        }

        match input  {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!("unknown char for from: {}", c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Loss = -1,
    Draw = 0,
    Win = 1,
}

impl Outcome {
    fn score(&self) -> i32 {
        (1 + (*self as i32)) * 3
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("unknown char: {} for Outcome", c),
        }
    }
}
impl From<(Shape, Shape)> for Outcome {
    fn from(shapes: (Shape, Shape)) -> Self {
        let (my, other) = shapes;

        // -2 | 0 | 2 | Rock vs Scissor  | Win
        // -1 | 0 | 1 | Rock vs Paper    | Loose
        //  0 | 0 | 0 | Rock vs Rock     | Draw
        //  1 | 1 | 0 | Paper vs Rock    | Win
        //  2 | 2 | 0 | Scissors vs Rock | Loose
        //
        //  Shift it by 1 and modulus 3: (n + 1) % 3 to get the right "index".
        //  Modulus for signed can be negative, so we shift an extra interval to make sure we
        //  always end up with a positive number.

        // This could easier to read with by matching shapes directly, but the implementation is a
        // leftover from the previous version. Why change that which isn't broken?
        match (my as i32 - other as i32 + 4) % 3 {
            0 => Self::Loss,
            1 => Self::Draw,
            2 => Self::Win,
            _ => panic!("math is broken: {:?}, {:?}", my, other),
        }
    }
}

fn my_shape_for_outcome(outcome: Outcome, opponent: Shape) -> Shape {
    // X | Loose | Rock -> Scissors
    //             Paper -> Rock
    //             Scissors -> Paper
    // Y | Draw  | Same
    // Z | Win   | Rock -> Paper
    //             Paper -> Scissors
    //             Scissors -> Rock
    let result = ((opponent as i32 + 3) + (outcome as i32)) % 3;
    match result {
        0 => Shape::Rock,
        1 => Shape::Paper,
        2 => Shape::Scissors,
        _ => panic!("wrong value for shape"),
    }
}

fn solution_b(data: &[(char, char)]) -> i32 {
    data.iter()
        .map(|(opponent, outcome)| (Shape::from(*opponent), Outcome::from(*outcome)))
        .map(|(opponent, outcome)| {
            outcome.score() + my_shape_for_outcome(outcome, opponent).score()
        })
        .sum()
}

fn solution_a(data: &[(char, char)]) -> i32 {
    data.iter()
        .map(|(a, b)| (Shape::from(*a), Shape::from(*b)))
        .map(|(opponent, my)| {
            Outcome::from((my, opponent)).score() + my.score()
        })
        .sum()
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<(char, char)> = stdin
        .lock()
        .lines()
        .map(|l| {
            let instructions: Vec<char> = l.unwrap().chars().collect();

            (instructions[0], instructions[2])
        })
        .collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod outcome {
        use super::*;

        mod from_shapes {
            use super::*;

            macro_rules! parameter_testcases {
                ($($name:ident: $input:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        let data = $input;

                        assert_eq!(data.1, Outcome::from(data.0), "Outcome::from({:?}) should result in {:?}", data.0, data.1);
                    }
                )*
                }
            }

            parameter_testcases! {
                rr_draw: ((Shape::Rock, Shape::Rock),         Outcome::Draw),
                rp_loss: ((Shape::Rock, Shape::Paper),        Outcome::Loss),
                rs_win:  ((Shape::Rock, Shape::Scissors),     Outcome::Win),
                pr_win:  ((Shape::Paper, Shape::Rock),        Outcome::Win),
                pp_draw: ((Shape::Paper, Shape::Paper),       Outcome::Draw),
                ps_loss: ((Shape::Paper, Shape::Scissors),    Outcome::Loss),
                sr_loss: ((Shape::Scissors, Shape::Rock),     Outcome::Loss),
                sp_win:  ((Shape::Scissors, Shape::Paper),    Outcome::Win),
                ss_draw: ((Shape::Scissors, Shape::Scissors), Outcome::Draw),
            }
        }
    }
}
