use std::collections::HashMap;

fn calculate_zone(
    input: &Vec<String>,
    visited: &mut HashMap<(usize, usize), char>,
    i: usize,
    j: usize,
    c: char,
) -> (i64, i64, HashMap<((isize, isize), (isize, isize)), char>) {
    let mut surface = 0;
    let mut perimeter = 0;
    let mut perimeters = HashMap::new();

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
            perimeters.insert(((i as isize, j as isize), (i as isize - 1, j as isize)), c);
        }
        if i == input.len() - 1 {
            perimeter += 1;
            perimeters.insert(((i as isize, j as isize), (i as isize + 1, j as isize)), c);
        }
        if j == 0 {
            perimeter += 1;
            perimeters.insert(((i as isize, j as isize), (i as isize, j as isize - 1)), c);
        }
        if j == input[i].len() - 1 {
            perimeter += 1;
            perimeters.insert(((i as isize, j as isize), (i as isize, j as isize + 1)), c);
        }

        if i > 0 {
            if let Some(visited_c) = visited.get(&(i - 1, j)) {
                if visited_c == &c {
                    queue.push((i - 1, j));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize - 1, j as isize)), c);
                }
            } else {
                let new_c = input[i - 1].chars().nth(j).unwrap();
                if new_c == c {
                    queue.push((i - 1, j));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize - 1, j as isize)), c);
                }
            }
        }
        if i < input.len() - 1 {
            if let Some(visited_c) = visited.get(&(i + 1, j)) {
                if visited_c == &c {
                    queue.push((i + 1, j));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize + 1, j as isize)), c);
                }
            } else {
                let new_c = input[i + 1].chars().nth(j).unwrap();
                if new_c == c {
                    queue.push((i + 1, j));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize + 1, j as isize)), c);
                }
            }
        }
        if j > 0 {
            if let Some(visited_c) = visited.get(&(i, j - 1)) {
                if visited_c == &c {
                    queue.push((i, j - 1));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize, j as isize - 1)), c);
                }
            } else {
                let new_c = input[i].chars().nth(j - 1).unwrap();
                if new_c == c {
                    queue.push((i, j - 1));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize, j as isize - 1)), c);
                }
            }
        }
        if j < input[i].len() - 1 {
            if let Some(visited_c) = visited.get(&(i, j + 1)) {
                if visited_c == &c {
                    queue.push((i, j + 1));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize, j as isize + 1)), c);
                }
            } else {
                let new_c = input[i].chars().nth(j + 1).unwrap();
                if new_c == c {
                    queue.push((i, j + 1));
                } else {
                    perimeter += 1;
                    perimeters.insert(((i as isize, j as isize), (i as isize, j as isize + 1)), c);
                }
            }
        }
    }

    (surface, perimeter, perimeters)
}

fn get_nb_contiguous_perimeters(
    perimeters: &mut HashMap<((isize, isize), (isize, isize)), char>,
) -> i64 {
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut nb_contiguous_perimeters = 0;
    let mut visited_perimeters: HashMap<((isize, isize), (isize, isize)), char> = HashMap::new();

    for (perimeter, _) in perimeters.iter() {
        let perimeter = perimeter.clone();
        let &c = perimeters.get(&perimeter).unwrap();
        if let Some(visited_perimeter) = visited_perimeters.get(&perimeter) {
            continue;
        }
        // println!("Perimeter: {:?}", perimeter);

        let normal_direction = (
            perimeter.1 .0 - perimeter.0 .0,
            perimeter.1 .1 - perimeter.0 .1,
        );
        let normal_direction_index = directions
            .iter()
            .position(|&x| x == normal_direction)
            .unwrap();
        let direction = directions[(normal_direction_index + 1) % 4];

        // println!("direction: {:?}", direction);

        // positive way

        let mut current_perimeter = perimeter;
        let mut current_direction = direction;

        while let Some(visited_perimeter) = perimeters.get(&current_perimeter) {
            visited_perimeters.insert(current_perimeter, c);
            current_perimeter = (
                (
                    current_perimeter.0 .0 + current_direction.0,
                    current_perimeter.0 .1 + current_direction.1,
                ),
                (
                    current_perimeter.1 .0 + current_direction.0,
                    current_perimeter.1 .1 + current_direction.1,
                ),
            );
        }

        // negative way

        let mut current_perimeter = perimeter;
        let mut current_direction = (-direction.0, -direction.1);

        while let Some(visited_perimeter) = perimeters.get(&current_perimeter) {
            visited_perimeters.insert(current_perimeter, c);
            current_perimeter = (
                (
                    current_perimeter.0 .0 + current_direction.0,
                    current_perimeter.0 .1 + current_direction.1,
                ),
                (
                    current_perimeter.1 .0 + current_direction.0,
                    current_perimeter.1 .1 + current_direction.1,
                ),
            );
        }

        nb_contiguous_perimeters += 1;
    }

    nb_contiguous_perimeters
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
            let (surface, perimeter, mut perimeters) = calculate_zone(input, &mut visited, i, j, c);
            // println!("{} : Perimeters: {:?}", c, perimeters);
            let nb_contiguous_perimeters = get_nb_contiguous_perimeters(&mut perimeters);
            // println!("{} : Nb contiguous perimeters: {}", c, nb_contiguous_perimeters);
            total += surface * nb_contiguous_perimeters;
        }
    }

    total
}
pub fn part2(input: &Vec<String>) -> i64 {
    let mut input = input.clone();
    let result = find_new_zone(&mut input);

    return result;
}
