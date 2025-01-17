use crate::utils::load_input;
use regex::Regex;

const DAY: u32 = 3;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let result_part1 = part1(&lines);
    let result_part2 = part2(&lines);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(lines: &Vec<String>) -> i32 {
    // Regex, the r in front avoids the need for \\ (escaping the \)
    // the extra brackets are the capture groups (the two numbers)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut pairs_of_numbers: Vec<(i32, i32)> = Vec::new();
    let mut result = 0;

    // go through all lines and capture numbers
    for line in lines {
        // capture the two numbers f1 and f2 for each match using captures_iter
        for (_, [f1, f2]) in re.captures_iter(line).map(|caps| caps.extract()) {
            // push the numbers into the pairs_of_numbers vector
            pairs_of_numbers.push((
                f1.parse::<i32>().expect("couldn't parse"),
                f2.parse::<i32>().expect("couldn't parse"),
            ));
        }
    }

    // multiply number pairs and add to result
    for (x, y) in pairs_of_numbers {
        result += x * y ;
    }

    result
}

fn part2(lines: &Vec<String>) -> i32 {
    // get all lines into one string
    let mut all_lines = String::new();
    for line in lines {
        all_lines.push_str(line);
    }

    // collect the strings between dos and don'ts, so the lines between don'ts and dos are left out
    let mut do_commands: Vec<String> = Vec::new();
    // strings between do and don't, also from start to do, and do to end, not greedy
    let do_regex = Regex::new(r"((^|do\(\)).*?)($|don't\(\))").unwrap();
    for (_, [f1, _, _]) in do_regex.captures_iter(&all_lines).map(|caps| caps.extract()) {
        // push all strings that start with do into the do_commands vector
        do_commands.push(String::from(f1));
    }

    // use part 1 to check all residual strings
    part1(&do_commands)
}