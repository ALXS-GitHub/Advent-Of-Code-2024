use std::collections::HashMap;
use std::collections::VecDeque;

fn parse_input(line: String) -> VecDeque<i64> {
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

fn split_number(n: i64) -> (i64, i64) {
    let digits = num_digits(n);
    let mid = digits / 2;

    let divisor = 10i64.pow(mid as u32);

    let first_part = n / divisor;
    let second_part = n % divisor;

    (first_part, second_part)
}

fn blink(number: i64, blinks_left: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(&result) = cache.get(&(number, blinks_left)) {
        return result;
    }
    if blinks_left == 0 {
        return 1;
    }
    let res = if number == 0 {
        blink(1, blinks_left - 1, cache)
    } else if num_digits(number) % 2 == 0 {
        let (first_part, second_part) = split_number(number);
        blink(first_part, blinks_left - 1, cache) + blink(second_part, blinks_left - 1, cache)
    } else {
        blink(number * 2024, blinks_left - 1, cache)
    };
    cache.insert((number, blinks_left), res);
    res
}


pub fn part2(input: &Vec<String>) -> i64 {
    let line = input[0].clone();
    let mut numbers = parse_input(line);
    let mut cache = HashMap::new();

    let mut res = 0;
    let blinks_left = 75;

    for &n in numbers.iter() {
        res += blink(n, blinks_left, &mut cache);
    }

    // print distinct numbers in the cache
    // println!("{:?}", cache.keys().map(|(x, _)| x).collect::<Vec<_>>().len());
    res
}