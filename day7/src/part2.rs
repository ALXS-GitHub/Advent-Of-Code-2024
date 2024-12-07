use rayon::prelude::*;

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

fn concat_numbers(a: i64, b: i64) -> i64 {
    let mut multiplier = 1;
    let mut temp = b;
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }
    a * multiplier + b
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
    let thrird_op = concat_numbers(eq[0], eq[1]);

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

    eq.remove(0);
    eq.remove(0);

    eq.insert(0, thrird_op);

    if is_equation_possible((res, eq)) {
        return true;
    }

    return false;

}

pub fn part2(input: &Vec<String>) -> i64 {
    let equations = parse_input(input);

    let sum: i64 = equations.par_iter().filter_map(|equation| {
        if is_equation_possible(equation.clone()) {
            return Some(equation.0);
        }
        return None;
    }).sum();

    return sum;
}