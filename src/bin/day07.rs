use std::collections::HashMap;
use std::io::Read;

fn solve_a(files: &[(String, usize)]) -> usize {
    // Map of directory path to size of files contained in the directory
    // This does not include the size of subdirectories
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut result = 0;

    for (path, size) in files.iter().rev() {
        //eprintln!("{} {}", path, size);
        if 0 != *size {
            if let Some((dir_path, _file)) = path.rsplit_once("/") {
                //eprintln!("{}", dir_path);
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

    result
}

fn solve_b(files: &[(String, usize)]) -> usize {
    let fs_limit = 70000000;
    let space_needed = 30000000;

    let used_space: usize = files.iter().map(|(_path, size)| size).sum();
    //eprintln!("used space:{:9}", used_space);

    let remove_space = space_needed - (fs_limit - used_space);
    //eprintln!("need space:{:9}", remove_space);

    let dir_sizes = directory_sizes(files);

    /*
    for (path, size) in &dir_sizes {
        if *size >= remove_space && (*size - remove_space) < 100000 {
            eprintln!("\x1b[33;1m{:9} : {:9} : {}\x1b[0m", size, size - remove_space, path);
        } else {
            eprintln!("{:9} : {:9} : {}", size, "", path);
        }
    }
    */

    let answer = dir_sizes.iter()
        .map(|(_p,s)| s.clone())
        .filter(|&size| size >= remove_space)
        .min()
        .unwrap();
    //eprintln!("remo space:{:9}", answer);

    answer
}

fn directory_sizes(files: &[(String, usize)]) -> Vec<(String, usize)> {

    let dir_sizes: Vec<(String, usize)> = files.iter().filter(|(_path, size)| *size == 0).map(|(dir_path, _size)| {
        let dir_size: usize = files.iter()
            .filter_map(|(file_path, size)| if file_path.starts_with(dir_path) { Some(*size) } else { None })
            .sum();

        (dir_path.clone(), dir_size)
    }).collect();

    dir_sizes
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
        match &command_line[..] {
            &["cd", ".."] => {
                dir_stack.pop();
            }

            &["cd", dir_name] => {
                dir_stack.push(dir_name.to_string());
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
            }
            _ => panic!("unknowwn command line: {}", command),
        };
    });

    files.sort();
    files.insert(0, (String::from(""), 0));

    /*
    for (path, size) in files.iter() {
        eprintln!("{:>9} : {}", size, path);
    }
    */
    println!("A: {}", solve_a(&files));
    println!("B: {}", solve_b(&files));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod directory_sizes {
        use super::*;

        #[test]
        fn should_sum_correctly_for_d() {
            // Given
            let input = vec![
                (String::from("/d/d.ext"), 5626152),
                (String::from("/d/d.log"), 8033020),
                (String::from("/d/j"), 4060174),
                (String::from("/d/k"), 7214296),
            ];

            // When
            let result = directory_sizes(&input);

            // Then
            let expected = vec![(String::from("/d"), 24933642)];
            assert_eq!(expected, result);
        }
    }
}

