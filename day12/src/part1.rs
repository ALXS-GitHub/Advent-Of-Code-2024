use std::collections::HashMap;

fn calculate_zone(
    input: &Vec<String>,
    visited: &mut HashMap<(usize, usize), char>,
    i: usize,
    j: usize,
    c: char,
) -> (i64, i64) {
    let mut surface = 0;
    let mut perimeter = 0;

    let mut queue = Vec::new();
    queue.push((i, j));

    while queue.len() > 0 {
        let (i, j) = queue.pop().unwrap();
        if i < 0 || i >= input.len() || j < 0 || j >= input[i].len() {
            continue;
        }

        if let Some(visited_c) = visited.get(&(i, j)) {
            continue;
        }

        let new_c = input[i].chars().nth(j).unwrap();
        if new_c != c {
            continue;
        }

        visited.insert((i, j), c);
        surface += 1;

        if i == 0 {
            perimeter += 1;
        }
        if i == input.len() - 1 {
            perimeter += 1;
        }
        if j == 0 {
            perimeter += 1;
        }
        if j == input[i].len() - 1 {
            perimeter += 1;
        }

        if i > 0 {
            if let Some(visited_c) = visited.get(&(i - 1, j)) {
                if visited_c == &c {
                    queue.push((i - 1, j));
                } else {
                    perimeter += 1;
                }
            } else {
                let new_c = input[i - 1].chars().nth(j).unwrap();
                if new_c == c {
                    queue.push((i - 1, j));
                } else {
                    perimeter += 1;
                }
            }
        }
        if i < input.len() - 1 {
            if let Some(visited_c) = visited.get(&(i + 1, j)) {
                if visited_c == &c {
                    queue.push((i + 1, j));
                } else {
                    perimeter += 1;
                }
            } else {
                let new_c = input[i + 1].chars().nth(j).unwrap();
                if new_c == c {
                    queue.push((i + 1, j));
                } else {
                    perimeter += 1;
                }
            }
        }
        if j > 0 {
            if let Some(visited_c) = visited.get(&(i, j - 1)) {
                if visited_c == &c {
                    queue.push((i, j - 1));
                } else {
                    perimeter += 1;
                }
            } else {
                let new_c = input[i].chars().nth(j - 1).unwrap();
                if new_c == c {
                    queue.push((i, j - 1));
                } else {
                    perimeter += 1;
                }
            }
        }
        if j < input[i].len() - 1 {
            if let Some(visited_c) = visited.get(&(i, j + 1)) {
                if visited_c == &c {
                    queue.push((i, j + 1));
                } else {
                    perimeter += 1;
                }
            } else {
                let new_c = input[i].chars().nth(j + 1).unwrap();
                if new_c == c {
                    queue.push((i, j + 1));
                } else {
                    perimeter += 1;
                }
            }
        }
    }

    (surface, perimeter)
}

fn find_new_zone(input: &mut Vec<String>) -> i64 {
    let mut total = 0;
    let mut visited = HashMap::new();

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i].chars().nth(j).unwrap();
            if let Some(visited_c) = visited.get(&(i, j)) {
                continue;
            }
            let (surface, perimeter) = calculate_zone(input, &mut visited, i, j, c);
            total += surface * perimeter;
        }
    }

    total
}

pub fn part1(input: &Vec<String>) -> i64 {
    let result = find_new_zone(&mut input.clone());

    return result;
}
