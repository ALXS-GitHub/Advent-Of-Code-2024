fn parse_input(input: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut is_design_section = false;
    let mut designs = Vec::new();
    let mut patterns = Vec::new();

    for line in input {
        if line == "" {
            is_design_section = true;
            continue;
        }

        if !is_design_section {
            patterns.extend(line.split(", ").map(|x| x.to_string()).collect::<Vec<String>>());
        } else {
            designs.push(line.clone());
        }
    }

    return (patterns, designs);
}

fn dp(patterns: &Vec<String>, design: String) -> i64 {
    let mut dp = vec![0; design.len() + 1];

    dp[0] = 1;

    for i in 1..=design.len() {
        for pattern in patterns {
            if design[0..i].ends_with(pattern) {
                dp[i] += dp[i - pattern.len()];
            }
        }
    }

    return dp[design.len()];
}

pub fn part2(input: &Vec<String>) -> i64 {

    let (patterns, designs) = parse_input(input);

    let mut counter = 0;

    for design in designs {
        counter += dp(&patterns, design);
    }

    return counter;
}