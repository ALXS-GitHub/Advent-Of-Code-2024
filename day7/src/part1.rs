use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> Vec<(i64, Vec<i64>)> {

    let mut equations = Vec::new();

    for line in input {
        let mut parts = line.split(":");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();

        let mut right_parts = right.trim().split(" ");

        let res = left.parse::<i64>().unwrap();
        let mut eq = Vec::new();

        for part in right_parts {
            eq.push(part.parse::<i64>().unwrap());
        }

        equations.push((res, eq));
    }

    return equations;
}

fn is_equation_possible(equation: (i64, Vec<i64>)) -> bool {

    if equation.1.len() == 1 {
        return equation.0 == equation.1[0];
    }

    let res = equation.0;
    let mut eq = equation.1;

    let i = 0;
    let mut result = eq[i];
        
    let eq_copy = eq.clone();

    let first_op = eq[0] + eq[1];
    let second_op = eq[0] * eq[1];

    eq.remove(0);
    eq.remove(0);
    eq.insert(0, first_op);

    if is_equation_possible((res, eq)) {
        return true;
    }

    eq = eq_copy.clone();
    eq.remove(0);
    eq.remove(0);

    eq.insert(0, second_op);

    if is_equation_possible((res, eq)) {
        return true;
    }

    eq = eq_copy.clone();

    return false;

}

pub fn part1(input: &Vec<String>) -> i64 {

    let equations = parse_input(input);
    // println!("{:?}", equations);

    let mut sum = 0;

    for equation in equations {
        if is_equation_possible(equation.clone()) {
            sum += equation.0;
        }
    }

    return sum;
}