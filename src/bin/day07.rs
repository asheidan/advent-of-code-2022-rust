use std::collections::HashMap;
use std::io::{BufRead, Read};

struct Directory {
    subdirs: HashMap<String, Self>,
    files: Vec<usize>,
}

impl Directory {
    fn new() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: Vec::new(),
        }
    }
}

fn solve_a(dir: &Directory) -> i32 {
    0
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buffer: String = String::new();
    stdin
        .read_to_string(&mut buffer)
        .expect("Reading from stdin");

    let mut dir_stack = vec![Directory::new()];

    buffer.split("\n $").for_each(|command| {
        let mut iterator = command.split("\n");
        let command_line = iterator.next().unwrap().split(" ").collect::<Vec<&str>>();
        match &command_line[..] {
            &["cd", ".."] => {
                dir_stack.pop();
            }
            &["ls"] => {
                let mut current_dir = &dir_stack[dir_stack.len() - 1];
                for output_line in iterator {
                    let mut split_output = output_line.split(" ");
                    match (split_output.next(), split_output.next()) {
                        (Some("dir"), Some(name)) => {current_dir
                            .subdirs
                            .insert(name.to_string(), Directory::new());},
                        (Some(size_string), Some(file_name)) => current_dir
                            .files
                            .push(size_string.parse::<usize>().unwrap()),
                        _ => panic!("unknown ls output: {}", output_line),
                    };
                }
            }
            _ => panic!("unknowwn command line: {}", command),
        };
        eprintln!("----\n{}", command);
    });

    println!("A: {}", solve_a(&dir_stack[0]));
    //println!("B: {}", solve_b(&root));
}
