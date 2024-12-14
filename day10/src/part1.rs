use std::collections::HashSet;

fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut result = Vec::new();

    for line in input {
        let mut line_result = Vec::new();
        for c in line.chars() {
            let v = c.to_digit(10).unwrap() as i64;
            line_result.push(v);
        }

        result.push(line_result);
    }

    return result;
}

fn calculate_score(input: &Vec<Vec<i64>>, i: i64, j: i64, last_height: i64, position_set: &mut HashSet<(i64, i64)>) {
    if input[i as usize][j as usize] == 9 {
        position_set.insert((i, j));
        return;
    }

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for direction in directions {
        let new_i = i + direction.0;
        let new_j = j + direction.1;

        if new_i >= 0 && new_i < input.len() as i64 && new_j >= 0 && new_j < input[0].len() as i64 {
            if input[new_i as usize][new_j as usize] == last_height + 1 {
                calculate_score(input, new_i, new_j, last_height + 1, position_set);
            }
        }
    }
}

fn get_starts(input: &Vec<Vec<i64>>) -> i64 {

    let mut total_score = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 0 {
                let mut position_set: HashSet<(i64, i64)> = HashSet::new();
                calculate_score(input, i as i64, j as i64, 0, &mut position_set);
                total_score += position_set.len() as i64;
            }
        }
    }

    return total_score;

}

pub fn part1(input: &Vec<String>) -> i64 {
    let input = parse_input(input);
    return get_starts(&input);
}