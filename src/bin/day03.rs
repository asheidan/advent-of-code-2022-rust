use std::io::BufRead;

fn bitset_from_char(c: char) -> u64 {
    let mut offset = match c {
        'A'..='Z' => (c as i32) - ('A' as i32) + 26,
        'a'..='z' => (c as i32) - ('a' as i32),
        _ => panic!("invalid char for conversion: {}", c),
    };
    //eprintln!("offset: {}", offset);

    1 << offset
}

fn bitset_from_string(s: &str) -> u64 {
    s.chars()
        .map(bitset_from_char)
        .reduce(|a, b| a | b)
        .unwrap()
}

fn priority_from_bitset(bitset: u64) -> i32 {
    (bitset as f32).log2().abs() as i32 + 1
}

fn solution_a(data: &[String]) -> i32 {
    data.iter()
        .map(|s| {
            let (first_string, second_string) = s.trim().split_at(s.len() / 2);
            let (first, second) = (
                bitset_from_string(first_string),
                bitset_from_string(second_string),
            );

            priority_from_bitset(first & second)
        })
        .sum()
}

fn solution_b(data: &[String]) -> i32 {
    0
}

fn main() {
    let stdin = std::io::stdin();
    let data: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    println!("A: {}", solution_a(&data));
    println!("B: {}", solution_b(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bitset_from_char {
        use super::*;

        #[test]
        fn lowercase_a_should_be_1() {
            // When
            let result = bitset_from_char('a');

            // Then
            assert_eq!(1 << 0, result);
        }

        #[test]
        fn lowercase_z_should_be_shifted_25() {
            // When
            let result = bitset_from_char('z');

            // Then
            assert_eq!(1 << 25, result);
        }

        #[test]
        fn uppercase_a_should_be_shifted_26() {
            // When
            let result = bitset_from_char('A');

            // Then
            assert_eq!(1 << 26, result);
        }

        #[test]
        fn uppercase_Z_should_be_shifted_51() {
            // When
            let result = bitset_from_char('Z');

            // Then
            assert_eq!(1 << 51, result);
        }
    }

    mod bitset_from_string {
        use super::*;

        #[test]
        fn lowercase_aa_should_be_1() {
            // When
            let result = bitset_from_string("aa");

            // Then
            assert_eq!(1 << 0, result);
        }

        #[test]
        fn lowercase_ab_should_be_3() {
            // When
            let result = bitset_from_string("ab");

            // Then
            assert_eq!(0b11, result);
        }
    }
}
