use std::{collections::HashMap, hash::Hash};
use std::cmp::Ordering;

fn parse_input(input: &Vec<String>) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {

    let mut after_map = HashMap::new();
    let mut updates = Vec::new();
    let mut second_section = false;

    for line in input {
        if line == "" {
            second_section = true;
            continue;
        }

        if !second_section {
            let parts: Vec<&str> = line.split("|").collect();
            let before = parts[0].parse::<i64>().unwrap();
            let after = parts[1].parse::<i64>().unwrap();

            // check if the key exists
            if after_map.contains_key(&before) {
                let after_vec:&mut Vec<i64> = after_map.get_mut(&before).unwrap();
                after_vec.push(after);
            } else {
                after_map.insert(before, vec![after]);
            }

        } else {
            let parts: Vec<&str> = line.split(",").collect();
            let mut update = Vec::new();
            for part in parts {
                update.push(part.parse::<i64>().unwrap());
            }
            updates.push(update);
        }
    }

    return (after_map, updates);

}

fn check_order(update: Vec<i64>, after_map: HashMap<i64, Vec<i64>>) -> i64 {

    for i in 0..update.len() {
        let current = update[i];
        let after = update[i+1..].to_vec();
        let empty: Vec<i64> = vec![];
        let after_current = after_map.get(&current).unwrap_or(&empty);

        for a in after {
            if !after_current.contains(&a) {
                return 0;
            }
        }
    }

    // get the middle element of update
    let middle = update.len() / 2;
    let middle_val = update[middle];

    return middle_val;

}

fn cmp(a: &i64, b: &i64, after_map: &HashMap<i64, Vec<i64>>) -> Ordering {
    let empty_a = vec![];
    let empty_b = vec![];
    let a_after = after_map.get(a).unwrap_or(&empty_a);
    let b_after = after_map.get(b).unwrap_or(&empty_b);

    if a_after.contains(b) {
        return Ordering::Less;
    } else if b_after.contains(a) {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

fn order(mut unordered: Vec<i64>, after_map: HashMap<i64, Vec<i64>>) -> i64 {
    unordered.sort_by(|a, b| cmp(a, b, &after_map));

    // get the middle element of update
    let middle = unordered.len() / 2;
    let middle_val = unordered[middle];

    return middle_val;

}

pub fn part2(input: &Vec<String>) -> i64 {

    let (after_map, updates) = parse_input(input);

    let mut unordereds:Vec<Vec<i64>> = vec![];
    for update in updates {
        let res = check_order(update.clone(), after_map.clone());
        if res == 0 {
            unordereds.push(update);
        }
    }

    let mut sum = 0;
    for unordered in unordereds {
        sum += order(unordered, after_map.clone());
    }

    return sum;
}