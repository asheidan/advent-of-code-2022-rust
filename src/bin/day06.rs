use std::io::BufRead;

fn solve_a(data: &String) -> usize {
    let bitsets: Vec<u32> = data.chars()
        .map(|c| 1 << ((c as u8) - ('a' as u8)))
        .collect();

    bitsets.windows(4)
        .map(|window| window.iter().fold(0x0, |a, &b| if 0 == (a & b) { a | b } else { 0xffffffff }))
        .enumerate()
        .find_map(|(index, bitset)| if bitset != 0xffffffff { Some(index) } else { None })
        .unwrap() + 4
}
fn solve_b(data: &String) -> usize {
    let bitsets: Vec<u32> = data.chars()
        .map(|c| 1 << ((c as u8) - ('a' as u8)))
        .collect();

    bitsets.windows(14)
        .map(|window| window.iter().fold(0x0, |a, &b| if 0 == (a & b) { a | b } else { 0xffffffff }))
        .enumerate()
        .find_map(|(index, bitset)| if bitset != 0xffffffff { Some(index) } else { None })
        .unwrap() + 14
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    data.iter()
        .for_each(|s| {
            println!("A: {}", solve_a(s));
            println!("B: {}", solve_b(s));
        });
}

