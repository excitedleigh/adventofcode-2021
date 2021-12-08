use std::io::{self, BufRead};

use itertools::Itertools;

pub(crate) fn day8() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let count_known: u32 = stdin_lock
        .lines()
        .map(|line| {
            let line_str = line.expect("line could not be read");
            let (all_digits_str, value_str) = line_str
                .splitn(2, '|')
                .collect_tuple()
                .expect("line contains no pipe");
            value_str
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| match x.len() {
                    2 | 4 | 3 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum();
    dbg!(count_known);
}
