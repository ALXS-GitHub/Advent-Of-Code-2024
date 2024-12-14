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

fn get_quadrants(robots: &BTreeMap<i64, Robot>, max_x: i64, max_y: i64) -> i64 {
    let mut quadrants = vec![0, 0, 0, 0]; // top-left, top-right, bottom-left, bottom-right

    for (_, robot) in robots.iter() {
        if robot.position.0 < max_x / 2 && robot.position.1 < max_y / 2 {
            quadrants[0] += 1;
        } else if robot.position.0 >= max_x / 2 + 1 && robot.position.1 < max_y / 2 {
            quadrants[1] += 1;
        } else if robot.position.0 < max_x / 2 && robot.position.1 >= max_y / 2 + 1 {
            quadrants[2] += 1;
        } else if robot.position.0 >= max_x / 2 + 1 && robot.position.1 >= max_y / 2 + 1 {
            quadrants[3] += 1;
        }
    }

    return quadrants.iter().fold(1, |acc, &x| acc * x);

}

pub fn part1(input: &Vec<String>) -> i64 {

    let max_x: i64 = 101; // don't forget to change this when needed
    let max_y: i64 = 103;
    let time: i64 = 100;

    let mut robots = parse_input(&input);
    
    for _ in 0..time {
        move_all(&mut robots, max_x, max_y);
    }
    
    return get_quadrants(&robots, max_x, max_y);

}