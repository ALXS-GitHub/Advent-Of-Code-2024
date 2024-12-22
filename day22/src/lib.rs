use core::panic;
use std::io::{self, Read};
use std::fs::File;

pub mod part1;
pub mod part2;

pub fn read_input(path: &str) -> Vec<String> {
    let mut input = String::new();
    match File::open(path) {
        Ok(mut file) => {
            file.read_to_string(&mut input).unwrap();
        }
        Err(_) => {
            panic!("Failed to open file '{}'", path);
        }
    }
    let input = input.lines().map(|s| s.to_string()).collect();

    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_part1() {
        let correct_answer = 37327623;
        let input: Vec<String> = read_input("inputtest.txt");
        let answer = part1::part1(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }

    #[test]
    fn test_part1() {
        let correct_answer = 20068964552;
        let input: Vec<String> = read_input("input.txt");
        let answer = part1::part1(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }

    #[test]
    fn test_test_part2() {
        let correct_answer = 24;
        let input: Vec<String> = read_input("inputtest.txt");
        let answer = part2::part2(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }

    #[test]
    fn test_part2() {
        let correct_answer = 2246;
        let input: Vec<String> = read_input("input.txt");
        let answer = part2::part2(&input);
        assert_eq!(answer, correct_answer, "\x1b[31m\x1b[1mExpected {}, got {}\x1b[0m", correct_answer, answer);
    }
}