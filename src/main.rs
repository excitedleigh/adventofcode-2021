use std::env::args;

mod day1;

fn main() {
    match &args().nth(1).unwrap()[..] {
        "1" => day1::day1(),
        "1a" => day1::day1a(),
        _ => panic!("that day hasn't been added yet"),
    }
}
