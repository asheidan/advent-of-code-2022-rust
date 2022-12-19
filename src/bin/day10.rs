use std::fmt;
use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Noop => f.write_fmt(format_args!("nop")),
            Instruction::AddX(v) => f.write_fmt(format_args!("add {}", v)),
        }
    }
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

struct Device<'i> {
    cycle_counter: i32,
    register_x: i32,

    instruction_cycle: i32,

    program_counter: i32,
    instructions: &'i [Instruction],
}

impl<'i> Device<'i> {
    fn new(instructions: &'i [Instruction]) -> Device<'i> {
        let program_counter = -1;

        Self {
            cycle_counter: 0,
            register_x: 1,

            instruction_cycle: 1,

            program_counter,
            instructions,
        }
    }

    fn cycles_for(instruction: &Instruction) -> i32 {
        match *instruction {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn current_instruction(&self) -> &'i Instruction {
        self.instructions
            .get(self.program_counter as usize)
            .unwrap()
    }
}

impl<'i> Iterator for Device<'i> {
    type Item = i32;

    /// Returns in the cycle
    ///
    /// Returns -v
    /// ____ ____ _
    ///  0  V   1  V
    ///     |      |
    ///
    fn next(&mut self) -> Option<i32> {
        let mut loaded_instruction = false;
        // End of previous cycle
        // 0. End cycle
        self.instruction_cycle -= 1;

        // 1. Perform changes from operation
        if self.instruction_cycle == 0 {
            match self.instructions.get(self.program_counter as usize) {
                Some(&Instruction::AddX(value)) => self.register_x += value,
                Some(&Instruction::Noop) => (),
                None => eprintln!("No instruction: pc: {}", self.program_counter),
            }
        }

        // Beginning of cycle
        self.cycle_counter += 1;

        if self.instruction_cycle == 0 {
            self.program_counter += 1;
            if self.program_counter as usize >= self.instructions.len() {
                return None;
            }

            self.instruction_cycle = Self::cycles_for(self.current_instruction());
            loaded_instruction = true;
        }

        if loaded_instruction {
            eprintln!(
                "pc:{:3} | x:{:3} | {}",
                self.cycle_counter,
                self.register_x,
                self.current_instruction());
        } else {
            eprintln!(
                "pc:{:3} | x:{:3} |",
                self.cycle_counter,
                self.register_x
                );
        }

        Some(self.cycle_counter)
    }
}

fn solve_b(data: &[Instruction]) -> () {
    let mut device = Device::new(data);

    let mut line = vec![' '; 40];

    let mut x: usize = 0;

    while let Some(cycle) = device.next() {
        if ((x as i32 - 1)..=(x as i32 + 1)).contains(&device.register_x) {
            line[x] = '#';
        }
        if cycle % 40 == 0 {
            println!("{}", line.iter().cloned().collect::<String>());

            line = vec![' '; 40];
            x = 0;
        }
        else {
            x += 1;
        }
    }
}

fn solve_a(data: &[Instruction]) -> i32 {
    let mut device = Device::new(data);
    let mut sum = 0;

    while let Some(cycle) = device.next() {
        if cycle % 40 == 20 {
            sum += cycle * device.register_x;
        }
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
    solve_b(&data);
}
