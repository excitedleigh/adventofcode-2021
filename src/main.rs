use std::{env::args, error::Error};

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn Error>> {
    match &args().nth(1).unwrap()[..] {
        "1" => day1::day1(),
        "1a" => day1::day1a(),
        "2" => day2::day2(),
        "2a" => day2::day2a(),
        "3" => day3::day3(),
        "3a" => day3::day3a(),
        "4" => day4::day4(),
        "5" => day5::day5(),
        "5a" => day5::day5a(),
        "6" => day6::day6(),
        "7" => day7::day7(),
        "7a" => day7::day7a(),
        "8" => day8::day8(),
        "8p2" => day8::day8p2(),
        "9" => day9::day9(),
        "9p2" => day9::day9p2(),
        "10" => day10::day10(),
        "10p2" => day10::day10p2(),
        _ => panic!("that day hasn't been added yet"),
    }
    Ok(())
}
