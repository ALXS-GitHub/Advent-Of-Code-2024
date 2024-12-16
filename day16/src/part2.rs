use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: (usize, usize),
    direction: (i64, i64),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut grid = Vec::new();

    for (y, line) in input.iter().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == 'S' {
                start = (x, y);
            } else if c == 'E' {
                end = (x, y);
            }
        }
        grid.push(row);
    }

    (grid, start, end)
}

fn is_intersection(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if grid[y][x] == '#' {
        return false;
    }

    let mut count = 0;
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
            return false;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if grid[ny][nx] == '.' {
            count += 1;
        }
        
    }

    return count >= 3;
}

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut dist = vec![vec![std::i64::MAX; grid[0].len()]; grid.len()];
    let directions = [(1i64, 0i64), (0i64, 1i64), (-1i64, 0i64), (0i64, -1i64)];
    let prev_direction = (1, 0);
    let mut predecessors: Vec<Vec<HashSet<(usize, usize)>>> = vec![vec![HashSet::new(); grid[0].len()]; grid.len()];

    let mut heap = BinaryHeap::new();
    dist[start.1][start.0] = 0;
    heap.push(State { cost: 0, position: start, direction: prev_direction });

    while let Some(State { cost, position: (x, y), direction: (pdx, pdy) }) = heap.pop() {

        for &(dx, dy) in &directions {

            if (dx, dy) == (-pdx, -pdy) {
                continue;
            }

            let mut score = 1;
            if (dx, dy) != (pdx, pdy) {
                score += 1000;
            }

            let nx = x as isize + dx as isize;
            let ny = y as isize + dy as isize;

            if nx < 0 || ny < 0 || nx >= grid[0].len() as isize || ny >= grid.len() as isize {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[ny][nx] == '#' {
                continue;
            }

            let next_cost = cost + score;

            if is_intersection(grid, nx, ny) {
                for (ddx, ddy) in directions {
                    let nnx = nx as isize + ddx as isize;
                    let nny = ny as isize + ddy as isize;

                    if nnx < 0 || nny < 0 || nnx >= grid[0].len() as isize || nny >= grid.len() as isize {
                        continue;
                    }

                    let mut nnext_cost = next_cost + 1;
                    if (ddx, ddy) == (-dx, -dy) {
                        continue
                    }
                    if (ddx, ddy) != (dx, dy) {
                        nnext_cost += 1000;
                    }

                    let nnx = nnx as usize;
                    let nny = nny as usize;

                    if grid[nny][nnx] == '#' {
                        continue;
                    }

                    if nnext_cost < dist[nny][nnx] {
                        dist[nny][nnx] = nnext_cost;
                        predecessors[ny][nx].clear();
                        predecessors[nny][nnx].clear();
                        predecessors[ny][nx].insert((x, y));
                        predecessors[nny][nnx].insert((nx, ny));
                        heap.push(State { cost: nnext_cost, position: (nnx, nny) , direction: (ddx, ddy) });
                    } else if nnext_cost == dist[nny][nnx] {
                        predecessors[ny][nx].insert((x, y));
                        predecessors[nny][nnx].insert((nx, ny));
                    }
                }
            } else {

                if next_cost < dist[ny][nx] {
                    dist[ny][nx] = next_cost;
                    predecessors[ny][nx].clear();
                    predecessors[ny][nx].insert((x, y));
                    heap.push(State { cost: next_cost, position: (nx, ny) , direction: (dx, dy) });
                } else if next_cost == dist[ny][nx] {
                    predecessors[ny][nx].insert((x, y));
                }
            }
        }
    }

    let mut paths = Vec::new();
    let mut path = Vec::new();
    dfs_paths(&predecessors, end, start, &mut path, &mut paths);
    paths
}

fn dfs_paths(
    predecessors: &Vec<Vec<HashSet<(usize, usize)>>>,
    current: (usize, usize),
    start: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    paths: &mut Vec<Vec<(usize, usize)>>,
) {
    path.push(current);
    if current == start {
        let mut complete_path = path.clone();
        complete_path.reverse();
        paths.push(complete_path);
    } else {
        for &prev in &predecessors[current.1][current.0] {
            dfs_paths(predecessors, prev, start, path, paths);
        }
    }
    path.pop();
}

fn count_paths_tiles(paths: &Vec<Vec<(usize, usize)>>) -> i64 {

    let mut visited = HashSet::new();
    let mut count = 0;
    for path in paths {
        for (x, y) in path {
            if !visited.contains(&(*x, *y)) {
                visited.insert((*x, *y));
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (grid, start, end) = parse_input(input);

    let paths = dijkstra(&grid, start, end);

    return count_paths_tiles(&paths);
}