fn parse_input(input: &mut Vec<String>) -> (i64, i64) {
    for i in 0..input.len() {
        if input[i].contains("^") {
            let x = i as i64;
            let y = input[i].find("^").unwrap() as i64;
            input[i].replace_range(y as usize..y as usize + 1, "X");
            return (x, y);
        }
    }
    return (0, 0);
}

fn walk_direction(input: &mut Vec<String>, i: i64, j: i64, dx: i64, dy: i64) -> (i64, i64) {
    let mut x = i;
    let mut y = j;
    loop {
        if x + dx < input.len() as i64 && x + dx >= 0 && y + dy < input[0].len() as i64 && y + dy >= 0 {
            if input[(x + dx) as usize].chars().nth((y + dy) as usize).unwrap() == '#' {
                return (x, y);
            }
            x += dx;
            y += dy;
            input[x as usize].replace_range(y as usize..y as usize + 1, "X");
        } else {
            return (-1, -1);
        }
    }
}

fn walk(input: &mut Vec<String>, i: i64, j: i64) -> i64 {
    let mut x = i;
    let mut y = j;
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;
    while x != -1 {
        let (dx, dy) = dirs[dir];
        let (nx, ny) = walk_direction(input, x, y, dx, dy);
        if nx == -1 {
            break;
        }
        x = nx;
        y = ny;
        dir = (dir + 1) % 4;
    }
    return 0;
}

fn count_X(input: &Vec<String>) -> i64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i].chars().nth(j).unwrap() == 'X' {
                count += 1;
            }
        }
    }
    return count;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut input = input.clone();
    let (x, y) = parse_input(&mut input);
    walk(&mut input, x, y);
    return count_X(&input);
}