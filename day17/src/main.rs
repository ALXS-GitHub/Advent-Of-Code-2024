use day17::{read_input, part1, part2};
use std::env;
use std::time::{Instant};

fn main() {

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let part1 = args.contains(&"--part1".to_string());
    let part2 = args.contains(&"--part2".to_string());

    let run_part1 = part1 || (!part1 && !part2);
    let run_part2 = part2 || (!part1 && !part2);

    // Start timer for reading input
    let now = Instant::now();

    // Read input
    let input: Vec<String> = read_input("inputtest.txt");

    // Stop timer for reading input
    let elapsed = now.elapsed();
    println!("\x1b[33m\x1b[1mTime taken to read input: {}.{:06} seconds\x1b[0m", elapsed.as_secs(), elapsed.subsec_micros());

    if run_part1 {
        // Start timer for part 1
        let now = Instant::now();
        let result = part1::part1(&input);
        // Stop timer for part 1
        let elapsed = now.elapsed();
        println!("\x1b[34m\x1b[1mPart 1 result: {}\x1b[0m", result);
        println!("\x1b[33m\x1b[1mTime taken for part 1: {}.{:06} seconds\x1b[0m", elapsed.as_secs(), elapsed.subsec_micros());
    }

    if run_part2 {
        // Start timer for part 2
        let now = Instant::now();
        let result = part2::part2(&input);
        // Stop timer for part 2
        let elapsed = now.elapsed();
        println!("\x1b[34m\x1b[1mPart 2 result: {}\x1b[0m", result);
        println!("\x1b[33m\x1b[1mTime taken for part 2: {}.{:06} seconds\x1b[0m", elapsed.as_secs(), elapsed.subsec_micros());
    }
}
