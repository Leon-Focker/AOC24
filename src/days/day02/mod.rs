use crate::utils::load_input;
const DAY: u32 = 2;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let result_part1 = part1(&lines);
    let result_part2 = part2(&lines);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut result = 0;

    for line in lines {
        let report = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if check_report(report) {result += 1};
    }

    result
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut result = 0;

    for line in lines {
        let report = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if check_report_dampened(report) {result += 1};
    }

    result
}

fn check_report(report: Vec<i32>) -> bool {
    // true if numbers are increasing, false if not
    let increasing: bool = (report[0] - report[1]) < 0;
    let mut last = report[0];

    for number in &report[1..] {
        let diff = last - number;

        // check if all increase/decrease within allowed difference
        if diff.abs() < 1 || diff.abs() > 3 {return false};
        // check whether all increase / decrease
        if (diff < 0) != increasing {return false};

        last = *number;
    }

    true
}

fn check_report_dampened(report: Vec<i32>) -> bool {
    let mut concerns: Vec<i32> = Vec::new();
    let mut last = report[0];
    let mut decreasing: Vec<i32> = Vec::new();
    let mut increasing: Vec<i32> = Vec::new();

    // check all diffs, push concerning steps into concerns, count increasing and decreasing steps
    for number in &report[1..] {
        let diff = last - number;
        if diff.abs() < 1 || diff.abs() > 3 {concerns.push(1)}
        else {concerns.push(0)};
        if diff < 0 {increasing.push(1); decreasing.push(0)}
        else {increasing.push(0); decreasing.push(1)};
        last = *number;
    }

    // find minimum number of decreasing/increasing steps
    let min_false_direction = if decreasing.iter().sum::<i32>() < increasing.iter().sum::<i32>() {
        decreasing
    } else {increasing};

    // if everything checks out, return true
    if min_false_direction.iter().sum::<i32>() == 0 && concerns.iter().sum::<i32>() == 0 {return true}
    // else check, whether removing a number helps
    // this removes the numbers where things did not check out
    else if min_false_direction.iter().sum::<i32>() < 2 && concerns.iter().sum::<i32>() < 2 {
        let mut index = 0;
        for (i, j) in concerns.iter().zip(min_false_direction) {
            if *i == 1 || j == 1 {break} else {index += 1};
        }
        let mut modified_vec = report.clone();
        let mut modified_vec2 = report.clone();
        modified_vec.remove(index);
        modified_vec2.remove(index+1);
        return check_report(modified_vec) || check_report(modified_vec2)
    };

    false
}