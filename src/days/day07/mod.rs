use crate::utils::load_input;
const DAY: u32 = 7;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut equations: Vec<(usize, Vec<usize>)> = Vec::new();

    // parse lines into the equations vector
    for line in lines {
        if let Some((value, equation)) = line.split_once(':') {
            equations.push((
                value.parse().expect("first element in line not a number"),
                equation
                    .split_whitespace()
                    .map(|x| x.parse()
                    .expect("not a number in equation")).rev().collect()
            ));
        }
    }

    let result_part1 = part1(equations.clone());
    let result_part2 = part2(equations);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn tryout_operators(target: usize, total: usize, mut numbers: Vec<usize>) -> bool {
    if numbers.is_empty() {
        return target == total;
    }

    if let Some(next) = numbers.pop() {
        if tryout_operators(target, total + next, numbers.clone()) {
            return true;
        }
        if tryout_operators(target, total * next, numbers) {
            return true;
        }
    }

    false
}

fn tryout_operators_2(target: usize, total: usize, mut numbers: Vec<usize>) -> bool {
    if numbers.is_empty() {
        return target == total;
    }
    if let Some(next) = numbers.pop() {

        // check if next operator is +
        if tryout_operators_2(target, total + next, numbers.clone()) {
            return true;
        }

        // check if next operator is *
        let new_total = if total == 0 { next } else { total * next };
        if tryout_operators_2(target, new_total, numbers.clone()) {
            return true;
        }

        // check if next operator is ||
        if tryout_operators_2(target, concatenate_numbers(total, next), numbers) {
            return true;
        }
    }

    false
}

fn concatenate_numbers(a: usize, b: usize) -> usize {
    let b_digits = 10usize.pow(b.to_string().len() as u32);
    a * b_digits + b
}

fn part1(equations: Vec<(usize, Vec<usize>)>) -> i64 {
    let mut result: i64 = 0;

    // try out possible all operator combinations, if something works, add test value to result
    for mut equation in equations {
        let total = equation.1.pop().expect("no number in equation");
        if tryout_operators(equation.0, total, equation.1) {result += equation.0 as i64}
    }

    result
}

fn part2(equations: Vec<(usize, Vec<usize>)>) -> i64 {
    let mut result: i64 = 0;

    // try out possible all operator combinations, if something works, add test value to result
    for equation in equations {
        if tryout_operators_2(equation.0, 0, equation.1) {result += equation.0 as i64}
    }

    result
}