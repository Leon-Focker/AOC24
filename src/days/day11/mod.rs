use std::collections::HashMap;
use crate::utils::{load_input, num_digits};
const DAY: u32 = 11;
const POWERS_OF_TEN: [usize; 20] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
    10000000000, 100000000000, 1000000000000, 10000000000000, 100000000000000,
    1000000000000000, 10000000000000000, 100000000000000000, 1000000000000000000, 10000000000000000000,
];


pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let line = &lines[0];

    // collect numbers into LineOfStones
    let line_of_stones: LineOfStones = line
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap_or_else(|_| panic!("Invalid number: {}", x)))
        .collect::<Vec<_>>()
        .into();

    let result_part1 = blink_on_line_of_stones(&line_of_stones, 25);
    let result_part2 = blink_on_line_of_stones(&line_of_stones, 75);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Debug, Default)]
struct LineOfStones {
    numbers: Vec<usize>,
}

impl From<Vec<usize>> for LineOfStones {
    fn from(vec: Vec<usize>) -> Self {
        LineOfStones { numbers: vec }
    }
}

// blink a number of times on a stone and return the number of stones this results in
fn blinks_one_stone(stone: usize, blinks_left: usize, lookup: &mut HashMap<(usize, usize), usize>) -> usize {
    // Check, whether we had these same arguments before:
    if let Some(result) = lookup.get(&(stone, blinks_left)) {
        return *result
    }

    // Else calculate the result and store it in the lookup table
    let result =
        if blinks_left == 0 {
            // when we're done blinking, return 1, as in one stone, to add to the other stones
            1
        } else if stone == 0 {
            // rule number one
            blinks_one_stone(1, blinks_left - 1, lookup)
        } else {
            // rule number two: Split numbers with an even number of digits in half
            let digits = num_digits(stone);

            if digits % 2 == 0 {
                // Calculate first and second halves
            let divisor = POWERS_OF_TEN[digits / 2];
                let first = stone / divisor;
                let second = stone % divisor;

                blinks_one_stone(first, blinks_left - 1, lookup)
                    + blinks_one_stone(second, blinks_left - 1, lookup)
            } else {
                // rule number three
                blinks_one_stone(stone * 2024, blinks_left - 1, lookup)
            }
        };

    // store value in lookup table
    lookup.insert((stone, blinks_left), result);

    result
}

fn blink_on_line_of_stones(line_of_stones: &LineOfStones, nr_blinks: usize) -> usize {

    let mut result = 0;
    let mut lookup: HashMap<(usize, usize), usize> = HashMap::new();

    for stone in line_of_stones.numbers.iter() {
        result += blinks_one_stone(*stone, nr_blinks, &mut lookup);
    }

    result
}