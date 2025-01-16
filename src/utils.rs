use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader};
use std::io::prelude::*;

pub fn input_into_lines(file_path: &str) -> Vec<String> {
    // create a path from the file_path string
    let path = Path::new(file_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, <dyn Error>::to_string(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

pub fn load_input(day: u32, debug: bool) -> Vec<String> {
    let file_path = if debug {
        format!("src/days/day{:02}/input2.txt", day)
    } else {
        format!("src/days/day{:02}/input.txt", day)
    };

    input_into_lines(&file_path)
}
