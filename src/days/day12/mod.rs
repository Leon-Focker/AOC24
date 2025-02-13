use crate::utils::load_input;
const DAY: u32 = 12;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let mut garden_map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        garden_map.push(line.chars().collect())
    }

    let result_part1 = part1(&garden_map);
    let result_part2 = part2(&garden_map);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

#[derive(Debug)]
struct Region {
    plant: char,
    coordinates: Vec<(usize, usize)>,
}

impl Region {

    fn new(plant: char, coord: (usize, usize)) -> Self {
        Region { plant, coordinates: vec![coord]}
    }

    fn get_area(&self) -> usize {
        self.coordinates.len()
    }

    fn get_perimeter(&self) -> usize {
        let mut result = 0;

        for (x, y) in &self.coordinates {
            // Check the four neighbors
            // (potentially false positives when a coordinate contains usize::MAX, but who cares...)
            if !self.coordinates.contains(&(x.wrapping_sub(1), *y)) { result += 1 }
            if !self.coordinates.contains(&(x + 1, *y)) { result += 1 }
            if !self.coordinates.contains(&(*x, y + 1)) { result += 1 }
            if !self.coordinates.contains(&(*x, y.wrapping_sub(1))) { result += 1 }
        }

        result
    }

    fn get_number_of_sides(&self) -> usize {
        let mut result = 0;

        let min_x = *self.coordinates.iter().map(|(x, _)| x).min().unwrap();
        let max_x = *self.coordinates.iter().map(|(x, _)| x).max().unwrap();
        let min_y = *self.coordinates.iter().map(|(_, y)| y).min().unwrap();
        let max_y = *self.coordinates.iter().map(|(_, y)| y).max().unwrap();

        // look from top to bottom for sides above or below
        for y in min_y..=max_y {
            let mut top_found = false;
            let mut btm_found = false;

            for x in min_x..=max_x {

                if self.coordinates.contains(&(x, y)) {
                    if !self.coordinates.contains(&(x, y.wrapping_sub(1))) {
                        top_found = true;
                    } else if top_found {
                        result += 1;
                        top_found = false;
                    }
                    if !self.coordinates.contains(&(x, y.wrapping_add(1))) {
                        btm_found = true;
                    } else if btm_found{
                        result += 1;
                        btm_found = false;
                    }
                } else {
                    if top_found {
                        result += 1;
                        top_found = false;
                    }
                    if btm_found {
                        result += 1;
                        btm_found = false;
                    }
                }
            }

            // whether we found 0, 1 or 2 sides
            result += [top_found, btm_found].iter().filter(|&&x| x).count();
        }

        // same as above but from left to right
        for x in min_x..=max_x {
            let mut left_found = false;
            let mut right_found = false;

            for y in min_y..=max_y {

                if self.coordinates.contains(&(x, y)) {
                    if !self.coordinates.contains(&(x.wrapping_sub(1), y)) {
                        left_found = true;
                    } else if left_found {
                        result += 1;
                        left_found = false;
                    }
                    if !self.coordinates.contains(&(x.wrapping_add(1), y)) {
                        right_found = true;
                    } else if right_found {
                        result += 1;
                        right_found = false;
                    }
                } else {
                    if right_found {
                        result += 1;
                        right_found = false;
                    }
                    if left_found {
                        result += 1;
                        left_found = false;
                    }
                }
            }

            // whether we found 0, 1 or 2 sides
            result += [left_found, right_found].iter().filter(|&&x| x).count();
        }

        result
    }

    fn price(&self) -> usize {
        let area = self.get_area();
        let perimeter = self.get_perimeter();

        // the price:
        area * perimeter
    }

    fn discounted_price(&self) -> usize {
        let area = self.get_area();
        let nr_of_sides = self.get_number_of_sides();

        // the price:
        area * nr_of_sides
    }

    fn is_adjacent(&self, (coord_x, coord_y): (usize, usize)) -> bool {
        // Look for coordinates adjacent to (coord_x, coord_y)
        self.coordinates.iter().any(|&(x, y)| {
            (x == coord_x && (y == coord_y + 1 || y + 1 == coord_y)) || // vertically adjacent
                (y == coord_y && (x == coord_x + 1 || x + 1 == coord_x))    // horizontally adjacent
        })
    }

}

fn split_map_into_regions(map: &[Vec<char>]) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();

    // Loop through all coordinates
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            // collect references to regions with same plant that are adjacent here:
            let mut adjacent_regions: Vec<(usize, &mut Region)> = Vec::new();

            for (idx, with_same_plant) in regions
                .iter_mut()
                .enumerate()
                .filter(| (_, reg) | reg.plant == *char) {
                if with_same_plant.is_adjacent((x, y)) {
                    adjacent_regions.push((idx, with_same_plant));
                }
            }

            match adjacent_regions.len() {
                0 => regions.push(Region::new(*char, (x, y))), // create new Region
                1 => adjacent_regions[0].1.coordinates.push((x, y)), // add coord to this Region
                _ => {
                    // Merge adjacent regions and remove old ones from regions
                    let mut merged_coordinates: Vec<(usize, usize)> = Vec::new();
                    let mut old_indices: Vec<usize> = Vec::with_capacity(adjacent_regions.len());

                    // collect all coordinates and indices of adjacent regions
                    for (idx, region) in adjacent_regions {
                        merged_coordinates.append(&mut region.coordinates);
                        old_indices.push(idx);
                    }

                    // add current coordinate to the merged coordinates
                    merged_coordinates.push((x, y));

                    // remove these regions from regions
                    for idx in old_indices.iter().rev() {
                        regions.remove(*idx);
                    }

                    // and add the merged region
                    regions.push(Region {
                        plant: *char,
                        coordinates: merged_coordinates,
                    })
                },
            }
        }
    }

    regions
}

fn part1(map: &[Vec<char>]) -> usize {
    let regions: Vec<Region> = split_map_into_regions(map);

    regions.iter().map(|region| region.price()).sum()
}

fn part2(map: &[Vec<char>]) -> usize {
    let regions: Vec<Region> = split_map_into_regions(map);

    regions.iter().map(|region| region.discounted_price()).sum()
}