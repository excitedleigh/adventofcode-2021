use either::Either;
use regex::Regex;
use std::io;
use std::io::BufRead;

macro_rules! rrange {
    ($a:expr, $b:expr) => {
        if $a < $b {
            Either::Left($a..=$b)
        } else {
            Either::Right(($b..=$a).rev())
        }
    };
}

pub(crate) fn day5() {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut vent_map = [[0u32; 1000]; 1000];
    for line in stdin_lock.lines().map(|x| x.unwrap()) {
        let capt = re.captures(line.as_str()).unwrap();
        let x1: usize = capt.get(1).unwrap().as_str().parse().unwrap();
        let y1: usize = capt.get(2).unwrap().as_str().parse().unwrap();
        let x2: usize = capt.get(3).unwrap().as_str().parse().unwrap();
        let y2: usize = capt.get(4).unwrap().as_str().parse().unwrap();

        if x1 == x2 {
            // vertical
            for y in rrange!(y1, y2) {
                vent_map[x1][y] += 1;
            }
        } else if y1 == y2 {
            // horizontal
            for x in rrange!(x1, x2) {
                vent_map[x][y1] += 1;
            }
        }
    }

    let mut count = 0u32;
    for x in 0..1000 {
        for y in 0..1000 {
            let value = vent_map[x][y];
            if value >= 2 {
                count += 1;
            }
        }
    }
    println!("{:?}", count);
}

pub(crate) fn day5a() {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut vent_map = [[0u32; 1000]; 1000];
    for line in stdin_lock.lines().map(|x| x.unwrap()) {
        let capt = re.captures(line.as_str()).unwrap();
        let x1: i32 = capt.get(1).unwrap().as_str().parse().unwrap();
        let y1: i32 = capt.get(2).unwrap().as_str().parse().unwrap();
        let x2: i32 = capt.get(3).unwrap().as_str().parse().unwrap();
        let y2: i32 = capt.get(4).unwrap().as_str().parse().unwrap();

        if x1 == x2 {
            // vertical
            for y in rrange!(y1, y2) {
                vent_map[x1 as usize][y as usize] += 1;
            }
        } else if y1 == y2 {
            // horizontal
            for x in rrange!(x1, x2) {
                vent_map[x as usize][y1 as usize] += 1;
            }
        } else if (x2 - x1).abs() == (y2 - y1).abs() {
            for (x, y) in rrange!(x1, x2).zip(rrange!(y1, y2)) {
                vent_map[x as usize][y as usize] += 1;
            }
        }
    }

    let mut count = 0u32;
    for x in 0..1000 {
        for y in 0..1000 {
            let value = vent_map[x][y];
            if value >= 2 {
                count += 1;
            }
        }
    }
    println!("{:?}", count);
}
