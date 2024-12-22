use std::collections::HashSet;
use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> Vec<i64> {
    return input.iter().map(|x| x.parse::<i64>().unwrap()).collect();
}

fn mix(value: i64, secret_number: i64) -> i64 {
    return value ^ secret_number;
}

fn prune(secret_number: i64) -> i64 {
    return (secret_number + 16777216) % 16777216; // 2^24 = 16777216
}

fn calculate_next_secret_number(secret: &mut i64) -> i64 {
    // step 1
    let res = *secret * 64;
    *secret = mix(res, *secret);
    *secret = prune(*secret);

    // step 2
    let res = *secret / 32;
    *secret = mix(res, *secret);
    *secret = prune(*secret);

    // step 3
    let res = *secret * 2048;
    *secret = mix(res, *secret);
    *secret = prune(*secret);

    let last_digit = *secret % 10;
    return last_digit;
}

fn get_diff_seq(seq: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::new();
    for i in 0..seq.len() - 1 {
        diff.push(seq[i+1] - seq[i]);
    }
    return diff;
}

fn process_sequences(prices: &mut Vec<i64>, seen: &mut HashMap<Vec<i64>, i64>) -> i64 {

    let mut seq_seen = HashSet::new();

    for i in 0..prices.len() - 4 {
        let mut seq = prices[i..i+5].to_vec();
        let diff = get_diff_seq(&seq);
        if seq_seen.contains(&diff) {
            continue;
        }
        seq_seen.insert(diff.clone());
        seen.entry(diff.clone()).or_insert(0);

        let price = seen.get_mut(&diff).unwrap();
        *price += seq[seq.len() - 1];
    }

    return 0
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut secret_numbers = parse_input(input);
    
    let mut seen = HashMap::new();

    for i in 0..secret_numbers.len() {
        let mut secret = secret_numbers[i];
        let mut prices = Vec::new();
        prices.push(secret % 10);
        for _ in 0..2000 {
            let last_digit = calculate_next_secret_number(&mut secret);
            prices.push(last_digit);
        }
        process_sequences(&mut prices, &mut seen);
    }

    let max = seen.values().max().unwrap().clone();

    return max;
}