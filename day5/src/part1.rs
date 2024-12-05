use std::{collections::HashMap, hash::Hash};

fn parse_input(input: &Vec<String>) -> (HashMap<i64, Vec<i64>>, HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {

    let mut after_map = HashMap::new();
    let mut before_map = HashMap::new();
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
            if before_map.contains_key(&after) {
                let before_vec:&mut Vec<i64> = before_map.get_mut(&after).unwrap();
                before_vec.push(before);
            } else {
                before_map.insert(after, vec![before]);
            }

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

    return (before_map, after_map, updates);

}

fn check_order(update: Vec<i64>, before_map: HashMap<i64, Vec<i64>>, after_map: HashMap<i64, Vec<i64>>) -> i64 {

    for i in 0..update.len() {
        let current = update[i];
        let before = update[..i].to_vec();
        let after = update[i+1..].to_vec();
        let before_current = before_map.get(&current).unwrap_or(&vec![]);
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

pub fn part1(input: &Vec<String>) -> i64 {
    let (before_map, after_map, updates) = parse_input(input);

    let mut sum = 0;
    for update in updates {
        sum += check_order(update, before_map.clone(), after_map.clone());
    }

    return sum;
}