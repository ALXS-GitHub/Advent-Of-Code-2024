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

                if c == '@' {
                    row.push('@');
                    row.push('.');
                } else if c == '#' {
                    row.push('#');
                    row.push('#');
                } else if c == '.' {
                    row.push('.');
                    row.push('.');
                } else if c == 'O' {
                    row.push('[');
                    row.push(']');
                }
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

fn push_boxes_horizontal(map: &mut Vec<Vec<char>>, x: usize, y: usize, direction: i64) -> bool {
    let (dx, dy) = match direction {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    };

    assert!((dx == 1 && dy == 0) || (dx == -1 && dy == 0));

    let mut new_x = x as isize;
    let mut new_y = y as isize;

    if map[y][x] == '[' || map[y][x] == ']' { // check that we are correctly on the O pos
        loop {
            new_x = new_x + dx;
            new_y = new_y + dy;

            if new_y >= map.len() as isize || new_x >= map[0].len() as isize || new_y < 0 || new_x < 0 { // should not happen
                return false;
            }

            if map[new_y as usize][new_x as usize] == '#' {
                return false;
            }

            if map[new_y as usize][new_x as usize] == '[' || map[new_y as usize][new_x as usize] == ']' {
                continue;
            }

            if map[new_y as usize][new_x as usize] == '.' {
                
                let length = new_x - x as isize;

                if length < 0 {
                    map[y].copy_within(new_x as usize + 1..(x + 1), new_x as usize);
                } else {
                    map[y].copy_within(x..new_x as usize, x + 1);
                }

                map[y][x] = '.';
                return true;
            }

            new_x = new_x;
            new_y = new_y;
        }
    }

    return false
}

fn can_box_move_vertical(map: &Vec<Vec<char>>, x: usize, y: usize, direction: i64) -> bool {

    if map[y][x] == '#' {
        return false;
    }

    let (dx, dy) = match direction {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    };

    assert!((dx == 0 && dy == -1) || (dx == 0 && dy == 1));

    let left_pos: isize;
    let right_pos: isize;

    if map[y][x] == '[' {
        left_pos = x as isize;
        right_pos = x as isize + 1;
    } else {
        left_pos = x as isize - 1;
        right_pos = x as isize;
    }

    let new_y = y as isize + dy;
    
    if map[new_y as usize][left_pos as usize] == '#' || map[new_y as usize][right_pos as usize] == '#' {
        return false;
    }

    let mut temp_success = true;

    if map[new_y as usize][left_pos as usize] == '.' {
        temp_success = temp_success && true;
    } else if map[new_y as usize][left_pos as usize] == '[' || map[new_y as usize][left_pos as usize] == ']' {
        temp_success = temp_success && can_box_move_vertical(map, left_pos as usize, new_y as usize, direction);
    } else {
        temp_success = false;
    }

    if map[new_y as usize][right_pos as usize] == '.' {
        temp_success = temp_success && true;
    } else if map[new_y as usize][right_pos as usize] == '[' || map[new_y as usize][right_pos as usize] == ']' {
        temp_success = temp_success && can_box_move_vertical(map, right_pos as usize, new_y as usize, direction);
    } else {
        temp_success = false;
    }

    return temp_success;
}

fn move_box_vertical(map: &mut Vec<Vec<char>>, x: usize, y: usize, direction: i64) {
    let (dx, dy) = match direction {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    };

    assert!((dx == 0 && dy == -1) || (dx == 0 && dy == 1));

    let left_pos: isize;
    let right_pos: isize;

    if map[y][x] == '[' {
        left_pos = x as isize;
        right_pos = x as isize + 1;
    } else {
        left_pos = x as isize - 1;
        right_pos = x as isize;
    }

    if map[(y as isize + dy) as usize][left_pos as usize] == '.' {
        map[(y as isize + dy) as usize][left_pos as usize] = '[';
        map[y][left_pos as usize] = '.';
    } else if map[(y as isize + dy) as usize][left_pos as usize] != '#' {
        move_box_vertical(map, left_pos as usize, (y as isize + dy) as usize, direction);
        map[(y as isize + dy) as usize][left_pos as usize] = '[';
        map[y][left_pos as usize] = '.';
    }

    if map[(y as isize + dy) as usize][right_pos as usize] == '.' {
        map[(y as isize + dy) as usize][right_pos as usize] = ']';
        map[y][right_pos as usize] = '.';
    } else if map[(y as isize + dy) as usize][right_pos as usize] != '#' {
        move_box_vertical(map, right_pos as usize, (y as isize + dy) as usize, direction);
        map[(y as isize + dy) as usize][right_pos as usize] = ']';
        map[y][right_pos as usize] = '.';
    }

    
}

fn push_boxes_vertical(map: &mut Vec<Vec<char>>, x: usize, y: usize, direction: i64) -> bool {
    let (dx, dy) = match direction {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    };

    assert!((dx == 0 && dy == -1) || (dx == 0 && dy == 1));

    let new_x = x as isize + dx;
    let new_y = y as isize + dy;

    let success = can_box_move_vertical(map, x, y, direction);

    if !success {
        return false;
    }

    move_box_vertical(map, x as usize, y as usize, direction);

    return true;
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

    if map[new_y][new_x] == '[' || map[new_y][new_x] == ']' {
        let mut success:bool = false;

        if direction == 0 || direction == 2 {
            success = push_boxes_vertical(map, new_x, new_y, direction);
        } else {
            success = push_boxes_horizontal(map, new_x, new_y, direction);
        }

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
            if map[i][j] == '[' {
                result += 100 * i as i64 + j as i64;
            }
        }
    }

    return result;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (mut map, directions) = parse_input(input);

    let start = find_start(&mut map);

    let mut x = start.0;
    let mut y = start.1;

    for direction in directions {
        let new_pos = move_direction(&mut map, x, y, direction);
        x = new_pos.0;
        y = new_pos.1;
    }
    
    return get_result(&map);
}