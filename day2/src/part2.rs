pub fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut reports: Vec<Vec<i64>> = Vec::new();

    for line in input {
        let mut report: Vec<i64> = Vec::new();
        for num in line.split(" ") {
            report.push(num.parse().unwrap());
        }
        reports.push(report);
    }

    return reports;
}

pub fn is_safe(report: &Vec<i64>) -> bool {
    
    if report[0] < report[1] { // incr
        for i in 0..report.len() - 1 {
            let diff = report[i+1] - report[i];
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    } else if report[0] > report[1] { // decr
        for i in 0..report.len() - 1 {
            let diff = report[i] - report[i+1];
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    } else {
        return false;
    }

    return true;
}

pub fn is_safe_4(report: &Vec<i64>) -> bool {
    
    for i in 0..report.len() {

        let mut report_copy = report.clone();
        report_copy.remove(i);

        if is_safe(&report_copy) {
            return true;
        }
    }
    return false
}

pub fn part2(input: &Vec<String>) -> i64 {
        
    let reports = parse_input(input);

    let mut sum = 0;
    for report in reports {
        if is_safe_4(&report) {
            sum += 1;
        }
    }

    return sum;
}
