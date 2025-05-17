use crate::utils::load_input;
const DAY: u32 = 15;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let (mut warehouse, mut moves) = parse_input(&lines);

    let result_part1 = part1(&mut warehouse, &mut moves);
    let result_part2 = part2(&mut warehouse, &mut moves);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

// 0 -> wall, 1 -> none, 2 -> box, 3 -> robot
#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    robot: (usize, usize),
}

impl Warehouse {
    fn new() -> Self {
        Warehouse { map: Vec::new(), width: 0, height: 0, robot: (0, 0) }
    }

    fn update_size(&mut self) -> () {
        self.width = self.map[0].len();
        self.height = self.map.len();
    }

    fn set_robot(&mut self) -> () {
        for y in 1..self.height {
            for x in 1..self.width {
                if self.map[y][x] == 3 {
                    self.robot = (x, y);
                    break
                }
            }
        }
    }

    fn update(&mut self, moves: &mut Moves) -> bool {
        let next_move = moves.next();

        if *next_move == Move(0, 0) {
            false
        } else {
            let (rx, ry) = self.robot;
            let (x, y) = ((rx as isize + next_move.0) as usize, (ry as isize + next_move.1) as usize);
            let next_block = self.map[y][x];

            match next_block {
                0 => (),
                1 => {
                    self.robot = (x, y);
                    self.map[y][x] = 3;
                    self.map[ry][rx] = 1;
                },
                _ => {
                    let (space_x, space_y) = self.space_behind(next_move, (x, y));
                    if (space_x, space_y) != (0, 0) {
                        self.robot = (x, y);
                        self.map[y][x] = 3;
                        self.map[ry][rx] = 1;
                        self.map[space_y][space_x] = 2;
                    }
                },
            }
            true
        }
    }

    fn space_behind(&self, mv: &Move, (mut x, mut y): (usize, usize)) -> (usize, usize) {
        loop {
            (x, y) = ((x as isize + mv.0) as usize, (y as isize + mv.1) as usize);
            let block = self.map[y][x];

            match block {
                1 => return (x, y),
                2 => (),
                _ => return (0, 0),
            }
        }
    }

    fn gps_sum(&self) -> usize {
        let mut result = 0;

        for y in 1..self.height {
            for x in 1..self.width {
                if self.map[y][x] == 2 {
                    result += 100 * y + x;
                }
            }
        }

         result
    }

    fn draw(&self) -> () {
        for y in 0..self.height {
            println!();
            for x in 0..self.width {
                let res = self.map[y][x];

                match res {
                    0 => print!("#"),
                    1 => print!("."),
                    2 => print!("O"),
                    _ => print!("@"),
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Move(isize, isize);

// 0 -> up, 1 -> down, 2 -> down, 3 -> left, 4 -> stop
struct Moves {
    moves: Vec<Move>,
    current: isize,
}

impl Moves {
    fn new() -> Self {
        Moves {moves: Vec::new(), current: -1}
    }

    fn next(&mut self) -> &Move {
        if self.current + 1 >= self.moves.len() as isize {
            &Move(0, 0)
        } else {
            self.current += 1;
            &self.moves[self.current as usize]
        }
    }
}

fn parse_input(lines: &Vec<String>) -> (Warehouse, Moves) {
    let mut warehouse = Warehouse::new();
    let mut moves = Moves::new();
    let mut warehouse_done = false;

    for line in lines {

        if !warehouse_done {
            let mut row = Vec::with_capacity(line.len());

            for char in line.chars() {
                match char {
                    '#' => row.push(0),
                    '.' => row.push(1),
                    'O' => row.push(2),
                    '@' => row.push(3),
                    _ => warehouse_done = true,
                }
            }

            if row.len() > 0 {
                warehouse.map.push(row);
            }
        }
        if warehouse_done {
            let mut row = Vec::with_capacity(line.len());

            for char in line.chars() {
                match char {
                    '^' => row.push(Move(0, -1)),
                    '>' => row.push(Move(1, 0)),
                    'v' => row.push(Move(0, 1)),
                    '<' => row.push(Move(-1, 0)),
                    _ => (),
                }
            }

            if row.len() > 0 {
                moves.moves.append(&mut row);
            }
        }
    }

    warehouse.update_size();
    warehouse.set_robot();

    (warehouse, moves)
}


fn part1(warehouse: &mut Warehouse, moves: &mut Moves) -> usize {

    loop {
        //warehouse.draw();
        if !warehouse.update(moves) {
            break
        }
    }

    warehouse.gps_sum()
}

fn part2(_warehouse: &mut Warehouse, _moves: &mut Moves) -> usize {
    let result = 0;

    result
}