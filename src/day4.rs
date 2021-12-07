use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct BoardEntry {
    number: u16,
    marked: bool,
}

pub(crate) fn day4() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    // read in numbers
    let mut number_str = "".to_owned();
    stdin_lock
        .read_line(&mut number_str)
        .expect("Can't read from stdin");
    let numbers = number_str
        .trim()
        .split(',')
        .map(|n| n.parse::<u16>().expect("Not a number"))
        .collect::<Vec<_>>();
    dbg!(numbers);

    // read past the empty line
    stdin_lock
        .read_line(&mut number_str)
        .expect("Can't read from stdin");

    let boards: Vec<[[BoardEntry; 5]; 5]> = Vec::new();
    // read in boards
    for chunk in stdin_lock.lines().chunks(6).into_iter() {
        let mut board = [[BoardEntry {
            number: 0,
            marked: false,
        }; 5]; 5];
        for (row, line) in chunk.take(5).enumerate() {
            for (col, val) in line
                .expect("Not a valid string")
                .trim()
                .split(char::is_whitespace)
                .filter(|x| !x.is_empty())
                .enumerate()
            {
                board[row][col] = BoardEntry {
                    number: val.parse().expect("Not a number"),
                    marked: false,
                }
            }
        }
        dbg!(board);
    }
}
