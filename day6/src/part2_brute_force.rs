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

fn walk_direction_check(input: &mut Vec<String>, i: i64, j: i64, dir: i64, known_dirs: &mut Vec<Vec<Vec<i64>>>, dirs: Vec<(i64, i64)>) -> (i64, i64) {
    let mut x = i;
    let mut y = j;
    let mut dx = dirs[dir as usize].0;
    let mut dy = dirs[dir as usize].1;
    let init_obj_x = x + dx;
    let init_obj_y = y + dy;
    loop {
        if x + dx < input.len() as i64 && x + dx >= 0 && y + dy < input[0].len() as i64 && y + dy >= 0 {
            if input[(x + dx) as usize].chars().nth((y + dy) as usize).unwrap() == '#' {
                return (x, y);
            }
            if known_dirs[(x + dx) as usize][(y + dy) as usize].contains(&dir) {
                return (-2, -2);
            }
            x += dx;
            y += dy;
            known_dirs[x as usize][y as usize].push(dir);
        } else {
            return (-1, -1);
        }
    }
}

fn come_to_known_dir(input: &mut Vec<String>, i: i64, j: i64, dir: i64, dirs: Vec<(i64, i64)>, known_dirs: &mut Vec<Vec<Vec<i64>>>, start_x: i64, start_y: i64, obs_positions: &mut Vec<(i64, i64)>) -> bool {
    let mut x = i;
    let mut y = j;
    let mut dir = dir;
    let mut dx = dirs[dir as usize].0;
    let mut dy = dirs[dir as usize].1;

    loop {
        let (nx, ny) = walk_direction_check(input, x, y, dir, known_dirs, dirs.clone());
        if nx == -1 {
            return false;
        }
        if nx == -2 {
            return true;
        }
        x = nx;
        y = ny;
        dir = (dir + 1) % 4;
    }

}

pub fn part2(input: &Vec<String>) -> i64 {

    let known_dirs:Vec<Vec<Vec<i64>>> = vec![vec![vec![]; input[0].len()]; input.len()];
    let mut input = input.clone();
    let mut obs_positions = vec![];
    let (start_x, start_y) = parse_input(&mut input.clone());
    let mut counter = 0;
    // let obs_count = walk(&mut input.clone(), start_x, start_y, start_x, start_y, &mut known_dirs.clone(), &mut obs_positions);

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            // println!("{}, {}", i, j);
            if i == start_x as usize && j == start_y as usize {
                continue;
            }
            if input[i].chars().nth(j).unwrap() != '#' {
                input[i].replace_range(j..j + 1, "#");
                let come_back = come_to_known_dir(&mut input.clone(), start_x as i64, start_y as i64, 0, vec![(-1, 0), (0, 1), (1, 0), (0, -1)], &mut known_dirs.clone(), start_x, start_y, &mut obs_positions);
                if come_back {
                    counter += 1;
                }
                input[i].replace_range(j..j + 1, ".");
            }
        }
    }

    println!("Counter: {}", counter);

    println!("Size of distinct obstacles: {}", obs_positions.len());

    return counter;
}