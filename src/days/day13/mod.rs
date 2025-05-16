use crate::utils::load_input;
const DAY: u32 = 13;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let clawmachines = parse_input(&lines);

    let result_part1 = part1(&clawmachines);
    let result_part2 = part2(&clawmachines);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Debug)]
struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

impl ClawMachine {
    // return 0 when too low, 1 when matching and 2 when too high
    fn check_combination(&self, a: usize, b: usize) -> usize {
        let x = a * self.a.0 + b * self.b.0;
        let y = a * self.a.1 + b * self.b.1;

        return
            if self.prize.0 < x || self.prize.1 < y {
                2
            } else if self.prize.0 == x && self.prize.1 == y {
                1
            } else {
                0
            }
    }

    fn check_combination_correct(&self, a: usize, b: usize) -> usize {
        let x = a * self.a.0 + b * self.b.0;
        let y = a * self.a.1 + b * self.b.1;

        return
            if self.prize.0 + 10000000000000 < x || self.prize.1 + 10000000000000 < y {
                2
            } else if self.prize.0 + 10000000000000 == x && self.prize.1 + 10000000000000 == y {
                1
            } else {
                0
            }
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<ClawMachine> {
    let mut result = Vec::new();

    // every 4 lines is all the numbers for 1 ClawMachine
    for machine in lines.chunks(4) {
        let mut buffer = Vec::new();

        // Look for numbers
        for split in machine.concat().split(&['+', '=', ',', 'B', 'P']) {
            if let Ok(i) = split.parse::<usize>() {
             buffer.push(i);
            }
        }

        // Create new ClawMachine
        result.push(ClawMachine {
            a: (buffer[0], buffer[1]),
            b: (buffer[2], buffer[3]),
            prize: (buffer[4], buffer[5]),
        });
    }

    result
}

fn tokens_for_prize(machine: &ClawMachine) -> usize {
    let mut result = 0;
    let mut options: Vec<(usize, usize)> = Vec::new();

    // start with the highest possible a and check all b's up to 100
    for a in (0..=(machine.prize.0 / machine.a.0).max(machine.prize.1 / machine.a.1).min(100)).rev() {
        for b in 0..101 {
            match machine.check_combination(a, b) {
                2 => break,
                1 => options.push((a, b)),
                _ => (),
            }
        }
    }

    // check for cheapest option and set result
    for (a, b) in options {
        let tokens = 3* a + b;
        if result == 0 || tokens < result {
            result = tokens
        }
    }

    result
}

fn tokens_for_prize_correct(machine: &ClawMachine) -> usize {
    let result = 0;

    result
}

fn part1(machines: &Vec<ClawMachine>) -> usize {
    let mut result = 0;

    for machine in machines {
        result += tokens_for_prize(machine);
    }

    result
}

fn part2(machines: &Vec<ClawMachine>) -> usize {
    let mut result = 0;

    for machine in machines {
        result += tokens_for_prize_correct(machine);
    }

    result
}