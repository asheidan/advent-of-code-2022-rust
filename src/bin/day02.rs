use std::io::BufRead;

#[derive(Debug)]
struct OpponentShape {
    shape: char,
}

impl OpponentShape {
    fn score(&self) -> i32 {
        1 + (self.shape as i32 - ('A' as i32))
    }
}

#[derive(Debug)]
struct MyShape {
    shape: char,
}

impl MyShape {
    fn score(&self) -> i32 {
        1 + (self.shape as i32 - ('X' as i32))
    }

    fn win_score(&self, other: &OpponentShape) -> i32 {
        // -2 | 1 | 3 | Rock vs Scissor  | Win   | 6
        // -1 | 1 | 2 | Rock vs Paper    | Loose | 0
        //  0 | 1 | 1 | Rock vs Rock     | Draw  | 3
        //  1 | 2 | 1 | Paper vs Rock    | Win   | 6
        //  2 | 3 | 1 | Scissors vs Rock | Loose | 0
        //
        //  Shift it by 1 and modulus 3: (n + 1) % 3 to get the right "index".
        //  Then we can multiply by 3 to get the win score.
        //  Modulus for signed can be negative, so we shift an extra interval to make sure we
        //  always end up with a positive number.
        let n = self.score() - other.score();

        ((n + 4) % 3) * 3
    }
}

struct Shape {
    shape: char,
}
impl Shape {
    fn score(&self) -> i32 {
        1 + (self.shape as i32 - ('A' as i32))
    }
}

fn char_outcome_as_score(c: char) -> i32 {

    let result = (c as i32 - 'X' as i32) * 3;
    //eprintln!("outcome: {} -> {}", c, result);

    result
}

fn shape_score_for(outcome: char, opponent: char) -> i32 {
    // X | Loose | Rock -> Scissors
    //             Paper -> Rock
    //             Scissors -> Paper
    // Y | Draw  | Same
    // Z | Win   | Rock -> Paper
    //             Paper -> Scissors
    //             Scissors -> Rock
    let result = ((opponent as i32 - 'A' as i32 + 3) + (outcome as i32 - 'Y' as i32)) % 3 + 1;
    if result < 1 {
        eprintln!("shape: {}:{} -> {}", opponent, outcome, result);
    }

    result
}

fn solution_b(data: &[(char, char)]) -> i32 {
    data.iter()
        .map(|(opponent, outcome)| {
            char_outcome_as_score(*outcome) + shape_score_for(*outcome, *opponent)
        })
        .sum()
}

fn solution_a(data: &[(char, char)]) -> i32 {
    data.iter()
        .map(|(a, b)| (OpponentShape{ shape: *a }, MyShape{ shape: *b }))
        .map(|(opponent, my)| {
            my.score() + my.win_score(&opponent)
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

    println!("first: {:?}", data[0]);
    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod my_shape {
        use super::*;

        mod score {
            use super::MyShape;

            #[test]
            fn rock_should_have_score_1() {
                // Given
                let input = MyShape{ shape: 'X' };

                // When
                let result = input.score();

                // Then
                assert_eq!(1, result);
            }

            #[test]
            fn paper_should_have_score_2() {
                // Given
                let input = MyShape{ shape: 'Y' };

                // When
                let result = input.score();

                // Then
                assert_eq!(2, result);
            }

            #[test]
            fn scissors_should_have_score_3() {
                // Given
                let input = MyShape{ shape: 'Z' };

                // When
                let result = input.score();

                // Then
                assert_eq!(3, result);
            }
        }

        mod win_score {
            use super::*;

            #[test]
            fn rock_rock_should_have_score_3() {
                // Given
                let my = MyShape{ shape: 'X' };
                let other = OpponentShape{ shape: 'A' };

                // When
                let result = my.win_score(&other);

                // Then
                assert_eq!(3, result);
            }

            #[test]
            fn rock_paper_should_have_score_0() {
                // Given
                let my = MyShape{ shape: 'X' };
                let other = OpponentShape{ shape: 'B' };

                // When
                let result = my.win_score(&other);

                // Then
                assert_eq!(0, result);
            }

            #[test]
            fn rock_scissors_should_have_score_6() {
                // Given
                let my = MyShape{ shape: 'X' };
                let other = OpponentShape{ shape: 'C' };

                // When
                let result = my.win_score(&other);

                // Then
                assert_eq!(6, result);
            }
        }
    }
}
