use crate::utils::load_input;
const DAY: u32 = 9;

pub fn run(debug: bool) {
    let lines = load_input(DAY, debug);
    let line = &lines[0];

    // collect digits in a vector
    let old_files: Vec<i32> =
        line.chars()
            .map(|x| x.to_digit(10).expect("char not a digit") as i32)
            .collect();

    let result_part1 = part1(old_files.clone());
    let result_part2 = part2(old_files);

    println!("Day {}, Part 1: {}", DAY, result_part1);
    println!("Day {}, Part 2: {}", DAY, result_part2);
}

fn part1(old_files: Vec<i32>) -> usize {
    let mut old_files_blocks: Vec<i32> = Vec::new();
    let mut nr_blocks = 0;

    // get all blocks explicitly
    for (i, file) in old_files.iter().enumerate() {
        // push as many numbers as the file is long (= the digit of file)
        for _ in 0..*file {
            // if it's actually a file, push the files index
            if i % 2 == 0 {
                nr_blocks += 1;
                old_files_blocks.push((i as f32 / 2.0).floor() as i32)
                // if it's empty space, push -1
            } else {
                old_files_blocks.push(-1);
            }
        }
    }

    // safe files more compact in new_blocks
    let mut new_blocks = Vec::new();

    for i in 0..old_files_blocks.len() {
        // exit, when all files are rearranged
        if new_blocks.len() >= nr_blocks {break}
        // collect files from old_files_blocks (else clause),
        // but when there is space, take from the back
        new_blocks.push(
            if old_files_blocks[i] == -1 {
                loop {
                    let val = old_files_blocks.pop().expect("Vector empty");
                    if val != -1 {
                        break val; // Exit the loop with the value
                    }
                }
            } else {old_files_blocks[i]}
        );
    }

    let mut result = 0;

    // get check sum
    for (i, block) in new_blocks.iter().enumerate() {
        result += i * *block as usize;
    }

    result
}

fn part2(mut old_files: Vec<i32>) -> usize {
    let mut files_with_index: Vec<(i32, u32)> = Vec::new();

    // store files and empty space as tuples, where
    // empty spaces: (- their size, 0)
    // files:        (  their size, their index)
    for (i,file) in old_files.iter_mut().enumerate() {
        if i % 2 == 1 {
            files_with_index.push((-*file, 0));
            // index only files, not empty space
        } else { files_with_index.push((*file, (i as f32 / 2.0).floor() as u32))}
    }

    // we use an index i below, but have to offset it when inserting numbers into the vector
    let mut offset = 0;

    // loop through all files from the end of the vector
    for i in (0..files_with_index.len()).rev() {
        let (value, val_index) = files_with_index[i+offset];
        let mut inserted = false;
        let mut new_order = Vec::new();

        // If it's a file, go through the vector and check for a large enough space
        if value > 0 {
            for (k, &(file, index)) in files_with_index.iter().enumerate() {
                // check whether there is space and need to insert
                if file <= -value && !inserted && k < i+offset {
                    // Insert Value into the empty space
                    new_order.push((value, val_index));
                    // decrease the empty space when value was inserted
                    if file + value < 0 {
                        new_order.push((file + value, 0));
                        offset += 1;
                    }
                    inserted = true;
                    // collect all other elements normally
                } else {
                    new_order.push((file, index));
                }
            }
            // update the Vector (if we moved a value, free up the space it had used)
            files_with_index = new_order;
            if inserted {
                files_with_index[i+offset] = (- files_with_index[i+offset].0, 0)
            }
        }
    }

    let mut new_disk = Vec::new();

    // get all blocks explicitly
    for (file, index) in files_with_index {
        // push as many numbers as the file is long (= the digit of file)
        for _ in 0..file.abs() {
           new_disk.push(index);
        }
    }

    let mut result: usize = 0;

    // get check sum
    for (i, block) in new_disk.iter().enumerate() {
        result += i * *block as usize;
    }

    result
}
