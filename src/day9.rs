use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use itertools::Itertools;

pub(crate) fn day9() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let heightmap = read_heightmap(&mut stdin_lock);
    dbg!(heightmap);

    let risk_level: u32 = heightmap
        .iter()
        .enumerate()
        .map::<u32, _>(|(x, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .map(move |(y, v)| {
                    if (clamp(0, x as i16 - 1, 99)..=clamp(0, x as i16 + 1, 99)).all(|cx| {
                        (clamp(0, y as i16 - 1, 99)..=clamp(0, y as i16 + 1, 99)).all(|cy| {
                            (cx as usize == x && cy as usize == y)
                                || heightmap[cx as usize][cy as usize] > v
                        })
                    }) {
                        v as u32 + 1
                    } else {
                        0
                    }
                })
                .sum()
        })
        .sum();
    dbg!(risk_level);
}

fn clamp<N: PartialOrd>(min: N, val: N, max: N) -> N {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn read_heightmap<B: BufRead>(buf: &mut B) -> [[u8; 100]; 100] {
    let mut heightmap = [[0u8; 100]; 100];
    for (y, line) in buf.lines().enumerate() {
        for (x, ch) in line.expect("reading a line failed").chars().enumerate() {
            heightmap[x][y] = ch.to_digit(10).expect("not a digit") as u8;
        }
    }
    heightmap
}

pub(crate) fn day9p2() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let heightmap = read_heightmap(&mut stdin_lock);
    let mut basin_idx = [[0u16; 100]; 100];
    for x in 0..100 {
        for y in 0..100 {
            fill(&heightmap, &mut basin_idx, x, y, (x * 100 + y) as u16);
        }
    }
    let mut count = HashMap::<u16, u32>::new();
    for idx in basin_idx.iter().flatten() {
        count.entry(*idx).and_modify(|x| *x += 1).or_insert(1);
    }
    dbg!(
        &count,
        count
            .values()
            .sorted()
            .rev()
            .skip(1)
            .take(3)
            .collect::<Vec<_>>()
    );
    let prod_largest_3: u32 = count.values().sorted().rev().skip(1).take(3).product();
    dbg!(prod_largest_3);
}

fn fill(
    heightmap: &[[u8; 100]; 100],
    basin_idx: &mut [[u16; 100]; 100],
    x: usize,
    y: usize,
    v: u16,
) {
    if basin_idx[x][y] != 0 {
        return;
    }
    if heightmap[x][y] == 9 {
        return;
    }
    basin_idx[x][y] = v;
    if x > 0 {
        fill(heightmap, basin_idx, x - 1, y, v);
    }
    if x < 99 {
        fill(heightmap, basin_idx, x + 1, y, v);
    }
    if y > 0 {
        fill(heightmap, basin_idx, x, y - 1, v);
    }
    if y < 99 {
        fill(heightmap, basin_idx, 1, y + 1, v);
    }
}
