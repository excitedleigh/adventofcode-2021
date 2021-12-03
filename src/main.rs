use std::env::args;

mod day1;
mod day2;
mod day3;

fn main() {
    match &args().nth(1).unwrap()[..] {
        "1" => day1::day1(),
        "1a" => day1::day1a(),
        "2" => day2::day2(),
        "2a" => day2::day2a(),
        "3" => day3::day3(),
        "3a" => day3::day3a(),
        _ => panic!("that day hasn't been added yet"),
    }
}
