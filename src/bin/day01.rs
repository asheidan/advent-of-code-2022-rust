use std::cmp::Reverse;
use std::io::BufRead;

fn solution_a(elf_data: &[Option<i32>]) -> i32 {
    let mut max_load: i32 = 0;
    let mut current_load: i32 = 0;

    elf_data.iter()
        .for_each(|maybe_load| match maybe_load {
            Some(load) => current_load += load,
            None => {
                max_load = i32::max(max_load, current_load);
                current_load = 0;
            },
        });

    max_load
}

fn split_by_none(elf_data: &[Option<i32>]) -> Vec<Vec<i32>> {
    elf_data.iter()
        .fold(vec![Vec::new()], |mut acc, maybe_value| match maybe_value {
        Some(value) => {
            acc.last_mut().unwrap().push(*value);
            acc
        },
        None => {
            acc.push(Vec::new());
            acc
        },
    })
}

fn solution_b(elf_data: &[Option<i32>]) -> i32 {
    let mut per_person: Vec<i32> = split_by_none(elf_data).into_iter()
        .map(|elf_load| elf_load.into_iter().sum()).collect();

    per_person.sort_by_key(|&num| Reverse(num));
    per_person[..3].iter().sum()
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<Option<i32>> = stdin
        .lock()
        .lines()
        .map(|l| match l.unwrap().parse() {
            Ok(value) => Some(value),
            _ => None,
        })
        //.map(|l| l.unwrap().split(","))
        .collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}
