use crate::utils::load_input;
const DAY: u32 = 1;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut ls1: Vec<i32> = Vec::new();
    let mut ls2: Vec<i32> = Vec::new();

    // split each line into two numbers and push numbers into respective vectors
    for line in lines {
        let mut numbers = line.split_whitespace();
        ls1.push(numbers.next().unwrap().parse().unwrap());
        ls2.push(numbers.next().unwrap().parse().unwrap());
    }

    let result_part1 = part1(ls1.clone(), ls2.clone());
    let result_part2 = part2(ls1, ls2);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(mut ls1: Vec<i32>, mut ls2: Vec<i32>) -> i32 {
    let mut ls3: Vec<i32> = Vec::new();
    let mut result = 0;

    ls1.sort();
    ls2.sort();

    // get differences of corresponding numbers
    for (first, second) in ls1.into_iter().zip(ls2) {
        ls3.push((first-second).abs())
    }

    for number in ls3 {
        result += number;
    }

    result
}

fn part2(ls1: Vec<i32>, ls2: Vec<i32>) -> i32 {
    let mut ls3: Vec<i32> = Vec::new();
    let mut result = 0;

    for number in ls1 {
        let count = ls2.iter().filter(|x| **x == number).count();
        ls3.push(number * count as i32);
    }

    for number in ls3 {
        result += number;
    }

    result
}
