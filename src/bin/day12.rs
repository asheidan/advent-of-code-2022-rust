// Day 12: Hill Climbing Algorithm
use std::io::BufRead;

struct Position {
    x: usize,
    y: usize,
}

fn solve_a(map: &[String]) -> i32 {
    let mut position = Position { x: 0, y: 0 };
    0
}

fn main() {
    let stdin = std::io::stdin();
    let map: Vec<String> = stdin
        .lock()
        .lines()
        .map(|maybe_line| maybe_line.unwrap())
        .collect();

    for line in map.iter() {
        eprintln!("{}", line);
    }

    println!("A: {}", solve_a(&map));
}
