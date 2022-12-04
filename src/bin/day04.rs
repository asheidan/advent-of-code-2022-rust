//use std::cmp::Reverse;
use std::io::BufRead;
use std::ops::RangeInclusive;

trait ContainsAll<T> {
    fn contains_all(&self, other: T) -> bool;
}

impl ContainsAll<&RangeInclusive<i32>> for RangeInclusive<i32> {
    fn contains_all(&self, other: &RangeInclusive<i32>) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

trait Overlaps {
    fn overlaps(&self, other: &Self) -> bool;
}

impl Overlaps for RangeInclusive<i32> {
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start())
            || self.contains(other.end())
            || other.contains(self.start())
            || other.contains(self.end())
    }
}

/// Full containment of one or the other
fn solution_a(data: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    data.iter()
        .filter(|(first, second)| first.contains_all(second) || second.contains_all(first))
        .count() as i32
}

/// Any overlap
fn solution_b(data: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> i32 {
    data.iter()
        .filter(|(first, second)| first.overlaps(second))
        .count() as i32
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(|c: char| c == '-' || c == ',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|assignments: Vec<i32>| match assignments[..] {
            [first_start, first_end, second_start, second_end] => {
                (first_start..=first_end, second_start..=second_end)
            }
            _ => panic!(""),
        })
        .collect();

    eprintln!("running solutions...");
    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}
