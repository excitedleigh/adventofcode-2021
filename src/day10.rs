use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Debug)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

pub(crate) fn day10(){
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let score = stdin_lock
        .lines()
        .map(|line| {
            let mut stack = Vec::<Bracket>::new();
            for char in line.expect("can read line").chars() {
                dbg!(char, &stack);
                match char {
                    '(' => stack.push(Bracket::Round),
                    '[' => stack.push(Bracket::Square),
                    '{' => stack.push(Bracket::Curly),
                    '<' => stack.push(Bracket::Angle),
                    ')' => {
                        if let Some(Bracket::Round) = stack.pop() {
                        } else {
                            return 3;
                        }
                    }
                    ']' => {
                        if let Some(Bracket::Square) = stack.pop() {
                        } else {
                            return 57;
                        }
                    }
                    '}' => {
                        if let Some(Bracket::Curly) = stack.pop() {
                        } else {
                            return 1197;
                        }
                    }
                    '>' => {
                        if let Some(Bracket::Angle) = stack.pop() {
                        } else {
                            return 25137;
                        }
                    }
                    v => panic!("don't know what {} is", v),
                }
            }
            0
        })
        .sum::<u32>();
    dbg!(score);
}

pub(crate) fn day10p2() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let scores = stdin_lock
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::<Bracket>::new();
            for char in line.expect("can read line").chars() {
                match (char, stack.last()) {
                    ('(', _) => stack.push(Bracket::Round),
                    ('[', _) => stack.push(Bracket::Square),
                    ('{', _) => stack.push(Bracket::Curly),
                    ('<', _) => stack.push(Bracket::Angle),
                    (')', Some(Bracket::Round)) => {
                        stack.pop();
                    }
                    (']', Some(Bracket::Square)) => {
                        stack.pop();
                    }
                    ('}', Some(Bracket::Curly)) => {
                        stack.pop();
                    }
                    ('>', Some(Bracket::Angle)) => {
                        stack.pop();
                    }
                    _ => return None,
                }
            }
            let score = stack
                .iter()
                .rev()
                .map(|x| match x {
                    Bracket::Round => 1,
                    Bracket::Square => 2,
                    Bracket::Curly => 3,
                    Bracket::Angle => 4,
                })
                .fold(0u64, |acc, x| (acc * 5) + x);
            dbg!(score);
            Some(score)
        })
        .sorted()
        .collect::<Vec<_>>();
    let score = scores[scores.len() / 2];
    dbg!(&scores, score, scores.len(), scores.len() / 2);
}
