use log::warn;
use num_traits::pow;
use crate::utils::load_input;
const DAY: u32 = 17;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut pc: Computer = parse_input(&lines);

    let result_part1: String = part1(&mut pc);
    let result_part2 = part2(&mut pc);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Debug)]
struct Computer {
    program: Vec<u8>,
    program_cnt: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    out: Vec<usize>,
}

impl Computer {
    fn new() -> Self {
        Computer { program: Vec::new(), program_cnt: 0, reg_a: 0, reg_b: 0, reg_c: 0, out: Vec::new() }
    }

    fn print_out(&self) -> String {
        self.out
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn run_program(&mut self) -> () {
        while self.program_cnt < self.program.len() {
            if self.run_instruction() {
                self.program_cnt += 2;
            }
        }
    }

    fn run_instruction(&mut self) -> bool {
        let operand = self.program[self.program_cnt+1];
        match self.program[self.program_cnt] {
            0 => self.adv(self.get_combo(operand)),
            1 => self.bxl(operand as usize),
            2 => self.bst(self.get_combo(operand)),
            3 => self.jnz(operand as usize),
            4 => self.bxc(),
            5 => self.out(self.get_combo(operand)),
            6 => self.bdv(self.get_combo(operand)),
            7 => self.cdv(self.get_combo(operand)),
            _ => true,
        }
    }

    fn get_combo(&self, operand: u8) -> usize {
        match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => { warn!("7 as combo operand!"); 7 },
            _ => operand as usize,
        }
    }

    fn adv(&mut self, operand: usize) -> bool {
        self.reg_a = (self.reg_a as f64 / pow(2, operand) as f64).floor() as usize;
        true
    }

    fn bxl(&mut self, operand: usize) -> bool {
        self.reg_b = self.reg_b ^ operand;
        true
    }

    fn bst(&mut self, operand: usize) -> bool {
        self.reg_b = operand.rem_euclid(8);
        true
    }

    fn jnz(&mut self, operand: usize) -> bool {
        if self.reg_a > 0 {
            self.program_cnt = operand;
            false
        } else {
            true
        }
    }

    fn bxc(&mut self) -> bool {
        self.reg_b = self.reg_b ^ self.reg_c;
        true
    }

    fn out(&mut self, operand: usize) -> bool {
        self.out.push(operand.rem_euclid(8));
        true
    }

    fn bdv(&mut self, operand: usize) -> bool {
        self.reg_b = (self.reg_a as f64 / pow(2, operand) as f64).floor() as usize;
        true
    }

    fn cdv(&mut self, operand: usize) -> bool {
        self.reg_c = (self.reg_a as f64 / pow(2, operand) as f64).floor() as usize;
        true
    }
}



fn parse_input(lines: &Vec<String>) -> Computer {
    let mut pc: Computer = Computer::new();

    for (i, line) in lines.iter().enumerate() {
        match i {
            0 => pc.reg_a = line[12..].parse().expect("couldn't parse Reg A!"),
            1 => pc.reg_b = line[12..].parse().expect("couldn't parse Reg B!"),
            2 => pc.reg_c = line[12..].parse().expect("couldn't parse Reg C!"),
            4 => pc.program = line[9..]
                .split(',')
                .map(|x| x.parse::<u8>().expect("couldn't parse program!"))
                .collect(),
            _ => (),
        }
    }

    pc
}

fn part1(pc: &mut Computer) -> String {
    pc.run_program();
    pc.print_out()
}

fn part2(_pc: &mut Computer) -> usize {
    0
}