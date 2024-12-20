use std::collections::HashSet;
use std::collections::HashMap;
// use std::hash::Hash;
use std::collections::VecDeque;

fn parse_input(input: &Vec<String>) -> ((usize, usize), (usize, usize), Vec<Vec<char>>) {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..input.len() {
        let mut row = Vec::new();
        for j in 0..input[0].len() {
            if input[i].chars().nth(j).unwrap() == 'S' {
                start = (i, j);
                row.push('.');
            } else if input[i].chars().nth(j).unwrap() == 'E' {
                end = (i, j);
                row.push('.');
            } else {
                row.push(input[i].chars().nth(j).unwrap());
            }
        }
        grid.push(row);
    }
    return (start, end, grid);
}

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<i64>> {

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut distance = vec![vec![std::i64::MAX; grid[0].len()]; grid.len()];

    let mut queue = Vec::new();
    queue.push(start);

    distance[start.0][start.1] = 0;

    while !queue.is_empty() {

        let current = queue.remove(0);

        if current == end {
            break;
        }

        let i = current.0;
        let j = current.1;

        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;

        let mut neighbours = Vec::new();
        // making sure the is no overflow
        if i > 0 {
            neighbours.push((i - 1, j));
        }
        neighbours.push((i + 1, j));
        if j > 0 {
            neighbours.push((i, j - 1));
        }
        neighbours.push((i, j + 1));

        for neighbour in neighbours {

            let ni = neighbour.0;
            let nj = neighbour.1;

            if ni < 0 || ni >= grid.len() || nj < 0 || nj >= grid[0].len() {
                continue;
            }

            if grid[ni][nj] == '#' {
                continue;
            }

            let new_distance = distance[i][j] + 1;

            if new_distance < distance[ni][nj] {
                distance[ni][nj] = new_distance;
                queue.push(neighbour);
            }

        }

    }

    return distance;
}

fn dfs_path(distance: Vec<Vec<i64>>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    // suppose that there is a unique path
    let mut path = Vec::new();

    let mut current = end;

    while current != start {

        path.push(current);

        let i = current.0;
        let j = current.1;

        let mut neighbours = Vec::new();
        // making sure the is no overflow
        if i > 0 {
            neighbours.push((i - 1, j));
        }
        neighbours.push((i + 1, j));
        if j > 0 {
            neighbours.push((i, j - 1));
        }
        neighbours.push((i, j + 1));

        for neighbour in neighbours {

            let ni = neighbour.0;
            let nj = neighbour.1;

            if ni < 0 || ni >= distance.len() || nj < 0 || nj >= distance[0].len() {
                continue;
            }

            if distance[ni][nj] == distance[i][j] - 1 {
                current = neighbour;
                break;
            }

        }

    }

    path.push(start);

    return path;
}

fn get_all_cheats_from_pos(grid: &Vec<Vec<char>>, path: Vec<(usize, usize)>, distance: Vec<Vec<i64>>, time_to_save: i64, start: (usize, usize), max_cheat_len:i64) -> HashSet<((usize, usize), (usize, usize))> {
    let mut cheat_set = HashSet::new();
    let mut i = 0;
    let mut current = start;
    // let mut visited = HashMap::new();
    while i < max_cheat_len {
        
        for radius in 2..=max_cheat_len {
            for di in 0..=radius {
                let dj = radius - di;
    
                for (ni, nj) in vec![
                    (start.0 as i64 + di, start.1 as i64 + dj),
                    (start.0 as i64 - di, start.1 as i64 + dj),
                    (start.0 as i64 + di, start.1 as i64 - dj),
                    (start.0 as i64 - di, start.1 as i64 - dj),
                ] {
                    if ni < 0 || ni >= grid.len() as i64 || nj < 0 || nj >= grid[0].len() as i64 {
                        continue;
                    }
    
                    let ni = ni as usize;
                    let nj = nj as usize;
    
                    if grid[ni][nj] == '#' {
                        continue;
                    }
    
                    let end = (ni, nj);
                    let end_distance = distance[end.0][end.1];
    
                    if distance[start.0][start.1] + time_to_save + radius <= end_distance {
                        cheat_set.insert((start, end));
                    }
                }
    
            }
        }

        i += 1;
    }
    return cheat_set;

}

fn look_for_cheats(grid: &Vec<Vec<char>>, path: Vec<(usize, usize)>, distance: Vec<Vec<i64>>, time_to_save: i64) -> i64 {
    let mut i = 0;
    let mut cheat_set = HashSet::new();
    let mut max_cheat_len = 20;
    for i in 0..path.len() {
        let current = path[i];
        let cheats = get_all_cheats_from_pos(grid, path.clone(), distance.clone(), time_to_save, current, max_cheat_len);
        cheat_set.extend(cheats);
    }

    return cheat_set.len() as i64;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut time_to_save = 100; // change this depending on the input

    let (start, end, grid) = parse_input(input);

    let mut distance_grid = dijkstra(&grid, start, end);

    let path = dfs_path(distance_grid.clone(), start, end);

    let cheats = look_for_cheats(&grid, path.clone(), distance_grid.clone(), time_to_save);

    return cheats;
}