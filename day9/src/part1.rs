use std::cmp::min;


fn parse_input(input_line: String) -> (Vec<i64>, i64) {
    let mut total_space = 0;
    let mut spaces = Vec::new();
    let mut id_counter = 0;

    let chars: Vec<char> = input_line.chars().collect();

    for i in 0..chars.len() {
        if i % 2 == 1 {
            continue;
        }
        let value = chars[i].to_digit(10).unwrap() as i64;
        total_space += value;
        for _ in 0..value {
            spaces.push(id_counter);
        }
        id_counter += 1;
    }

    return (spaces, total_space);

}

fn fill_free_spaces(line: String, spaces: &mut Vec<i64>, total_space: i64) -> Vec<i64> {
    let mut chars: Vec<char> = line.chars().collect();
    let mut final_spaces = Vec::new();
    let mut id_counter: i64 = 0;

    for i in 0..chars.len() {

        if final_spaces.len() >= total_space as usize {
            break;
        }

        if i % 2 == 1 {
            
            let value = chars[i].to_digit(10).unwrap() as i64;
            let until_end = total_space - final_spaces.len() as i64;
            let moving = min(value, until_end);

            for _ in 0..moving {
                let pop = spaces.pop().unwrap();
                final_spaces.push(pop);
            }

        } else {
            let value = chars[i].to_digit(10).unwrap() as i64;
            for _ in 0..value {
                final_spaces.push(id_counter);
                if final_spaces.len() >= total_space as usize {
                    break;
                }
            }
            id_counter += 1;
        }
    }

    return final_spaces;
}

fn get_result(final_spaces: Vec<i64>) -> i64 {
    let mut result = 0;
    
    for i in 0..final_spaces.len() {
        result += final_spaces[i] * i as i64;
    }

    return result;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let line = input[0].clone();
    let (spaces, total_space) = parse_input(line.clone());
    let final_spaces = fill_free_spaces(line.clone(), &mut spaces.clone(), total_space);
    let result = get_result(final_spaces);

    return result;
}