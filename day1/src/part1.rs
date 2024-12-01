use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn parse_input(input: &Vec<String>) -> (Vec<i64>, Vec<i64>) {

    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split("   ").collect();
        let left = parts[0];
        let right = parts[1];
        left_list.push(left.parse::<i64>().unwrap());
        right_list.push(right.parse::<i64>().unwrap());
    }

    return (left_list, right_list);

}



pub fn part1(input: &Vec<String>) -> i64 {

    let mut total_distance: i64 = 0;
    let (mut left_list, mut right_list) = parse_input(input);

    let mut left_heap: BinaryHeap<Reverse<i64>> = left_list.into_iter().map(Reverse).collect();
    let mut right_heap: BinaryHeap<Reverse<i64>> = right_list.into_iter().map(Reverse).collect();

    while let (Some(Reverse(min_left_value)), Some(Reverse(min_right_value))) = (left_heap.pop(), right_heap.pop()) {
        let distance = (min_left_value - min_right_value).abs();
        total_distance += distance;
    }

    return total_distance;
}