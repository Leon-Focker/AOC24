use crate::utils::load_input;
const DAY: u32 = 15;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let (mut normal_warehouse, mut large_warehouse, mut moves) = parse_input(&lines);

    let result_part1 = part1(&mut normal_warehouse, &mut moves.clone());
    let result_part2 = part2(&mut large_warehouse, &mut moves);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

// Shared data struct for common fields and logic
#[derive(Debug, Clone)]
struct WarehouseData {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    robot: (usize, usize),
}

impl WarehouseData {
    fn new() -> Self {
        WarehouseData { map: Vec::new(), width: 0, height: 0, robot: (0, 0) }
    }
    fn update_size(&mut self) {
        self.width = self.map[0].len();
        self.height = self.map.len();
    }

    fn init_robot(&mut self) {
        for y in 1..self.height {
            for x in 1..self.width {
                if self.map[y][x] == 3 {
                    self.robot = (x, y);
                    return;
                }
            }
        }
    }

    fn gps_sum(&self, look_for: usize) -> usize {
        let mut result = 0;

        for y in 1..self.height {
            for x in 1..self.width {
                if self.map[y][x] == look_for {
                    result += 100 * y + x;
                }
            }
        }

        result
    }
}

// Trait providing shared methods
trait Warehouse {
    #[allow(dead_code)]
    fn data(&self) -> &WarehouseData;
    fn data_mut(&mut self) -> &mut WarehouseData;

    fn update_size(&mut self) {
        self.data_mut().update_size();
    }

    fn init_robot(&mut self) {
        self.data_mut().init_robot();
    }

    fn update(&mut self, moves: &mut Moves) -> bool;
    fn space_behind(&self, mv: &Move, x: usize, y: usize)-> Vec<(usize, usize)>;

    #[allow(dead_code)]
    fn draw(&self) -> () {
        for y in 0..self.data().height {
            println!();
            for x in 0..self.data().width {
                let res = self.data().map[y][x];

                match res {
                    0 => print!("#"),
                    1 => print!("."),
                    2 => print!("O"),
                    4 => print!("["),
                    5 => print!("]"),
                    _ => print!("@"),
                }
            }
        }
        println!();
    }

    fn gps_sum(&self) -> usize;
}

#[derive(Debug, Clone)]
struct NormalWarehouse {
    data: WarehouseData,
}

impl NormalWarehouse {
    fn new() -> Self {
        Self { data: WarehouseData::new() }
    }
}

impl Warehouse for NormalWarehouse {
    fn data(&self) -> &WarehouseData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut WarehouseData {
        &mut self.data
    }

    fn update(&mut self, moves: &mut Moves) -> bool {
        let next_move = moves.next();

        if *next_move == Move(0, 0) {
            false
        } else {
            let (rx, ry) = self.data.robot;
            let (x, y) = ((rx as isize + next_move.0) as usize, (ry as isize + next_move.1) as usize);
            let next_block = self.data.map[y][x];

            match next_block {
                0 => (),
                1 => {
                    self.data.robot = (x, y);
                    self.data.map[y][x] = 3;
                    self.data.map[ry][rx] = 1;
                },
                _ => {
                    let space_behind = self.space_behind(next_move, x, y);
                    if !space_behind.is_empty() {
                        self.data.robot = (x, y);
                        self.data.map[y][x] = 3;
                        self.data.map[ry][rx] = 1;
                        self.data.map[space_behind[0].1][space_behind[0].0] = 2;
                    }
                },
            }
            true
        }
    }

    fn space_behind(&self, mv: &Move, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        loop {
            (x, y) = ((x as isize + mv.0) as usize, (y as isize + mv.1) as usize);
            let block = self.data.map[y][x];

            match block {
                1 => {result.push((x, y)); return result},
                2 => (),
                _ => return result,
            }
        }
    }

    fn gps_sum(&self) -> usize {
        self.data.gps_sum(2)
    }
}

#[derive(Debug)]
struct LargeWarehouse {
    data: WarehouseData,
}

impl LargeWarehouse {
    fn new() -> Self {
        Self { data: WarehouseData::new() }
    }

