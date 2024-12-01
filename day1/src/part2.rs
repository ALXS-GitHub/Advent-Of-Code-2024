use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> (Vec<i64>, Vec<i64>) {

    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split("   ").collect();
        let left = parts[0];
        let right = parts[1];
        left_list.push(left.parse::<i64>().unwrap());
        right_list.push(right.parse::<i64>().unwrap());
    }

    return (left_list, right_list);
}

fn get_right_list_map(right_list: &Vec<i64>) -> HashMap<i64, i64> {
    let mut right_list_map: HashMap<i64, i64> = HashMap::new();
    for element in right_list.iter() {
        let count = right_list_map.entry(*element).or_insert(0);
        *count += 1;
    }
    return right_list_map;
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut similarity_score = 0;
    let (mut left_list, mut right_list) = parse_input(input);
    let right_list_map = get_right_list_map(&right_list);

    for element in left_list.iter() {
        let count = right_list_map.get(element).unwrap_or(&0);
        similarity_score += count * element;
    }

    return similarity_score;
}