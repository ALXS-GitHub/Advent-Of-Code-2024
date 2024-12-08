use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> HashMap<char, Vec<(i64, i64)>> {
    let mut map = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i].chars().nth(j).unwrap();
            if c != '.' {
                let mut v = map.entry(c).or_insert(Vec::new());
                v.push((i as i64, j as i64));
            }
        }
    }
    return map;
}

fn calculate_antinodes(input: &Vec<String>, map: &HashMap<char, Vec<(i64, i64)>>) -> Vec<Vec<char>> {
    let mut antinodes = vec![vec!['.'; input[0].len() as usize]; input.len() as usize];
    for (c, v) in map {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i != j {
                    let (x1, y1) = v[i];
                    let (x2, y2) = v[j];
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let mut xa1 = x1; // ! important -> the original position also counts as an antinode
                    let mut ya1 = y1;
                    let mut xa2 = x2;
                    let mut ya2 = y2;

                    while xa1 >= 0 && xa1 < input.len() as i64 && ya1 >= 0 && ya1 < input[0].len() as i64 {
                        antinodes[xa1 as usize][ya1 as usize] = '#';
                        xa1 -= dx;
                        ya1 -= dy;
                    }

                    while xa2 >= 0 && xa2 < input.len() as i64 && ya2 >= 0 && ya2 < input[0].len() as i64 {
                        antinodes[xa2 as usize][ya2 as usize] = '#';
                        xa2 += dx;
                        ya2 += dy;
                    }

                }
            }
        }
    }
    return antinodes;
}

fn count_hashtags(antinodes: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    for i in 0..antinodes.len() {
        for j in 0..antinodes[0].len() {
            if antinodes[i][j] == '#' {
                count += 1;
            }
        }
    }
    return count;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let map = parse_input(input);
    let antinodes = calculate_antinodes(input, &map);
    return count_hashtags(&antinodes);
}