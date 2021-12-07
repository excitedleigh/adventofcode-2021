use std::collections::HashSet;
use std::io;
use std::io::Read;

pub(crate) fn day7() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = "".to_owned();
    stdin_lock.read_to_string(&mut input).unwrap();
    let positions = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().expect("Not a number"))
        .collect::<Vec<_>>();

    let unique_positions = positions.iter().map(|x| *x).collect::<HashSet<_>>();

    let best_position = unique_positions
        .iter()
        .map(|pos| (*pos, positions.iter().map(|x| (x - *pos).abs()).sum()))
        .fold(
            (i32::MAX, i32::MAX),
            |acc, v| if acc.1 < v.1 { acc } else { v },
        );
    dbg!(best_position);
}

fn factorial(v: i32) -> i32 {
    (1..=v).sum()
}

pub(crate) fn day7a() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = "".to_owned();
    stdin_lock.read_to_string(&mut input).unwrap();
    let positions = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().expect("Not a number"))
        .collect::<Vec<_>>();

    let unique_positions = positions.iter().map(|x| *x).collect::<HashSet<_>>();

    let best_position = unique_positions
        .iter()
        .map(|pos| {
            (
                *pos,
                positions.iter().map(|x| factorial((x - *pos).abs())).sum(),
            )
        })
        .fold(
            (i32::MAX, i32::MAX),
            |acc, v| if acc.1 < v.1 { acc } else { v },
        );
    dbg!(best_position);
}
