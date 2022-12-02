use std::cmp::Reverse;
use std::io::BufRead;

use advent_of_code_2022_rust::split_by_none::SplitByNone;

fn solution_a(elf_data: &[i32]) -> i32 {
    elf_data[0]
}

fn solution_b(elf_data: &[i32]) -> i32 {
    elf_data[..3].iter().sum()
}

fn main() {
    let stdin = std::io::stdin();
    let mut data: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|l| match l.unwrap().parse::<i32>() {
            Ok(value) => Some(value),
            _ => None,
        })
        .split_by_none().into_iter()
        .map(|elf_load| elf_load.into_iter().sum())
        .collect();

    data.sort_by_key(|&value| Reverse(value));

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}
