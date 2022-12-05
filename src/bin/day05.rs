use std::io::{Read, BufRead, BufReader};

fn solution_a(stacks_: &[Vec<char>], instructions: &[Vec<usize>]) -> () {
    let mut stacks = stacks_.to_vec();

    instructions.iter()
        .for_each(|instruction| {
            for _ in 1..=instruction[0] {
                //eprintln!("move {} from {} to {}", instruction[0], instruction[1], instruction[2]);

                let moved_crate = stacks[instruction[1] - 1].pop().unwrap();
                stacks[instruction[2] - 1].push(moved_crate);
            }
    //eprintln!("{:?}", stacks);
        });

    let result: String = stacks.iter().map(|stack| stack[stack.len() - 1]).collect();
    println!("A: {}", result);
}

fn solution_b(stacks_: &[Vec<char>], instructions: &[Vec<usize>]) -> () {
    let mut stacks = stacks_.to_vec();

    instructions.iter()
        .for_each(|instruction| {
            //eprintln!("move {} from {} to {}", instruction[0], instruction[1], instruction[2]);

            let source_len_after = stacks[instruction[1] - 1].len() - instruction[0];
            let mut moved_crates = stacks[instruction[1] - 1].split_off(source_len_after);
            stacks[instruction[2] - 1].append(&mut moved_crates);
    //eprintln!("{:?}", stacks);
        });

    let result: String = stacks.iter().map(|stack| stack[stack.len() - 1]).collect();
    println!("B: {}", result);
}

fn main() {
    let mut stdin = std::io::stdin();

    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("Failed to read from stdin");

    let stack_and_instructions: Vec<_> = buffer.split("\n\n").collect();
    //eprintln!("{:?}", stack_and_instructions);
    let stack_description = stack_and_instructions[0];
    let instruction_block = stack_and_instructions[1];

    let mut stacks: Vec<Vec<char>> = Vec::new();

    stack_description.rsplit("\n")  // .skip(1)  // skip line of stack numbers
        .for_each(|line| {
            line.as_bytes()
                .chunks(4)
                .map(|maybe_crate| maybe_crate[1] as char)
                .enumerate()
                .filter(|(_i, c)| *c != ' ')
                .for_each(|(index, c)| match c {
                    '0'..='9' => stacks.push(Vec::new()),  // Should only be lowest row, so used
                                                           // for setup
                    'A'..='Z' => stacks[index].push(c),
                    _ => panic!("unknown instruction in stacks: '{}'", c),
                })
        });
    eprintln!("{:?}", stacks);

    let instructions: Vec<Vec<usize>> = instruction_block
        .split("\n")
        .filter(|line| ! line.is_empty())
        .map(|line| {
            line.split(|c: char| ! c.is_digit(10))
                .filter(|s| ! s.is_empty())
                .map(|s|  s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    //eprintln!("{:?}", instructions);



    eprintln!("running solutions...");
    solution_a(&stacks, &instructions);
    solution_b(&stacks, &instructions);
}
