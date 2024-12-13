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

fn backtrack(machine: &((i64, i64), (i64, i64), (i64, i64)), cost: &(i64, i64), visited: &mut HashMap<(i64, i64), i64>) -> Vec<(i64, i64)> {

    if visited.contains_key(cost) {
        return vec![];
    }

    if (cost.0 > 100) || (cost.1 > 100) {
        return vec![];
    }

    if (cost.0 * machine.0.0 + cost.1 * machine.1.0) > machine.2.0 || (cost.0 * machine.0.1 + cost.1 * machine.1.1) > machine.2.1 {
        return vec![];
    }

    visited.insert(*cost, 1);

    if (cost.0 * machine.0.0 + cost.1 * machine.1.0) == machine.2.0 && (cost.0 * machine.0.1 + cost.1 * machine.1.1) == machine.2.1 {
        return vec![*cost];
    }

    let mut result = vec![];

    let mut new_cost = (cost.0 + 1, cost.1);
    result.extend(backtrack(machine, &new_cost, visited));

    new_cost = (cost.0, cost.1 + 1);
    result.extend(backtrack(machine, &new_cost, visited));

    return result;
} 

fn get_tokens(cost: (i64, i64)) -> i64 {
    return cost.0 * 3 + cost.1;
}

fn get_fewest_tokens(machine: &((i64, i64), (i64, i64), (i64, i64))) -> i64 {
    let mut visited = HashMap::new();

    let result = backtrack(machine, &(0, 0), &mut visited);
    let tokens = result.iter().map(|x| get_tokens(*x)).collect::<Vec<i64>>();
    return *tokens.iter().min().unwrap_or(&0);
}

pub fn part1(input: &Vec<String>) -> i64 {

    let machines = parse_input(input);

    let mut total_tokens = 0;

    for machine in machines {
        total_tokens += get_fewest_tokens(&machine);
    }

    return total_tokens;
}