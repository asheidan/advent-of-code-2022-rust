use std::io::BufRead;

fn solution_a(data: &[Vec<i32>]) -> i32 {
    0
}

fn solution_b(data: &[()]) -> i32 {
    0
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
                .collect::<Vec<i32>>()
        })
        .collect();

    eprintln!("running solutions...");
    println!("A: {}", solution_a(&data));
    //println!("B: {}", solution_b(&data));
}
