use std::collections::BinaryHeap;
use std::cmp::Ordering;

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

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut dist = vec![vec![std::i64::MAX; grid[0].len()]; grid.len()];
    let directions = [(1i64, 0i64), (0i64, 1i64), (-1i64, 0i64), (0i64, -1i64)];
    let prev_direction = (1, 0);

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

            if next_cost < dist[ny][nx] {
                if (dx, dy) != (pdx, pdy) && x != end.0 && y != end.1 {
                    dist[y][x] += 1000;
                } 
                dist[ny][nx] = next_cost;
                heap.push(State { cost: next_cost, position: (nx, ny) , direction: (dx, dy) });
            }
        }
    }

    return dist[end.1][end.0]
}

pub fn part1(input: &Vec<String>) -> i64 {

    let (grid, start, end) = parse_input(input);

    return dijkstra(&grid, start, end);

}