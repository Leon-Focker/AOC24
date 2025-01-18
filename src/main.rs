mod days;
mod utils;

fn main() {
    let day = 6;
    let debug = false;
    match day {
        1 => days::day01::run(debug),
        2 => days::day02::run(debug),
        3 => days::day03::run(debug),
        4 => days::day04::run(debug),
        5 => days::day05::run(debug),
        6 => days::day06::run(debug),
        _ => println!("The day you requested is not implemented"),
    }
}
