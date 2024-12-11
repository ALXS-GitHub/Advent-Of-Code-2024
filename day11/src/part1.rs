use std::collections::HashMap;

fn parse_input(line: String) -> Vec<i64> {
    return line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
}

fn num_digits(mut n: i64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    n = n.abs();
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

// fn split_number(n: i64) -> (i64, i64) {
//     let num_str = n.to_string();
//     let len = num_str.len();
//     assert!(len % 2 == 0, "Number must have an even number of digits");

//     let mid = len / 2;
//     let (first_half, second_half) = num_str.split_at(mid);
//     let first_part = first_half.parse::<i64>().unwrap();
//     let second_part = second_half.parse::<i64>().unwrap();

//     (first_part, second_part)
// }

fn split_number(n: i64) -> (i64, i64) {
    let digits = num_digits(n);
    let mid = digits / 2;

    let divisor = 10i64.pow(mid as u32);

    let first_part = n / divisor;
    let second_part = n % divisor;

    (first_part, second_part)
}

fn blink(numbers: &Vec<i64>, cache: &mut HashMap<i64, Vec<i64>>) -> Vec<i64> {
    
    let mut new_numbers = Vec::new();

    for i in 0..numbers.len() {

        if let Some(cached) = cache.get(&numbers[i]) {
            new_numbers.extend(cached);
            continue;
        }
        
        if numbers[i] == 0 {
            new_numbers.push(1);
        } 
        else if num_digits(numbers[i]) % 2 == 0 { // maybe opti here without using string
            let (first_part, second_part) = split_number(numbers[i]);
            new_numbers.push(first_part);
            new_numbers.push(second_part);
            cache.insert(numbers[i], vec![first_part, second_part]);
        }
        else {
            new_numbers.push(numbers[i] * 2024);
            cache.insert(numbers[i], vec![numbers[i] * 2024]);
        }

    }

    return new_numbers;

}

pub fn part1(input: &Vec<String>) -> i64 {
    let line = input[0].clone();
    let mut numbers = parse_input(line);
    let mut cache = HashMap::new();

    let blinks = 25; // change this for testing purposes

    for _ in 0..blinks {
        numbers = blink(&mut numbers, &mut cache);
    }

    // println!("{:?}", numbers);

    return numbers.len() as i64;
}