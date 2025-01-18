use crate::utils::load_input;
const DAY: u32 = 6;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let result_part1 = part1(&lines);
    let result_part2 = part2(&lines);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

struct Map {
    grid: Vec<Vec<u32>>,
}

impl Map {
    fn new() -> Self {
        Self {grid: Vec::new()}
    }
    // This could have been nice and recursive but rust said no :c
    fn guard_loop(&mut self, guard_position: (usize, usize, (isize, isize))) -> (usize, usize, (isize, isize)) {
        let new_position=
            (guard_position.0 as isize + guard_position.2.0,
             guard_position.1 as isize + guard_position.2.1,
             guard_position.2);
        // set current position as visited
        self.grid[guard_position.1][guard_position.0] = 2;
        // when out of bounds, we are done
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= self.grid[0].len() as isize
            ||new_position.1 >= self.grid.len() as isize {
            return (0, 0, (0, 0))
        }
        // when there is an obstacle, just rotate orientation and try again
        if self.grid[new_position.1 as usize][new_position.0 as usize] == 1 {
            (guard_position.0, guard_position.1, rotate_orientation(guard_position.2))
            // when there is no obstacle, move on
        } else {(new_position.0 as usize, new_position.1 as usize, new_position.2)}
    }
}

fn rotate_orientation(orientation: (isize, isize)) -> (isize, isize) {
    match orientation {
        (0, -1) => (1, 0),
        (1, 0)  => (0, 1),
        (0, 1)  => (-1, 0),
        (-1, 0) => (0, -1),
        _       => orientation,
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut result = 0;
    let mut map: Map = Map::new();
    // guard position and orientation:
    // (x-position, y-position/line, (x-direction, y-direction)))
    let mut guard_position: (usize, usize, (isize, isize)) = (0, 0, (0, -1));

    // fill the map and set the initial guard position
    for line in lines {
        map.grid.push(Vec::new());
        for char in line.chars() {
            let current_y = map.grid.len() -1;
            if char == '.' {
                map.grid[current_y].push(0);
            } else if char == '#' {
                map.grid[current_y].push(1);
            } else if char == '^' {
                map.grid[current_y].push(0);
                guard_position = (map.grid[current_y].len() -1, current_y, (0, -1));
            }
        }
    }

    // let the guard wander through the grid and mark where he has been
    loop {
        let pos = map.guard_loop(guard_position);
        if let (0, 0, (0, 0)) = pos {
            break; // Exit the loop when the guard has left the map
        } else {
            guard_position = pos; // Update the guard position
        }
    }

    // count the locations that the guard has been to (marked with a 2)
    for line in map.grid {
        for cell in line {
            if cell == 2 {result += 1}
        }
    }

    result
}

fn part2(_lines: &Vec<String>) -> i32 {
    let result = 0;

    result
}