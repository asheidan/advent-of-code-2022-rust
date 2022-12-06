use std::io::BufRead;

fn first_unique_in_window(window_size: usize, data: &[u32]) -> usize {
    data.windows(window_size)
        .map(|window| {
            window
                .iter()
                .fold(0x0, |a, &b| if 0 == (a & b) { a | b } else { 0xffffffff })
        })
        .enumerate()
        .find_map(|(index, bitset)| {
            if bitset != 0xffffffff {
                Some(index)
            } else {
                None
            }
        })
        .unwrap()
        + window_size
}

fn solve_a(bitsets: &[u32]) -> usize {
    first_unique_in_window(4, bitsets)
}
fn solve_b(bitsets: &[u32]) -> usize {
    first_unique_in_window(14, bitsets)
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<Vec<u32>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| 1 << ((c as u8) - ('a' as u8)))
                .collect()
        })
        .collect();

    data.iter().for_each(|s| {
        println!("A: {}", solve_a(s));
        println!("B: {}", solve_b(s));
    });
}
