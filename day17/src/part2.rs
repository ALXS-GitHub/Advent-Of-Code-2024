use regex::Regex;

fn parse_input(input: &Vec<String>) -> (i64, i64, i64, Vec<i64>) {

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut program = Vec::new();

    for line in input {
        if line == "" {
            continue;
        }
        if line.starts_with("Register A") {
            let re = Regex::new(r"Register A: (\d+)").unwrap();
            let cap = re.captures(line).unwrap();
            a = cap[1].parse::<i64>().unwrap();
        } else if line.starts_with("Register B") {
            let re = Regex::new(r"Register B: (\d+)").unwrap();
            let cap = re.captures(line).unwrap();
            b = cap[1].parse::<i64>().unwrap();
        } else if line.starts_with("Register C") {
            let re = Regex::new(r"Register C: (\d+)").unwrap();
            let cap = re.captures(line).unwrap();
            c = cap[1].parse::<i64>().unwrap();
        } else if line.starts_with("Program") {
            let nb_list = line.split(": ").collect::<Vec<&str>>();
            let nb = nb_list[1].split(",").collect::<Vec<&str>>();
            for n in nb {
                program.push(n.parse::<i64>().unwrap());
            }
        }
    }

    return (a, b, c, program);

}

fn get_combo_operand(value: i64, a: i64, b: i64, c: i64) -> i64 {
    match value {
        0..=3 => value,
        4 => a,
        5 => b,
        6 => c,
        7 => {
            // Operand 7 is reserved; handle accordingly
            panic!("Invalid combo operand: 7");
        },
        _ => panic!("Invalid combo operand: {}", value),
    }
}

fn run_program(program: &Vec<i64>, a: &mut i64, b: &mut i64, c: &mut i64) -> Vec<i64> {

    let mut ip = 0;
    let mut out = Vec::new();

    while ip < program.len() {
        let instr = program[ip];
        let value = program[ip + 1];

        match instr {
            0 => {
                let co = get_combo_operand(value, *a, *b, *c);
                let res = *a / 2i64.pow(co as u32);
                *a = res;
            },
            1 => {
                let lo = value;
                let res = *b ^ lo;
                *b = res;
                // println!("b: {}", *b);
            },
            2 => {
                let co = get_combo_operand(value, *a, *b, *c);
                let res = (co + 8) % 8;
                *b = res;
                // println!("b: {}", *b);
            }, 
            3 => {
                if *a == 0 {
                    ip += 2;
                    continue;
                } else {
                    ip = value as usize;
                    continue;
                }
            },
            4 => {
                let res = *b ^ *c;
                *b = res;
                // println!("b: {}", *b);
            },
            5 => {
                let co = get_combo_operand(value, *a, *b, *c);
                let res = (co + 8) % 8;
                out.push(res);
            }, 
            6 => {
                let co = get_combo_operand(value, *a, *b, *c);
                let res = *a / 2i64.pow(co as u32);
                *b = res;
            },
            7 => {
                let co = get_combo_operand(value, *a, *b, *c);
                let res = *a / 2i64.pow(co as u32);
                *c = res;
            },
            _ => {
                println!("Unknown instruction: {}", instr);
                break;
            }
        }

        ip += 2;
    }

    return out;
}

fn solve_recursive(a: i64, b: i64, c: i64, program: &Vec<i64>, pow: i64) -> Option<i64> {
    if pow < 0 {
        return Some(a);
    }

    for n in 0..8 {
        let _a = a + n * 8i64.pow(pow as u32);
        let output = run_program(program, &mut _a.clone(), &mut b.clone(), &mut c.clone());

        if pow < output.len() as i64 && output[pow as usize] == program[pow as usize] {
            if let Some(final_a) = solve_recursive(_a, b, c, program, pow - 1) {
                return Some(final_a);
            }
        }
    }

    None    
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (a, b, c, program) = parse_input(input);
    let pow = program.len() as i64 - 1;

    let res = solve_recursive(0, b, c, &program, pow).unwrap();

    return res;
}