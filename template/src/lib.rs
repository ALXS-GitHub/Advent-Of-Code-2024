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