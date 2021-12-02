use std::env::args;

mod day1;
mod day2;

fn main() {
    match &args().nth(1).unwrap()[..] {
        "1" => day1::day1(),
        "1a" => day1::day1a(),
        "2" => day2::day2(),
        "2a" => day2::day2a(),
        _ => panic!("that day hasn't been added yet"),
    }
}
