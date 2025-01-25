use std::collections::HashSet;
use crate::utils::load_input;
const DAY: u32 = 10;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut topographic_map: Vec<Vec<usize>> = Vec::new();
    let mut trailheads = Vec::new();

    // collect all digits in the topographic_map and retrieve trailheads
    for (y, line) in lines.iter().enumerate() {
        // collect row of digits
        let row: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        // select trailheads
        for (x, &value) in row.iter().enumerate() {
            if value == 0 {
                trailheads.push((x, y));
            }
        }
        topographic_map.push(row);
    }

    let result_part1 = part1(&trailheads, &topographic_map);
    let result_part2 = part2(&trailheads, &topographic_map);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(trailheads: &Vec<(usize, usize)>, topographic_map: &Vec<Vec<usize>>)-> usize {
    let mut result = 0;

    // score all trailheads and sum result
    for trailhead in trailheads {
        result += get_trailhead_score(*trailhead, &topographic_map);
    }

    result
}

fn part2(trailheads: &Vec<(usize, usize)>, topographic_map: &Vec<Vec<usize>>)-> usize {
    let mut result = 0;

    // score all trailheads and sum result
    for trailhead in trailheads {
        result += get_other_trailhead_score(*trailhead, &topographic_map);
    }

    result
}

fn get_trailhead_score ((x, y): (usize, usize), topographic_map: &Vec<Vec<usize>>) -> usize {
    let mut endpoints: HashSet<(usize, usize)> = HashSet::new();

    // go through neighbours of trailhead and push endpoints to HashSet
    if topographic_map[y][x] == 0 {
        endpoints.extend(find_paths((x, y), topographic_map))
    }

    endpoints.len()
}

// essentially the same as for part one, except we do not get rid of duplicates of endpoints
fn get_other_trailhead_score ((x, y): (usize, usize), topographic_map: &Vec<Vec<usize>>) -> usize {
    let mut endpoints: Vec<(usize, usize)> = Vec::new();

    // go through neighbours of trailhead and push endpoints to HashSet
    if topographic_map[y][x] == 0 {
        endpoints.extend(find_paths((x, y), topographic_map))
    }

    endpoints.len()
}


fn find_paths ((x, y): (usize, usize), topographic_map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let current = topographic_map[y][x];
    // yay, we found an end to a trail!
    if current == 9 {
        return vec![(x, y)]
        // keep looking
    } else {
        let mut result = Vec::new();
        // get coordinates of all 4 surrounding positions
        let top = (x, y.saturating_sub(1));
        let right = ((x+1).min(topographic_map[x].len() - 1), y);
        let bottom = (x, (y+1).min(topographic_map.len() - 1));
        let left = (x.saturating_sub(1), y);

        // check whether trail continues
        for (x, y) in vec![top, right, bottom, left] {
            if topographic_map[y][x] == current + 1 {
                result.extend(find_paths((x, y), topographic_map))
            }
        }
        result
    }
}