use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use itertools::Itertools;

// cribbed from https://riptutorial.com/rust/example/4149/create-a-hashset-macro
macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

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

pub(crate) fn day8p2() {
    let ALL_SEGMENTS = set!(0, 1, 2, 3, 4, 5, 6);
    let SEGMENTS_1_LIT = set!(2, 5);
    let SEGMENTS_1_UNLIT = ALL_SEGMENTS
        .difference(&SEGMENTS_1_LIT)
        .map(|x| *x)
        .collect();
    let SEGMENTS_4_LIT = set!(1, 2, 3, 5);
    let SEGMENTS_4_UNLIT = ALL_SEGMENTS
        .difference(&SEGMENTS_4_LIT)
        .map(|x| *x)
        .collect();
    let SEGMENTS_7_LIT = set!(0, 2, 5);
    let SEGMENTS_7_UNLIT = ALL_SEGMENTS
        .difference(&SEGMENTS_7_LIT)
        .map(|x| *x)
        .collect();
    // 0, 6, 9 have a total of 6 lit segments, with 4 in common lit and none definitely unlit
    let LIT_WHEN_6_TOTAL = set!(0, 1, 5, 6);
    let EMPTY = set!();
    // 2, 5 have a total of 5 lit segments, with 3 in common lit
    let LIT_WHEN_5_TOTAL = set!(0, 3, 6);

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

            // this value maps a jumbled segment in the input to all possible actual segments
            let mut segment_map = [
                set!(0u8, 1, 2, 3, 4, 5, 6),
                set!(0, 1, 2, 3, 4, 5, 6),
                set!(0, 1, 2, 3, 4, 5, 6),
                set!(0, 1, 2, 3, 4, 5, 6),
                set!(0, 1, 2, 3, 4, 5, 6),
                set!(0, 1, 2, 3, 4, 5, 6),
                set!(0, 1, 2, 3, 4, 5, 6),
            ];
            let all_digits = all_digits_str
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();

            for digit in all_digits {
                dbg!(digit);
                // get the scrambled segments that are lit
                let scrambled_segments = digit
                    .chars()
                    .map(|c| c.to_segment_num())
                    .collect::<HashSet<_>>();
                // try to guess the actual segments that are lit
                let actual_segments = match digit.len() {
                    2 => Some((&SEGMENTS_1_LIT, &SEGMENTS_1_UNLIT)),
                    4 => Some((&SEGMENTS_4_LIT, &SEGMENTS_4_UNLIT)),
                    3 => Some((&SEGMENTS_7_LIT, &SEGMENTS_7_UNLIT)),
                    6 => Some((&LIT_WHEN_6_TOTAL, &EMPTY)),
                    5 => Some((&LIT_WHEN_5_TOTAL, &EMPTY)),
                    _ => None,
                };
                if let Some((definitely_lit, definitely_unlit)) = actual_segments {
                    for n in 0..7 {
                        if scrambled_segments.contains(&n) {
                            // if scrambled segment n is lit, it cannot be one of the definitely-unlit ones for this count
                            segment_map[n as usize] = segment_map[n as usize]
                                .difference(definitely_unlit)
                                .map(|x| *x)
                                .collect();
                        } else {
                            // if scrambled segment n is unlit, it cannot be one of the definitely-lit ones
                            segment_map[n as usize] = segment_map[n as usize]
                                .difference(definitely_lit)
                                .map(|x| *x)
                                .collect();
                        }
                    }
                }
            }

            let values_we_have_mapped = segment_map
                .iter()
                .filter(|x| x.len() == 1)
                .map(|x| *x.iter().next().expect("empty map with nonzero len"))
                .collect::<HashSet<_>>();
            let final_segment_map = segment_map
                .iter()
                .map(|x| {
                    if x.len() == 1 {
                        *x.iter().next().expect("empty map with nonzero len")
                    } else {
                        *x.iter()
                            .filter(|y| !values_we_have_mapped.contains(y))
                            .next()
                            .expect("we messed up oops")
                    }
                })
                .collect::<Vec<_>>();
            dbg!(
                &line_str,
                segment_map,
                values_we_have_mapped,
                final_segment_map
            );
            0
        })
        .sum();
    dbg!(count_known);
}

trait CharExt {
    fn to_segment_num(&self) -> u8;
}

impl CharExt for char {
    fn to_segment_num(&self) -> u8 {
        (*self as u32 - 0x61) as u8
    }
}
