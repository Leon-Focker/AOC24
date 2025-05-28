use std::collections::{HashMap};
use std::collections::hash_map::Entry;
use crate::utils::load_input;
const DAY: u32 = 16;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut maze: Maze = parse_input(&lines);

    let result_part1 = part1(&mut maze);
    let result_part2 = part2(&mut maze);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<usize>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
    reindeer: Reindeer,
    pos_hash: HashMap<(usize, usize), usize>,
    best_reindeer: Vec<Reindeer>,
}

impl Maze {
    fn new() -> Self {
        Maze { map: Vec::new(), height: 0, width: 0, start: (0, 0), end: (0, 0), reindeer: Reindeer::new(), pos_hash: HashMap::new(), best_reindeer: Vec::new() }
    }

    fn update_size(&mut self) {
        self.width = self.map[0].len();
        self.height = self.map.len();
    }

    #[allow(dead_code)]
    fn draw(&self) -> () {
        for y in 0..self.height {
            println!();
            for x in 0..self.width {
                let res = self.map[y][x];

                match res {
                    0 => print!("#"),
                    1 => print!("."),
                    2 => print!("S"),
                    3 => print!("E"),
                    _ => print!("O"),
                }
            }
        }
        println!();
    }

    fn compare_pos_hash(&mut self, position: (usize, usize), score: usize) -> bool {
        match self.pos_hash.entry(position) {
            Entry::Occupied(mut entry) => {
                // this isn't optimal
                if *entry.get() >= score || *entry.get() >= (score - 1000) {
                    *entry.get_mut() = score;
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(_) => {
                self.pos_hash.insert(position, score);
                true
            }
        }
    }

    fn find_path(&mut self) -> () {
        let mut stack = vec![self.reindeer.clone()];
        let mut all_reindeer: Vec<Reindeer> = Vec::new();

        while let Some(mut deer) = stack.pop() {
            deer.set_options(&self);

            for opt in &deer.options {
                let mut new_deer = deer.clone();
                new_deer.move_it(&opt);

                if self.compare_pos_hash(new_deer.position, new_deer.score) {
                    if new_deer.position == self.end {
                        all_reindeer.push(new_deer);
                    } else if !new_deer.options.is_empty() {
                        stack.push(new_deer);
                    }
                }
            }
        }

        if all_reindeer.len() > 0 {
            let mut min = all_reindeer[0].score;

            for deer in all_reindeer {
                if deer.score < min {
                    min = deer.score;
                    self.reindeer = deer.clone();
                    self.best_reindeer = Vec::new();
                    self.best_reindeer.push(deer.clone());
                } else if deer.score == min {
                    self.best_reindeer.push(deer.clone());
                }
            }
        }
    }

    fn mark_visited(&mut self) -> () {
        for deer in &self.best_reindeer {
            for (x, y) in &deer.visited {
                self.map[*y][*x] = 5;
            }
        }
    }

    fn good_seats(&self) -> usize {
        let mut result = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let res = self.map[y][x];

                match res {
                    0 => (),
                    1 => (),
                    _ => result += 1,
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
struct Reindeer {
    position: (usize, usize),
    score: usize,
    last: (isize, isize),
    options: Vec<(isize, isize)>,
    visited: Vec<(usize, usize)>,
}

impl Reindeer {
    fn new() -> Self {
        Reindeer { position: (0, 0), score: 0, last: (1,0), options: Vec::new(), visited: Vec::new() }
    }

    fn move_it(&mut self, mv: &(isize, isize)) -> () {
        self.score += if self.last == *mv { 1 } else { 1001 };
        self.last = *mv;
        self.visited.push(self.position);
        self.position =
            ((self.position.0 as isize + mv.0) as usize,
             (self.position.1 as isize + mv.1) as usize);
    }

    fn set_options(&mut self, maze: &Maze) -> () {
        self.options = Vec::new();
        let up = (self.position.0, self.position.1.saturating_sub(1));
        let down = (self.position.0, self.position.1.saturating_add(1));
        let left = (self.position.0.saturating_sub(1), self.position.1);
        let right = (self.position.0.saturating_add(1), self.position.1);

        if (maze.map[up.1][up.0] == 1 || maze.map[up.1][up.0] == 3) && !self.visited.contains(&up) {
            self.options.push((0, -1))
        }
        if (maze.map[down.1][down.0] == 1 || maze.map[down.1][down.0] == 3) && !self.visited.contains(&down) {
            self.options.push((0, 1))
        }
        if (maze.map[left.1][left.0] == 1 || maze.map[left.1][left.0] == 3) && !self.visited.contains(&left) {
            self.options.push((-1, 0))
        }
        if (maze.map[right.1][right.0] == 1 || maze.map[right.1][right.0] == 3) && !self.visited.contains(&right) {
            self.options.push((1, 0))
        }
    }
}

fn parse_input(lines: &Vec<String>) -> Maze {
    let mut maze: Maze = Maze::new();
    let mut reindeer: Reindeer = Reindeer::new();

    for line in lines {
        let mut row = Vec::with_capacity(line.len());

        for char in line.chars() {
            match char {
                '#' => {
                    row.push(0);
                },
                '.' => {
                    row.push(1);
                },
                'S' => {
                    reindeer.position = (row.len(), maze.map.len());
                    maze.start = (row.len(), maze.map.len());
                    row.push(2);
                },
                'E' => {
                    maze.end = (row.len(), maze.map.len());
                    row.push(3);
                },
                _ => (),
            }
        }

        maze.map.push(row);
    }

    maze.update_size();
    reindeer.set_options(&maze);
    maze.reindeer = reindeer;
    maze
}

fn part1(maze: &mut Maze) -> usize {
    maze.find_path();
    maze.reindeer.score

}

fn part2(maze: &mut Maze) -> usize {
    maze.mark_visited();
    //maze.draw();

    maze.good_seats()
}