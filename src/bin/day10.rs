use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl From<&String> for Instruction {
    fn from(line: &String) -> Self {
        let mut parts = line.split(" ");

        match parts.next() {
            Some("addx") => Self::AddX(parts.next().unwrap().parse::<i32>().unwrap()),
            _ => Self::Noop,
        }
    }
}

fn should_accumulate(cycle: i32) -> i32 {
    if cycle % 40 == 20 {
        eprintln!("------------accumulate!!!");
        cycle
    } else {
        0
    }
}

fn solve_a(data: &[Instruction]) -> i32 {
    let mut cycle = 0;
    let mut x = 1;

    let mut sum = 0;

    for instruction in data.iter() {
        eprintln!("-- {:?}", instruction);
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                sum += should_accumulate(cycle) * x;
            },
            Instruction::AddX(value) => {
                cycle += 1;
                sum += should_accumulate(cycle) * x;
                eprintln!("C:{:4} |", cycle);

                cycle += 1;
                sum += should_accumulate(cycle) * x;

                x += value;
            }
        };
        eprintln!("C:{:4} | X: {}", cycle, x);

    }

    sum
}

fn main() -> () {
    let stdin = std::io::stdin();
    let data: Vec<Instruction> = stdin
        .lock()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.unwrap();

            Instruction::from(&line)
        })
        .collect();

    println!("A: {}", solve_a(&data));
}
