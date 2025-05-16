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
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl ClawMachine {
    // return 0 when too low, 1 when matching and 2 when too high
    fn check_combination(&self, a: isize, b: isize) -> isize {
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

    fn verify_combination(&self, a: isize, b: isize) -> bool {
        let x = a * self.a.0 + b * self.b.0;
        let y = a * self.a.1 + b * self.b.1;

        self.prize.0 == x && self.prize.1 == y
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<ClawMachine> {
    let mut result = Vec::new();

    // every 4 lines is all the numbers for 1 ClawMachine
    for machine in lines.chunks(4) {
        let mut buffer = Vec::new();

        // Look for numbers
        for split in machine.concat().split(&['+', '=', ',', 'B', 'P']) {
            if let Ok(i) = split.parse::<isize>() {
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

fn tokens_for_prize(machine: &ClawMachine) -> isize {
    let mut result = 0;
    let mut options: Vec<(isize, isize)> = Vec::new();

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

        let tokens = 3 * a + b;
        if result == 0 || tokens < result {
            result = tokens
        }
    }

    result
}

fn get_factor_b(machine: &ClawMachine) -> isize {
    (machine.a.0 * machine.prize.1 - machine.a.1 * machine.prize.0)
        / (machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0)
}

fn get_factor_a(machine: &ClawMachine, factor_b: isize) -> isize {
    (-machine.b.0 * factor_b + machine.prize.0) / machine.a.0
}

fn tokens_for_prize_new(machine: &ClawMachine) -> isize {
    let correct_machine = ClawMachine {
        prize: (machine.prize.0 + 10000000000000, machine.prize.1 + 10000000000000),
        ..*machine
    };
    let b = get_factor_b(&correct_machine);
    let a = get_factor_a(&correct_machine, b);

    if correct_machine.verify_combination(a, b) {
        a * 3 + b
    } else {
        0
    }
}

fn part1(machines: &Vec<ClawMachine>) -> isize {
    let mut result = 0;

    for machine in machines {
        result += tokens_for_prize(machine);
    }

    result
}

fn part2(machines: &Vec<ClawMachine>) -> isize {
    let mut result = 0;

    for machine in machines {
        result += tokens_for_prize_new(machine);
    }

    result
}