    fn move_boxes(&mut self, mv: &Move, boxes: Vec<(usize,usize)>) -> () {
        let mut new_map = self.data.map.clone();
        let mut new_locations: Vec<(usize, usize)> = Vec::with_capacity(boxes.len());

        // move boxes to new location
        for bx in boxes.iter().rev() {
            let new_location = ((bx.0 as isize + mv.0) as usize, (bx.1 as isize + mv.1) as usize);
            new_locations.push(new_location);
            new_map[new_location.1][new_location.0] = self.data.map[bx.1][bx.0];
        }

        for bx in boxes.into_iter().filter(|x| !new_locations.contains(x)) {
            new_map[bx.1][bx.0] = 1;
        }

        self.data.map = new_map;
    }
}

impl Warehouse for LargeWarehouse {
    fn data(&self) -> &WarehouseData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut WarehouseData {
        &mut self.data
    }

    fn update(&mut self, moves: &mut Moves) -> bool {
        let next_move = moves.next();

        if *next_move == Move(0, 0) {
            false
        } else {
            let (rx, ry) = self.data.robot;
            let (x, y) = ((rx as isize + next_move.0) as usize, (ry as isize + next_move.1) as usize);
            let next_block = self.data.map[y][x];

            match next_block {
                0 => (),
                1 => {
                    self.data.robot = (x, y);
                    self.data.map[y][x] = 3;
                    self.data.map[ry][rx] = 1;
                },
                _ => {
                    let to_move = self.space_behind(next_move, x, y);

                    if !to_move.is_empty() {
                        self.move_boxes(next_move, to_move);
                        self.data.robot = (x, y);
                        self.data.map[y][x] = 3;
                        self.data.map[ry][rx] = 1;
                    }
                },
            }
            true
        }
    }

    fn space_behind(&self, mv: &Move, x: usize, y: usize) -> Vec<(usize,usize)> {
        let mut result = Vec::new();
        let block = self.data.map[y][x];
        let mut is_blocked= false;

        match block {
            1 => { result.push(((x as isize - mv.0) as usize,(y as isize - mv.1) as usize)); result},
            0 => return Vec::new(),
            _ => {
                result.push((x, y));

                if mv.0 == 0 {
                    let other_half = if block == 4 { (x + 1, y) } else { (x - 1, y) };
                    result.push(other_half);
                }

                for blk in result.clone() {
                    let mut res = self
                        .space_behind(
                            mv,
                            (blk.0 as isize + mv.0) as usize,
                            (blk.1 as isize + mv.1) as usize
                        );

                    if res.is_empty() {
                        is_blocked = true;
                    } else {
                        result.append(&mut res);
                    }
                }
               if is_blocked { Vec::new() } else { result }
            },
        }
    }

    fn gps_sum(&self) -> usize {
        self.data.gps_sum(4)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Move(isize, isize);

// 0 -> up, 1 -> down, 2 -> down, 3 -> left, 4 -> stop
#[derive(Debug, Clone)]
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

fn parse_input(lines: &Vec<String>) -> (NormalWarehouse, LargeWarehouse, Moves) {
    let mut normal_warehouse = NormalWarehouse::new();
    let mut large_warehouse = LargeWarehouse::new();
    let mut moves = Moves::new();
    let mut warehouse_done = false;

    for line in lines {

        if !warehouse_done {
            let mut row = Vec::with_capacity(line.len());
            let mut large_row = Vec::new();

            for char in line.chars() {
                match char {
                    '#' => {
                        row.push(0);
                        large_row.push(0);
                        large_row.push(0);
                    },
                    '.' => {
                        row.push(1);
                        large_row.push(1);
                        large_row.push(1);
                    },
                    'O' => {
                        row.push(2);
                        large_row.push(4);
                        large_row.push(5);
                    },
                    '@' => {
                        row.push(3);
                        large_row.push(3);
                        large_row.push(1);
                    },
                    _ => warehouse_done = true,
                }
            }

            if row.len() > 0 {
                normal_warehouse.data.map.push(row);
                large_warehouse.data.map.push(large_row);
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

    normal_warehouse.update_size();
    normal_warehouse.init_robot();
    large_warehouse.update_size();
    large_warehouse.init_robot();

    (normal_warehouse, large_warehouse, moves)
}


fn part1(normal_warehouse: &mut NormalWarehouse, moves: &mut Moves) -> usize {

    loop {
        //normal_warehouse.draw();
        if !normal_warehouse.update(moves) {
            break
        }
    }

    normal_warehouse.gps_sum()
}

fn part2(large_warehouse: &mut LargeWarehouse, moves: &mut Moves) -> usize {

    loop {
        //large_warehouse.draw();
        if !large_warehouse.update(moves) {
            break
        }
    }

    //large_warehouse.draw();
    large_warehouse.gps_sum()
}