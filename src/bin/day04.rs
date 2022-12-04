//use std::cmp::Reverse;
use std::io::BufRead;
use std::ops::RangeInclusive;


fn range_contains(first: &RangeInclusive<i32>, other: &RangeInclusive<i32>) -> bool {
    other.clone().all(|n| first.contains(&n))
}

/// Full containment of one or the other
fn solution_a(data: &[Vec<i32>]) -> i32 {
    data.iter()
        .map(|assignments| match assignments[..] {
            [first_start, first_end, second_start, second_end] => {
                (first_start..=first_end, second_start..=second_end)
            }
            _ => panic!(""),
        })
        .filter(|(first, second)| range_contains(first, second) || range_contains(second, first))
        .count() as i32
}

/// Any overlap
fn solution_b(data: &[Vec<i32>]) -> i32 {
    data.iter()
        .map(|assignments| match assignments[..] {
            [first_start, first_end, second_start, second_end] => {
                (first_start..=first_end, second_start..=second_end)
            }
            _ => panic!(""),
        })
        .filter(|(first, second)| first.clone().any(|n| second.contains(&n)))
        .count() as i32
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<Vec<i32>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(|c: char| c == '-' || c == ',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    eprintln!("running solutions...");
    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}
