use regex::Regex;
use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    
    let mut machines = Vec::new();
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut p = (0, 0);

    for line in input {
        if line.is_empty() {
            a = (0, 0);
            b = (0, 0);
            p = (0, 0);
        }

        if line.starts_with("Button A") {
            let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            a = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        } else if line.starts_with("Button B") {
            let re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            b = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        } else if line.starts_with("Prize") {
            let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            p = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
            machines.push((a, b, p));
        }

    }

    return machines;

}

fn find_combination_2d(machine: &((i64, i64), (i64, i64), (i64, i64))) -> Vec<(i64, i64)> {

    // Système de Cramer
    // (x1, x2) (n1) = (x)
    // (y1, y2) (n2) = (y)

    let (x1, y1) = machine.0;
    let (x2, y2) = machine.1;
    let (x, y) = machine.2;

    let d = x1 * y2 - x2 * y1;
    let n1 = x * y2 - y * x2;
    let n2 = x1 * y - y1 * x;

    if n1 % d != 0 || n2 % d != 0 {
        return vec![];
    }

    let n1 = n1 / d;
    let n2 = n2 / d;

    if n1 < 0 || n2 < 0 {
        return vec![];
    }
    
    return vec![(n1, n2)];
}

fn get_tokens(cost: (i64, i64)) -> i64 {
    return cost.0 * 3 + cost.1;
}

fn get_fewest_tokens(machine: &((i64, i64), (i64, i64), (i64, i64))) -> i64 {
    let result = find_combination_2d(machine);
    if result.len() == 0 {
        return 0;
    }
    return get_tokens(result[0]);
}

pub fn part1(input: &Vec<String>) -> i64 {

    let machines = parse_input(input);

    let mut total_tokens = 0;

    for machine in machines {
        total_tokens += get_fewest_tokens(&machine);
    }

    return total_tokens;
}