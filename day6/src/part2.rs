use std::collections::HashSet;

fn parse_input(input: &mut Vec<String>) -> (usize, usize) {
    for i in 0..input.len() {
        if input[i].contains("^") {
            let x = i;
            let y = input[i].find("^").unwrap();
            return (x, y);
        }
    }
    return (0, 0);
}

fn get_path(input: &Vec<String>, start_x: usize, start_y: usize, start_dir: usize, dirs: &[(isize, isize); 4]) -> Vec<(usize, usize, usize)> {
    let rows = input.len();
    let cols = input[0].len();
    let mut x = start_x as isize;
    let mut y = start_y as isize;
    let mut dir = start_dir;
    let mut path = vec![(x as usize, y as usize, dir)];

    loop {
        let (dx, dy) = dirs[dir];
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
            let c = input[nx as usize].chars().nth(ny as usize).unwrap();
            if c == '#' {
                dir = (dir + 1) % 4;
                path.push((x as usize, y as usize, dir));
            } else {
                x = nx;
                y = ny;
                path.push((x as usize, y as usize, dir));
            }
        } else {
            break;
        }
    }
    path
}

fn detect_loop(input: &Vec<String>, start_x: usize, start_y: usize, start_dir: usize, dirs: &[(isize, isize); 4]) -> bool {
    let rows = input.len();
    let cols = input[0].len();
    let mut x = start_x as isize;
    let mut y = start_y as isize;
    let mut dir = start_dir;
    let mut visited = HashSet::new();
    visited.insert((x, y, dir));

    loop {
        let (dx, dy) = dirs[dir];
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
            let c = input[nx as usize].chars().nth(ny as usize).unwrap();
            if c == '#' {
                dir = (dir + 1) % 4;
            } else {
                x = nx;
                y = ny;
            }
        } else {
            break;
        }
        if !visited.insert((x, y, dir)) {
            return true;
        }
        if x == start_x as isize && y == start_y as isize && dir == start_dir {
            return true;
        }
    }
    false
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut input = input.clone();
    let rows = input.len();
    let cols = input[0].len();

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let start_dir = 0;

    let (start_x, start_y) = parse_input(&mut input);

    let path = get_path(&input, start_x, start_y, start_dir, &dirs);

    let mut candidate_positions = HashSet::new();
    for &(x, y, dir) in &path {
        let (dx, dy) = dirs[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
            let c = input[nx as usize].chars().nth(ny as usize).unwrap();
            if c == '.' {
                candidate_positions.insert((nx as usize, ny as usize));
            }
        }
    }

    let mut loop_positions = Vec::new();
    for &(obs_x, obs_y) in &candidate_positions {
        let mut new_input = input.clone();
        // add obstacle
        new_input[obs_x].replace_range(obs_y..obs_y + 1, "#");

        let loop_detected = detect_loop(&new_input, start_x, start_y, start_dir, &dirs);
        if loop_detected {
            loop_positions.push((obs_x, obs_y));
        }
    }

    loop_positions.len() as i64
}