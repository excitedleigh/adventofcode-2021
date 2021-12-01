use itertools::Itertools;
use std::io;
use std::io::BufRead;

pub(crate) fn day1() {
    let stdin = io::stdin();
    {
        let handle = stdin.lock();
        let count = handle
            .lines()
            .map(|x| x.unwrap().parse::<i32>().unwrap())
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count();
        println!("count: {}", count);
    }
}

pub(crate) fn day1a() {
    let stdin = io::stdin();
    {
        let handle = stdin.lock();
        let count = handle
            .lines()
            .map(|x| x.unwrap().parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count();
        println!("count: {}", count);
    }
}
