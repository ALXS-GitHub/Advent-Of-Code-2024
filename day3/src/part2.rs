use regex::Regex;

fn parse_input(input: &Vec<String>) -> i64 {

    let mut sum = 0;
    let mut enabled = true;

    for line in input {
        let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
        // Find all matches
        let matches: Vec<&str> = re.find_iter(&line).map(|mat| mat.as_str()).collect();

        for m in matches {
            if m == "do()" {
                enabled = true;
            } else if m == "don't()" {
                enabled = false;
            } else {
                if enabled == false {
                    continue;
                }
                let nums: Vec<&str> = m.split(",").collect();
                let num1: i64 = nums[0].split("(").collect::<Vec<&str>>()[1].parse().unwrap();
                let num2: i64 = nums[1].split(")").collect::<Vec<&str>>()[0].parse().unwrap();
                sum += num1 * num2;
            }
        } 
    }

    return sum;
    
}

pub fn part2(input: &Vec<String>) -> i64 {
    return parse_input(input);
}