use std::collections::HashSet;
use std::io::BufRead;
use std::ops::AddAssign;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" | "u" => Self::Up,
            "D" | "d" => Self::Down,
            "L" | "l" => Self::Left,
            "R" | "r" => Self::Right,
            _ => panic!("unknown char for Direction: {}", c),
        }
    }
}

#[derive(Clone, Copy)]
struct Step {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn closer(&self, other: &Self) -> Self {
        let x = ((self.x + other.x * 2) as f32 / 3.0).round() as i32;
        let y = ((self.y + other.y * 2) as f32 / 3.0).round() as i32;
        Self { x , y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl AddAssign<Step> for Position {
    fn add_assign(&mut self, rhs: Step) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<Direction> for Step {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Step { x: 0, y: 1 },
            Direction::Down => Step { x: 0, y: -1 },
            Direction::Left => Step { x: -1, y: 0 },
            Direction::Right => Step { x: 1, y: 0 },
        }
    }
}

fn solve_b(data: &[(Direction, u8)]) -> i32 {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut knots: Vec<Position> = (0..10).map(|_| Position{x:0, y:0}).collect();

    visited.insert(knots[9]);

    for (direction, steps) in data.iter() {
        //eprintln!("{:2} {:?}", steps, direction);
        let step = Step::from(*direction);

        for _ in 0..*steps {
            knots[0] += step;

            (1..(knots.len()))
                .for_each(|index| {
                    let other = knots[index - 1];
                    let this = knots[index];

                    if ! this.is_touching(&other) {
                        //eprintln!("not touching!!! {} <> {}", this, other);
                        let new_pos = this.closer(&other);
                        //eprintln!("new: {}", new_pos);
                        knots[index].x = new_pos.x;
                        knots[index].y = new_pos.y;
                    }
                });

            /*
            for p in knots.iter() {
                eprint!("{},", p);
            }
            eprintln!();
            */

            visited.insert(knots[9]);
        }

    }

    return visited.len() as i32
}

fn solve_a(data: &[(Direction, u8)]) -> i32 {
    let mut visited: HashSet<Position> = HashSet::new();

    let mut head = Position { x: 0, y: 0 };
    let mut previous_head = head.clone();
    let mut tail = Position { x: 0, y: 0 };

    visited.insert(tail);

    for (direction, steps) in data.iter() {
        //eprintln!("{:2} {:?}", steps, direction);
        let step = Step::from(*direction);
        for _ in 0..*steps {
            head += step;
            //eprintln!("tail: {:?} | head: {:?}", tail, head);

            if ! tail.is_touching(&head) {
                // Tail need to move closer, cheat and use previous position of head
                tail = previous_head.clone();
                //eprintln!("tail moved to: {:?}", tail);
                visited.insert(tail);
            }

            previous_head = head.clone();
        }
    }

    visited.len() as i32
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<(Direction, u8)> = stdin
        .lock()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.unwrap();
            let mut parts = line.split(" ");
            let direction = Direction::from(parts.next().unwrap());
            let steps = parts.next().unwrap().parse::<u8>().unwrap();

            (direction, steps)
        })
        .collect();

    println!("A: {}", solve_a(&data));
    println!("B: {}", solve_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod position {
        use super::Position;

        mod is_touching {
            use super::Position;

            #[test]
            fn same_position_should_be_touching() {
                // Given
                let position = Position { x: 4, y: 4 };
                let other = Position { x: 4, y: 4 };

                // When
                let result = position.is_touching(&other);

                // Then
                assert_eq!(true, result);
            }

            #[test]
            fn one_left_position_should_be_touching() {
                // Given
                let position = Position { x: 5, y: 4 };
                let other = Position { x: 4, y: 4 };

                // When
                let result = position.is_touching(&other);

                // Then
                assert_eq!(true, result);
            }

            #[test]
            fn northeast_position_should_be_touching() {
                // Given
                let position = Position { x: 4, y: 4 };
                let other = Position { x: 5, y: 5 };

                // When
                let result = position.is_touching(&other);

                // Then
                assert_eq!(true, result);
            }

            #[test]
            fn northnortheast_position_should_not_be_touching() {
                // Given
                let position = Position { x: 4, y: 4 };
                let other = Position { x: 5, y: 6 };

                // When
                let result = position.is_touching(&other);

                // Then
                assert_eq!(false, result);
            }
        }

        mod closer {
            use super::Position;

            #[test]
            fn east_east_position_should_move_east() {
                // Given
                let position = Position { x: 4, y: 4 };
                let other = Position { x: 6, y: 4 };

                // When
                let result = position.closer(&other);

                // Then
                let expected = Position { x: 5, y: 4 };
                assert_eq!(expected, result);
            }


            #[test]
            fn north_north_position_should_move_north() {
                // Given
                let position = Position { x: 4, y: 4 };
                let other = Position { x: 4, y: 6 };

                // When
                let result = position.closer(&other);

                // Then
                let expected = Position { x: 4, y: 5 };
                assert_eq!(expected, result);
            }

            #[test]
            fn north_northeast_position_should_move_northeast() {
                // Given
                let position = Position { x: 4, y: 4 };
                let other = Position { x: 5, y: 6 };

                // When
                let result = position.closer(&other);

                // Then
                let expected = Position { x: 5, y: 5 };
                assert_eq!(expected, result);
            }
        }
    }
}

