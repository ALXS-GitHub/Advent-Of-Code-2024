fn parse_input(input: &Vec<String>, max_dim: usize, limit: usize) -> Vec<Vec<char>> {

    let mut grid = vec![vec!['.'; max_dim + 1]; max_dim + 1];
    
    for (i, line) in input.iter().enumerate() {

        if i == limit {
            break;
        }

        let coords = line.split(",").collect::<Vec<&str>>();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();

        grid[y][x] = '#';
    }

    return grid;

}

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i64 {

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut distance = vec![vec![std::i64::MAX; grid[0].len()]; grid.len()];

    let mut queue = Vec::new();
    queue.push(start);

    distance[start.1][start.0] = 0;

    while !queue.is_empty() {

        let current = queue.remove(0);

        if current == end {
            break;
        }

        let x = current.0;
        let y = current.1;

        if visited[y][x] {
            continue;
        }

        visited[y][x] = true;

        let mut neighbours = Vec::new();
        // making sure the is no overflow
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        neighbours.push((x + 1, y));
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        neighbours.push((x, y + 1));

        for neighbour in neighbours {

            let nx = neighbour.0;
            let ny = neighbour.1;

            if nx < 0 || nx >= grid[0].len() || ny < 0 || ny >= grid.len() {
                continue;
            }

            if grid[ny][nx] == '#' {
                continue;
            }

            let new_distance = distance[y][x] + 1;

            if new_distance < distance[ny][nx] {
                distance[ny][nx] = new_distance;
                queue.push(neighbour);
            }

        }

    }

    return distance[end.1][end.0];

}

pub fn part2(input: &Vec<String>) -> (i64, i64) {
    
    let mut max_dim = 70;
    let mut start_limit = 1024;

    if input.len() <= 25 { // for test input
        max_dim = 6;
        start_limit = 12;
    }

    let mut limit: usize = 0;

    let nb_lines = input.len();

    for i in start_limit..nb_lines {
        limit = i;
        let grid = parse_input(input, max_dim, limit);
        let start = (0, 0);
        let end = (max_dim, max_dim);
    
        let res = dijkstra(&grid, start, end);

        if res == std::i64::MAX {
            let coords = input[limit - 1].split(",").collect::<Vec<&str>>();
            return (coords[0].parse::<i64>().unwrap(), coords[1].parse::<i64>().unwrap());
        }
    }

    return (0, 0);

}