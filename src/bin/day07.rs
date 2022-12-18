use std::collections::HashMap;
use std::io::Read;

fn solve_a(files: &[(String, usize)]) -> usize {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut result = 0;

    for (path, size) in files.iter().rev() {
        eprintln!("{} {}", path, size);
        if 0 != *size {
            if let Some((dir_path, _file)) = path.rsplit_once("/") {
                eprintln!("{}", dir_path);
                // Update entry in HashTable
                let entry = dir_sizes.entry(dir_path.to_string());
                *(entry.or_default()) += *size;
            }
        } else {
            let dir_size: usize = dir_sizes
                .iter()
                .filter_map(|(key, value)| {
                    if key.starts_with(path) {
                        Some(*value)
                    } else {
                        None
                    }
                })
                .sum();
            if dir_size < 100000 {
                result += dir_size;
            }
        }
    }

    eprintln!("{:?}", dir_sizes);

    result
}

fn path_from(dir_stack: &[String], file_name: &str) -> String {
    let mut path = String::from("/");
    if !dir_stack.is_empty() {
        path.push_str(&dir_stack.join("/"));
        path.push_str("/");
    }
    path.push_str(file_name);

    path
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut buffer: String = String::new();
    stdin
        .read_to_string(&mut buffer)
        .expect("Reading from stdin");

    //let mut root_dir = Directory::new();
    //let mut current_dir = &mut root_dir;
    let mut files: Vec<(String, usize)> = Vec::new();

    let mut dir_stack: Vec<String> = Vec::new();

    buffer.split("\n$ ").for_each(|command| {
        let mut iterator = command.split("\n").filter_map(|line| {
            let stripped_line = line.trim();
            if stripped_line.is_empty() {
                None
            } else {
                Some(stripped_line)
            }
        });
        let command_line = iterator.next().unwrap().split(" ").collect::<Vec<&str>>();
        //eprintln!("cmd: {:?}", command_line);
        match &command_line[..] {
            &["cd", ".."] => {
                dir_stack.pop();
                //current_dir = &mut root_dir;
                //current_dir = &mut root_dir;
                //dir_stack.iter().for_each(|dir_name| current_dir = current_dir.subdirs.get_mut(dir_name).unwrap());
                //dir_stack.iter().fold(&root_dir, |current_dir, dir_name| current_dir.subdirs.get(dir_name).unwrap());
            }

            &["cd", dir_name] => {
                dir_stack.push(dir_name.to_string());
                //current_dir = current_dir.subdirs.get_mut(&dir_name.to_string()).unwrap();
            }

            &["ls"] => {
                for output_line in iterator {
                    let mut split_output = output_line.split(" ");
                    match (split_output.next(), split_output.next()) {
                        (Some("dir"), Some(dir_name)) => {
                            let path = path_from(&dir_stack, dir_name);
                            files.push((path, 0));
                        }
                        (Some(size_string), Some(file_name)) => {
                            //current_dir .files .push(size_string.parse::<usize>().unwrap());
                            let path = path_from(&dir_stack, file_name);
                            files.push((path, size_string.parse::<usize>().unwrap()));
                        }
                        _ => panic!("unknown ls output: '{}'", output_line),
                    };
                }
            }

            &["$", "cd", "/"] => {
                while 0 < dir_stack.len() {
                    dir_stack.pop();
                }
                //current_dir = &mut root_dir;
            }
            _ => panic!("unknowwn command line: {}", command),
        };
        //eprintln!("----\n{}", command);
    });

    files.sort();

    for (path, size) in files.iter() {
        eprintln!("{:>9} : {}", size, path);
    }
    println!("A: {}", solve_a(&files));
    //println!("B: {}", solve_b(&root));
}
