use std::collections::{HashMap, HashSet};
use crate::utils::load_input;
const DAY: u32 = 8;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    
    let mut antenna_map = HashMap::new();
    let max_y = lines.len();
    let max_x = lines[0].len();

    // collect all coordinates for antennas, sorted by their frequency
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antenna_map.entry(char).or_insert_with(Vec::new).push((x, y));
            }
        }
    }
    
    let result_part1 = part1(antenna_map.clone(), max_x, max_y);
    let result_part2 = part2(antenna_map, max_x, max_y);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(antenna_map: HashMap<char, Vec<(usize, usize)>>, max_x: usize, max_y: usize) -> usize {

    // get all antinodes without duplicates
    let mut result = HashSet::new();
    for (_, coordinates) in antenna_map.iter() {
        for coord in produce_antinodes(coordinates) {
            if coord.0 < max_x && coord.1 < max_y {
                result.insert(coord);
            }
        }
    }

    result.len()
}

fn produce_antinodes(antennas: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (x1, y1) = antennas[i];
            let (x2, y2) = antennas[j];
            let x_diff: isize = (x1 as isize)-(x2 as isize);
            let y_diff: isize = (y1 as isize)-(y2 as isize);

            result.push(((x1 as isize + x_diff) as usize, (y1 as isize + y_diff) as usize));
            result.push(((x2 as isize - x_diff) as usize, (y2 as isize - y_diff) as usize));
        }
    }

    result
}

fn produce_harmonic_antinodes(antennas: &[(usize, usize)], max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (x1, y1) = antennas[i];
            let (x2, y2) = antennas[j];
            let x_diff: isize = (x1 as isize)-(x2 as isize);
            let y_diff: isize = (y1 as isize)-(y2 as isize);

            // find all harmonic antinodes by multiplying the difference (wavelength) by n
            for n in 0.. {
                let new_x = (x1 as isize + x_diff * n) as usize;
                let new_y = (y1 as isize + y_diff * n) as usize;

                // break loop when we leave the map
                if new_x >= max_x || new_y >= max_y {break}

                result.push((new_x, new_y));
            }

            // same for the other direction
            for n in 1.. {
                let new_x = (x1 as isize - x_diff * n) as usize;
                let new_y = (y1 as isize - y_diff * n) as usize;

                if new_x >= max_x || new_y >= max_y {break}

                result.push((new_x, new_y));
            }
        }
    }

    result
}

fn part2(antenna_map: HashMap<char, Vec<(usize, usize)>>, max_x: usize, max_y: usize) -> usize {

    // get all antinodes without duplicates
    let mut result = HashSet::new();
    for (_, coordinates) in antenna_map.iter() {
        for coord in produce_harmonic_antinodes(coordinates, max_x, max_y) {
            if coord.0 < max_x && coord.1 < max_y {
                result.insert(coord);
            }
        }
    }

    result.len()
}