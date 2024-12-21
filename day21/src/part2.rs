use std::collections::HashMap;
use lazy_static::lazy_static;
use itertools::Itertools;

const NUMPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A']
];

const DIRPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>']
];

lazy_static! {
    #[derive(Debug)]
    static ref NUMPAD_MAP: HashMap<char, (usize, usize)> = {
        let mut map = HashMap::new();
        for i in 0..4 {
            for j in 0..3 {
                map.insert(NUMPAD[i][j], (i, j));
            }
        }
        map
    };

    #[derive(Debug)]
    static ref DIRPAD_MAP: HashMap<char, (usize, usize)> = {
        let mut map = HashMap::new();
        for i in 0..2 {
            for j in 0..3 {
                map.insert(DIRPAD[i][j], (i, j));
            }
        }
        map
    };
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, Vec<i64>) {
    let mut codes = Vec::new();
    let mut associated_numbers = Vec::new();

    for line in input {
        let mut code = Vec::new();
        for c in line.chars() {
            code.push(c);
        }
        codes.push(code);

        let num = line.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap();
        associated_numbers.push(num);
    }

    return (codes, associated_numbers);
} 

fn get_all_permutations_way(way: Vec<char>) -> Vec<Vec<char>> {
    let permutations: Vec<Vec<char>> = way.clone().into_iter().permutations(way.len()).collect();
    let mut permutations = permutations.into_iter().unique().collect::<Vec<Vec<char>>>();
    return permutations;
}

fn find_ways_numpad(a: char, b: char) -> Vec<Vec<char>> {

    let a_pos = NUMPAD_MAP.get(&a).unwrap();
    let b_pos = NUMPAD_MAP.get(&b).unwrap();

    let mut di = b_pos.0 as i32 - a_pos.0 as i32;
    let mut dj = b_pos.1 as i32 - a_pos.1 as i32;

    let mut way = Vec::new();

    if di > 0 {
        for _ in 0..di {
            way.push('v');
        }
    } else if di < 0 {
        for _ in 0..-di {
            way.push('^');
        }
    }

    if dj > 0 {
        for _ in 0..dj {
            way.push('>');
        }
    } else if dj < 0 {
        for _ in 0..-dj {
            way.push('<');
        }
    }

    let mut ways = get_all_permutations_way(way);
    let mut valid_ways = Vec::new();

    // check if the way is valid
    for w in ways {
        let mut i = a_pos.0 as i32;
        let mut j = a_pos.1 as i32;
        let mut valid = true;

        for c in w.clone().into_iter() {
            if c == '^' {
                i -= 1;
            } else if c == 'v' {
                i += 1;
            } else if c == '<' {
                j -= 1;
            } else if c == '>' {
                j += 1;
            }

            if i < 0 || i >= 4 || j < 0 || j >= 3 {
                valid = false;
                break;
            }

            if NUMPAD[i as usize][j as usize] == ' ' {
                valid = false;
                break;
            }

        }

        if valid {
            let mut wa = w.clone();
            wa.extend(vec!['A']);
            valid_ways.push(wa);
        }
    }

    return valid_ways;

}

fn find_ways_dirpad(a: char, b: char) -> Vec<Vec<char>> {

    let a_pos = DIRPAD_MAP.get(&a).unwrap();
    let b_pos = DIRPAD_MAP.get(&b).unwrap();

    let mut di = b_pos.0 as i32 - a_pos.0 as i32;
    let mut dj = b_pos.1 as i32 - a_pos.1 as i32;

    let mut way = Vec::new();

    if di > 0 {
        for _ in 0..di {
            way.push('v');
        }
    } else if di < 0 {
        for _ in 0..-di {
            way.push('^');
        }
    }

    if dj > 0 {
        for _ in 0..dj {
            way.push('>');
        }
    } else if dj < 0 {
        for _ in 0..-dj {
            way.push('<');
        }
    }

    let mut ways = get_all_permutations_way(way);
    let mut valid_ways = Vec::new();

    // check if the way is valid
    for w in ways {
        let mut i = a_pos.0 as i32;
        let mut j = a_pos.1 as i32;
        let mut valid = true;

        for c in w.clone().into_iter() {
            if c == '^' {
                i -= 1;
            } else if c == 'v' {
                i += 1;
            } else if c == '<' {
                j -= 1;
            } else if c == '>' {
                j += 1;
            }

            if i < 0 || i >= 2 || j < 0 || j >= 3 {
                valid = false;
                break;
            }

            if DIRPAD[i as usize][j as usize] == ' ' {
                valid = false;
                break;
            }

        }

        if valid {
            let mut wa = w.clone();
            wa.extend(vec!['A']);
            valid_ways.push(wa);
        }
    }

    return valid_ways;

}

fn get_shortest_way(a: char, b: char, depth: i64, max_depth: i64, cache: &mut HashMap<(char, char, i64), i64>) -> i64 {

    if cache.contains_key(&(a, b, depth)) {
        return *cache.get(&(a, b, depth)).unwrap();
    }

    if depth == 0 {
        let ways = find_ways_dirpad(a, b);
        cache.insert((a, b, depth), ways.iter().map(|w| w.len() as i64).min().unwrap());
        return ways.iter().map(|w| w.len() as i64).min().unwrap();
    }

    let ways;
    if depth == max_depth {
        ways = find_ways_numpad(a, b);  
    } else {
        ways = find_ways_dirpad(a, b);
    }

    let mut min_way = i64::MAX;
    for w in ways {
        let mut way = vec!['A'];
        way.extend(w);

        let mut total = 0;
        for i in 0..way.len() - 1 {
            total += get_shortest_way(way[i], way[i + 1], depth - 1, max_depth, cache);
        }

        if total < min_way {
            min_way = total;
        }
        
    }

    cache.insert((a, b, depth), min_way);
    return min_way;

} 

pub fn part2(input: &Vec<String>) -> i64 {

    let mut cache = HashMap::new();

    let (codes, associated_numbers) = parse_input(input);

    let mut total = 0;

    for (n, code) in codes.iter().enumerate() {
        let mut code_a = vec!['A'];
        code_a.extend(code.clone());
        let mut code_total = 0;
        for i in 0..code_a.len() - 1 {
            code_total += get_shortest_way(code_a[i], code_a[i + 1], 25, 25, &mut cache);
        }

        total += code_total * associated_numbers[n];
    }

    return total;
}