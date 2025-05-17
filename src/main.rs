mod days;
mod utils;

fn main() {
    let day = 15;
    let debug = false;
    match day {
        1 => days::day01::run(debug),
        2 => days::day02::run(debug),
        3 => days::day03::run(debug),
        4 => days::day04::run(debug),
        5 => days::day05::run(debug),
        6 => days::day06::run(debug),
        7 => days::day07::run(debug),
        8 => days::day08::run(debug),
        9 => days::day09::run(debug),
        10 => days::day10::run(debug),
        11 => days::day11::run(debug),
        12 => days::day12::run(debug),
        13 => days::day13::run(debug),
        14 => days::day14::run(debug),
        15 => days::day15::run(debug),
        _ => println!("The day you requested is not implemented"),
    }
}
