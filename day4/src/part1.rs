fn get_xmas_vec() -> Vec<char> {
    return vec!['X', 'M', 'A', 'S'];
}

fn check_forward(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if j + 3 >= input[0].len() {
        return false;
    }
    for k in 0..4 {
        if input[i].chars().nth(j + k).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_backward(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if j < 3 {
        return false;
    }
    for k in 0..4 {
        if input[i].chars().nth(j - k).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_up(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if i < 3 {
        return false;
    }
    for k in 0..4 {
        if input[i - k].chars().nth(j).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_down(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if i + 3 >= input.len() {
        return false;
    }
    for k in 0..4 {
        if input[i + k].chars().nth(j).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_diag_up_forward(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if i < 3 || j + 3 >= input[0].len() {
        return false;
    }
    for k in 0..4 {
        if input[i - k].chars().nth(j + k).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_diag_up_backward(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if i < 3 || j < 3 {
        return false;
    }
    for k in 0..4 {
        if input[i - k].chars().nth(j - k).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_diag_down_forward(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if i + 3 >= input.len() || j + 3 >= input[0].len() {
        return false;
    }
    for k in 0..4 {
        if input[i + k].chars().nth(j + k).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn check_diag_down_backward(input: &Vec<String>, i: usize, j: usize) -> bool {
    let xmas_vec = get_xmas_vec();
    if i + 3 >= input.len() || j < 3 {
        return false;
    }
    for k in 0..4 {
        if input[i + k].chars().nth(j - k).unwrap() != xmas_vec[k] {
            return false;
        }
    }
    return true;  
}

fn find_xmas(input: &Vec<String>) -> i64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if check_forward(input, i, j) {
                count += 1;
            }
            if check_backward(input, i, j) {
                count += 1;
            }
            if check_up(input, i, j) {
                count += 1;
            }
            if check_down(input, i, j) {
                count += 1;
            }
            if check_diag_up_forward(input, i, j) {
                count += 1;
            }
            if check_diag_up_backward(input, i, j) {
                count += 1;
            }
            if check_diag_down_forward(input, i, j) {
                count += 1;
            }
            if check_diag_down_backward(input, i, j) {
                count += 1;
            }
        }
    }

    return count;
     
}

pub fn part1(input: &Vec<String>) -> i64 {
    return find_xmas(input);
}