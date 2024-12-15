fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, Vec<i64>) {
    let mut map = Vec::new();
    let mut directions = Vec::new();

    let mut direction_part = false;

    for line in input {
        if line == "" {
            direction_part = true;
            continue;
        }

        if !direction_part {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            map.push(row);
        } else {
            for c in line.chars() {
                if c == '^' {
                    directions.push(0);
                } else if c == '>' {
                    directions.push(1);
                } else if c == 'v' {
                    directions.push(2);
                } else if c == '<' {
                    directions.push(3);
                }
            }
        }
    }



    return (map, directions);
}

fn find_start(map: &mut Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                map[y][x] = '.';
                return (x, y);
            }
        }
    }

    return (0, 0);
}

fn push_boxes(map: &mut Vec<Vec<char>>, x: usize, y: usize, direction: i64) -> bool {
    let (dx, dy) = match direction {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    };

    let mut new_x = x as isize;
    let mut new_y = y as isize;

    if map[y][x] == 'O' { // check that we are correctly on the O pos
        loop {
            new_x = new_x + dx;
            new_y = new_y + dy;

            if new_y >= map.len() as isize || new_x >= map[0].len() as isize || new_y < 0 || new_x < 0 { // should not happen
                return false;
            }

            if map[new_y as usize][new_x as usize] == '#' {
                return false;
            }

            if map[new_y as usize][new_x as usize] == 'O' { // skip if we are on another O
                continue;
            }

            if map[new_y as usize][new_x as usize] == '.' {
                map[new_y as usize][new_x as usize] = 'O';
                map[y][x] = '.';
                return true;
            }

            new_x = new_x;
            new_y = new_y;
        }
    }

    return false
}

fn move_direction(map: &mut Vec<Vec<char>>, x: usize, y: usize, direction: i64) -> (usize, usize) {
    let mut new_x = x;
    let mut new_y = y;

    if direction == 0 {
        new_y -= 1;
    } else if direction == 1 {
        new_x += 1;
    } else if direction == 2 {
        new_y += 1;
    } else if direction == 3 {
        new_x -= 1;
    }

    if new_y >= map.len() || new_x >= map[0].len() || new_y < 0 || new_x < 0 { // should not happen
        return (x, y);
    }

    if map[new_y][new_x] == '#' {
        return (x, y);
    }

    if map[new_y][new_x] == 'O' {
        let success = push_boxes(map, new_x, new_y, direction);
        if !success {
            return (x, y);
        } else {
            return (new_x, new_y);
        }
    }

    return (new_x, new_y);
}

fn get_result(map: &Vec<Vec<char>>) -> i64 {
    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                result += 100 * i as i64 + j as i64;
            }
        }
    }

    return result;
}

pub fn part1(input: &Vec<String>) -> i64 {


    let (mut map, directions) = parse_input(input);

    // println!("{:?}", map);

    // println!("{:?}", directions);

    let start = find_start(&mut map);
    // println!("{:?}", start);

    let mut x = start.0;
    let mut y = start.1;

    for direction in directions {
        let new_pos = move_direction(&mut map, x, y, direction);
        x = new_pos.0;
        y = new_pos.1;
    }


    // println!("{:?}", map);

    return get_result(&map);
}