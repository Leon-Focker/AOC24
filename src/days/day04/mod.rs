use crate::utils::load_input;
const DAY: u32 = 4;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let result_part1 = part1(&lines);
    let result_part2 = part2(&lines);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut result = 0;
    // get all chars into a matrix (maybe this is stupid)
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        char_matrix.push(line.chars().collect());
    }
    let nr_rows = char_matrix.len();

    for row in 0..nr_rows {
        for index in 0..char_matrix[row].len() {
            // When char is an X, look for surrounding Ms
            if char_matrix[row][index] == 'X' {
                // Look for surrounding Ms
                for row1 in row.saturating_sub(1)..(row+2).min(nr_rows) {
                    for index1 in index.saturating_sub(1)..(index+2).min(char_matrix[row1].len()) {
                        if char_matrix[row1][index1] == 'M' {
                            // When M is found, check for whole XMas, using the direction of the M relative to the X
                            if check_for_xmas(&char_matrix,
                                              (row,index),
                                              index1 as i32 - index as i32,
                                              row1 as i32 - row as i32)
                            {result += 1}
                        }
                    }
                }
            }
        }
    }
    result
}

// Yes, this is ugly as hell, and technically I don't need to check for X and M...
fn check_for_xmas (matrix: &[Vec<char>], position: (usize, usize), x_direction: i32, y_direction: i32) -> bool {
    let nr_rows = matrix.len() as i32;
    let nr_collums = matrix[0].len() as i32;
    if matrix[position.0][position.1] == 'X' {
        let new_position = (position.0 as i32 + y_direction, position.1 as i32 + x_direction);
        if new_position.0 >= 0 && new_position.0 < nr_rows &&
            new_position.1 >= 0 &&  new_position.1 < nr_collums &&
            matrix[new_position.0 as usize][new_position.1 as usize] == 'M' {
            let new_position = (new_position.0 + y_direction, new_position.1 + x_direction);
            if new_position.0 >= 0 && new_position.0 < nr_rows &&
                new_position.1 >= 0 && new_position.1 < nr_collums &&
                matrix[new_position.0 as usize][new_position.1 as usize] == 'A' {
                let new_position = (new_position.0 + y_direction, new_position.1 + x_direction);
                if new_position.0 >= 0 && new_position.0 < nr_rows &&
                    new_position.1 >= 0 && new_position.1 < nr_collums &&
                    matrix[new_position.0 as usize][new_position.1 as usize] == 'S' {
                    return true
                }
            }
        }
    }
    false
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut result = 0;
    // get all chars into a matrix (maybe this is stupid)
    let mut char_matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        char_matrix.push(line.chars().collect());
    }
    let nr_rows = char_matrix.len();

    // Check for As
    for row in 1..(nr_rows-1) {
        for index in 0..char_matrix[row].len() {
            // When char is an A, check for an X-Mas
            if char_matrix[row][index] == 'A' && check_for_x_mas(&char_matrix, (row, index)) {result += 1}
        }
    }

    result
}

// Also ugly as hell and no need to check for A again
fn check_for_x_mas (matrix: &[Vec<char>], position: (usize, usize)) -> bool {
    if matrix[position.0][position.1] == 'A' &&
        (position.0 as i32) > 0 && (position.1 as i32) > 0 &&
        (position.0 as i32)+1 < matrix.len() as i32 && (position.1 as i32)+1 < matrix[0].len() as i32 {
        if matrix[(position.0)-1][(position.1)-1] == 'M' && matrix[(position.0)-1][(position.1)+1] == 'M' &&
            matrix[(position.0)+1][(position.1)-1] == 'S' && matrix[(position.0)+1][(position.1)+1] == 'S'
            ||
            matrix[(position.0)-1][(position.1)-1] == 'M' && matrix[(position.0)-1][(position.1)+1] == 'S' &&
                matrix[(position.0)+1][(position.1)-1] == 'M' && matrix[(position.0)+1][(position.1)+1] == 'S'
            ||
            matrix[(position.0)-1][(position.1)-1] == 'S' && matrix[(position.0)-1][(position.1)+1] == 'S' &&
                matrix[(position.0)+1][(position.1)-1] == 'M' && matrix[(position.0)+1][(position.1)+1] == 'M'
            ||
            matrix[(position.0)-1][(position.1)-1] == 'S' && matrix[(position.0)-1][(position.1)+1] == 'M' &&
                matrix[(position.0)+1][(position.1)-1] == 'S' && matrix[(position.0)+1][(position.1)+1] == 'M' {
            return true
        }
    }
    false
}