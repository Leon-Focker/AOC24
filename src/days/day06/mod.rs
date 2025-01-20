use crate::utils::load_input;
const DAY: u32 = 6;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
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

    let result_part1 = part1(&mut map, &guard_position);
    let result_part2 = part2(&mut map, &guard_position);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Clone, Debug)]
struct Map {
    grid: Vec<Vec<usize>>,
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
            || new_position.1 >= self.grid.len() as isize {
            return (0, 0, (0, 0))
        }
        // when there is an obstacle, just rotate orientation and try again
        if self.grid[new_position.1 as usize][new_position.0 as usize] == 1 {
            (guard_position.0, guard_position.1, rotate_orientation(guard_position.2))
            // when there is no obstacle, move on
        } else {(new_position.0 as usize, new_position.1 as usize, new_position.2)}
    }
    // similar to before, but other return value
    fn guard_loop2(&mut self, guard_position: (usize, usize, (isize, isize)), step_nr: usize) -> (usize, usize, (isize, isize), usize, usize) {
        let current_value = self.grid[guard_position.1][guard_position.0];
        let new_position=
            (guard_position.0 as isize + guard_position.2.0,
             guard_position.1 as isize + guard_position.2.1,
             guard_position.2);
        // set current position as visited
        if current_value == 0 {self.grid[guard_position.1][guard_position.0] = step_nr}
        // when out of bounds, we are done
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.0 >= self.grid[0].len() as isize
            || new_position.1 >= self.grid.len() as isize {
            return (0, 0, (0, 0), 0, 0)
        }
        // when there is an obstacle, just rotate orientation and try again
        if self.grid[new_position.1 as usize][new_position.0 as usize] == 1 {
            (guard_position.0, guard_position.1, rotate_orientation(guard_position.2), current_value, 0)
            // when there is no obstacle, move on
        } else {(new_position.0 as usize, new_position.1 as usize, new_position.2, current_value, 1)}
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

fn part1(map: &mut Map, guard_position: &(usize, usize, (isize, isize))) -> i32 {
    let mut result = 0;
    let mut guard_position: (usize, usize, (isize, isize)) = guard_position.clone();

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
    for line in map.grid.clone() {
        for cell in line {
            if cell >= 2 {result += 1}
        }
    }

    result
}


// This code is kinda poopy but I cannot be bothered to beautify it
fn part2(map: &mut Map, orig_guard_position: &(usize, usize, (isize, isize))) -> i32 {
    let mut coords = Vec::new();
    let mut result = 0;

    // collect all coordinates which we want to check, because the guard can visit them
    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            if map.grid[y][x] == 2 {
                map.grid[y][x] = 0;
                coords.push((x, y))
            }
        }
    }
    
    // go through all coordinates and check what a blockade would do
    for (x, y) in coords {
        let mut new_map = map.clone();
        // set new blockade
        new_map.grid[y][x] = 1;
        let mut guard_position: (usize, usize, (isize, isize)) = orig_guard_position.clone();

        let mut step_nr = 4;
        let mut last_return = 2;

        // let the guard wander through the grid and mark where he has been
        loop {
            let pos = new_map.guard_loop2(guard_position, step_nr);
            // Exit the loop when the guard has left the map
            if let (0, 0, (0, 0), 0, 0) = pos {
                break;
                // else check for a loop and continue if none is found
            } else {
                // Update the guard position
                guard_position = (pos.0, pos.1, pos.2);
                // pos.4 is 1 when we visited a new field, so increase step_nr
                if pos.4 == 1 {step_nr += 1}
                if pos.3 == last_return + 1 {
                    result += 1;
                    break
                }
                last_return = pos.3
            }
        }
    }

    result
}