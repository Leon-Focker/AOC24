mod days;
mod utils;

fn main() {
    let day = 1;
    let debug = false;
   match day {
        1 => days::day01::run(debug),
        // 2 => days::day02::run(debug),
        _ => println!("The day you requested is not implemented"),
    }
}
