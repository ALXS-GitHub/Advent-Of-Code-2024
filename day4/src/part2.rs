fn get_mas_vec() -> Vec<char> {
    return vec!['M', 'A', 'S'];
}

// ! now we check from the middle of the word (A) and check the M and S

fn check_diag_up_forward(input: &Vec<String>, i: i64, j: i64) -> bool {
    let xmas_vec = get_mas_vec();
    if i < 1 || j + 1 >= input[0].len() as i64 {
        return false;
    }
    if i + 1 >= input.len() as i64 || j < 1 {
        return false;
    }
    for k in -1..2 {
        if input[(i - k) as usize].chars().nth((j + k) as usize).unwrap() != xmas_vec[(k + 1) as usize] {
            return false;
        }
    }
    return true;  
}

fn check_diag_up_backward(input: &Vec<String>, i: i64, j: i64) -> bool {
    let xmas_vec = get_mas_vec();
    if i < 1 || j < 1 {
        return false;
    }
    if i + 1 >= input.len() as i64 || j + 1 >= input[0].len() as i64 {
        return false;
    }
    for k in -1..2 {
        if input[(i - k) as usize].chars().nth((j - k) as usize).unwrap() != xmas_vec[(k + 1) as usize] {
            return false;
        }
    }
    return true;  
}

fn check_diag_down_forward(input: &Vec<String>, i: i64, j: i64) -> bool {
    let xmas_vec = get_mas_vec();
    if i + 1 >= input.len() as i64 || j + 1 >= input[0].len() as i64 {
        return false;
    }
    if i < 1 || j < 1 {
        return false;
    }
    for k in -1..2 {
        if input[(i + k) as usize].chars().nth((j + k) as usize).unwrap() != xmas_vec[(k + 1) as usize] {
            return false;
        }
    }
    return true;  
}

fn check_diag_down_backward(input: &Vec<String>, i: i64, j: i64) -> bool {
    let xmas_vec = get_mas_vec();
    if i + 1 >= input.len() as i64 || j < 1 {
        return false;
    }
    if i < 1 || j + 1 >= input[0].len() as i64 {
        return false;
    }
    for k in -1..2 {
        if input[(i + k) as usize].chars().nth((j - k) as usize).unwrap() != xmas_vec[(k + 1) as usize] {
            return false;
        }
    }
    return true;  
}

fn find_x_mas(input: &Vec<String>) -> i64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut valid_x = 0;
            if check_diag_up_forward(input, i as i64, j as i64) {
                valid_x += 1;
            }
            if check_diag_up_backward(input, i as i64, j as i64) {
                valid_x += 1;
            }
            if check_diag_down_forward(input, i as i64, j as i64) {
                valid_x += 1;
            }
            if check_diag_down_backward(input, i as i64, j as i64) {
                valid_x += 1;
            }
            if valid_x == 2 {
                count += 1;
            }
        }
    }

    return count;
     
}

pub fn part2(input: &Vec<String>) -> i64 {
    return find_x_mas(input);
}