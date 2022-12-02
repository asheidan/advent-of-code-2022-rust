use std::io::BufRead;



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
}
