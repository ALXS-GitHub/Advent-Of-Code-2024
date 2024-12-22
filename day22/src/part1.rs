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


    return *secret;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut secret_numbers = parse_input(input);
    
    let mut total = 0;

    for i in 0..secret_numbers.len() {
        let mut secret = secret_numbers[i];
        for _ in 0..2000 {
            calculate_next_secret_number(&mut secret);
        }
        total += secret;
    }

    return total;
}