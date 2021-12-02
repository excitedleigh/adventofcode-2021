use itertools::Itertools;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Position {
    horiz: i32,
    depth: i32,
}

#[derive(Debug)]
struct Position2 {
    horiz: i32,
    depth: i32,
    aim: i32,
}

pub(crate) fn day2() {
    let stdin = io::stdin();
    {
        let handle = stdin.lock();
        let position = handle
            .lines()
            .map(|x| {
                let stri = x.unwrap();
                let (cmd, amount) = stri.splitn(2, " ").collect_tuple().unwrap();
                (cmd.to_owned(), amount.parse::<i32>().unwrap())
            })
            .fold(Position { horiz: 0, depth: 0 }, |p, (cmd, n)| {
                match cmd.as_str() {
                    "forward" => Position {
                        horiz: p.horiz + n,
                        depth: p.depth,
                    },
                    "down" => Position {
                        horiz: p.horiz,
                        depth: p.depth + n,
                    },
                    "up" => Position {
                        horiz: p.horiz,
                        depth: p.depth - n,
                    },
                    _ => panic!("didn't understand command"),
                }
            });
        println!("position: {:?}", position);
    }
}

pub(crate) fn day2a() {
    let stdin = io::stdin();
    {
        let handle = stdin.lock();
        let position = handle
            .lines()
            .map(|x| {
                // this bit is largely the result of fighting the borrow checker :(
                let stri = x.unwrap();
                let (cmd, amount) = stri.splitn(2, " ").collect_tuple().unwrap();
                (cmd.to_owned(), amount.parse::<i32>().unwrap())
            })
            .fold(
                Position2 {
                    horiz: 0,
                    depth: 0,
                    aim: 0,
                },
                |p, (cmd, n)| match cmd.as_str() {
                    "forward" => Position2 {
                        horiz: p.horiz + n,
                        depth: p.depth + p.aim * n,
                        aim: p.aim,
                    },
                    "down" => Position2 {
                        horiz: p.horiz,
                        depth: p.depth,
                        aim: p.aim + n,
                    },
                    "up" => Position2 {
                        horiz: p.horiz,
                        depth: p.depth,
                        aim: p.aim - n,
                    },
                    _ => panic!("didn't understand command"),
                },
            );
        println!("position: {:?}", position);
    }
}
