use std::collections::BTreeMap;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn parse_input(input: &Vec<String>) -> BTreeMap<i64, Robot> {
    let mut robots = BTreeMap::new();
    let mut id = 0;
    
    for line in input {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let x: i64 = caps[1].parse().unwrap();
        let y: i64 = caps[2].parse().unwrap();
        let vx: i64 = caps[3].parse().unwrap();
        let vy: i64 = caps[4].parse().unwrap();
        let robot = Robot {
            position: (x, y),
            velocity: (vx, vy),
        };
        robots.insert(id, robot);
        id += 1;
    }

    return robots;
}

fn move_all(robots: &mut BTreeMap<i64, Robot>, max_x: i64, max_y: i64) {
    for (_, robot) in robots.iter_mut() {
        robot.position.0 = ((robot.position.0 + robot.velocity.0) % max_x + max_x) % max_x;
        robot.position.1 = ((robot.position.1 + robot.velocity.1) % max_y + max_y) % max_y;
    }
}

fn all_different_pos(robots: &BTreeMap<i64, Robot>) -> bool {
    let mut positions = BTreeMap::new();
    for (_, robot) in robots.iter() {
        positions.insert(robot.position, 0);
    }
    return positions.len() == robots.len();
}

pub fn part2(input: &Vec<String>) -> i64 {
    let max_x: i64 = 101; // don't forget to change this when needed
    let max_y: i64 = 103;
    let mut time: i64 = 0;

    let mut robots = parse_input(&input);
    
    while !all_different_pos(&robots) {
        time += 1;
        move_all(&mut robots, max_x, max_y);
    }
    
    return time;
}