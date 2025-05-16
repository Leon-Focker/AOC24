use crate::utils::load_input;
const DAY: u32 = 14;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let floor: Floor = if debug {
        Floor::new(11, 7)
    } else {
        Floor::new(101, 103)
    };
    let mut robots = parse_robots(&lines);

    let result_part1 = part1(&mut robots, &floor);
    let _result_part2 = part2(&mut robots, &floor);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    //println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn parse_robots(lines: &Vec<String>) -> Vec<Robot> {
    let mut result: Vec<Robot> = Vec::new();

    for line in lines {
        let mut buffer = Vec::new();

        // Look for numbers
        for split in line.split(&['=', ',', ' ']) {
            if let Ok(i) = split.parse::<isize>() {
                buffer.push(i);
            }
        }

        // Create new Robot
        result.push(Robot {
            x: buffer[0],
            y: buffer[1],
            vel_x: buffer[2],
            vel_y: buffer[3],
        });
    }

    result
}

struct Floor {
    width: usize,
    height: usize,
}

impl Floor {
    fn new(width: usize, height: usize) -> Self {
        Floor {width, height}
    }
}

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    vel_x: isize,
    vel_y: isize,
}

impl From<(isize, isize, isize, isize)> for Robot {
    fn from((x, y, vel_x, vel_y): (isize, isize, isize, isize)) -> Self {
        Self { x, y, vel_x, vel_y }
    }
}

impl Robot {
    fn update(&mut self, floor: &Floor) -> () {
        self.x = (self.x + self.vel_x).rem_euclid(floor.width as isize);
        self.y = (self.y + self.vel_y).rem_euclid(floor.height as isize);
    }

    fn matching_tile(&self, x: isize, y: isize) -> bool {
        if self.x == x && self.y == y { true } else { false }
    }
}

fn calculate_safety_factor(robots: &Vec<Robot>, floor: &Floor) -> usize {
    let vertical = floor.width as f64 / 2.0 - 0.5;
    let horizontal = floor.height as f64 / 2.0 - 0.5;
    let mut quadrants = vec![0,0,0,0];

    for robot in robots {
        let x = robot.x as f64;
        let y = robot.y as f64;

        if x < vertical {
            if y < horizontal {
                quadrants[0]+= 1;
            } else if y > horizontal {
                quadrants[2]+= 1;
            }
        } else if x > vertical {
            if y < horizontal {
                quadrants[1]+= 1;
            } else if y > horizontal {
                quadrants[3]+= 1;
            }
        }
    }

    quadrants[0] * quadrants[1] *quadrants[2] * quadrants[3]
}

fn check_for_possible_tree(robots: &Vec<Robot>, floor: &Floor) -> bool {
    let mut floor_map: Vec<Vec<bool>> = vec![vec![false; floor.width]; floor.height];
    let mut max_streak = 0;
    let mut streak = 0;

    for robot in robots {
        floor_map[robot.y as usize][robot.x as usize] = true;
    }

    for y in 0..floor.height {
        for x in 0..floor.width {
            if floor_map[y][x] {
                streak += 1;
            } else {
                max_streak = max_streak.max(streak);
                streak = 0;
            }
        }
    }
    max_streak = max_streak.max(streak);

    if max_streak > 5 {
        true
    } else {
        false
    }
}

fn print_floor(robots: &Vec<Robot>, floor: &Floor) -> () {
    println!();
    for y in 0..floor.height {
        println!();
        for x in 0..floor.width {
            if robots.iter().any(|r| r.matching_tile(x as isize, y as isize)) {
                print!("O")
            } else {
                print!(".")
            }
        }
    }
}

fn part1(robots: &mut Vec<Robot>, floor: &Floor) -> usize {

    for _ in 0..100 {
        for robot in &mut *robots {
            robot.update(&floor);
        }
    }

    calculate_safety_factor(robots, floor)
}

fn part2(robots: &mut Vec<Robot>, floor: &Floor) -> () {

    for i in 100..8000 {
        if check_for_possible_tree(&robots, floor) {
            print_floor(robots, floor);
            println!();
            println!("{i}");
        }

        for robot in &mut *robots {
            robot.update(&floor);
        }
    }
}