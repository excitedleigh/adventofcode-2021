use std::io;
use std::io::BufRead;

pub(crate) fn day3() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let bit_count = stdin_lock
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| match c {
                    '1' => 1,
                    '0' => -1,
                    a => panic!("{:?} is not binary", a),
                })
                .collect::<Vec<_>>()
        })
        .fold(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], |acc, val| {
            acc.iter().zip(val.iter()).map(|(a, b)| a + b).collect()
        });
    println!("bit_count={:?}", bit_count);
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in bit_count {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if bit > 0 {
            gamma += 1;
        } else if bit < 0 {
            epsilon += 1;
        } else {
            panic!("one of the bits was exactly zero");
        }
    }
    println!("{:?}", gamma * epsilon);
}

#[derive(Debug)]
struct Candidate {
    val: u32,
    is_o2_candidate: bool,
    is_co2_candidate: bool,
}

pub(crate) fn day3a() {
    /*
    Different tack for this one, converting the values to numbers early and using bitwise ops instead of string manipulation.
    */
    let stdin = io::stdin();
    let mut line_list: Vec<_> = {
        stdin
            .lock()
            .lines()
            .map(|x| Candidate {
                val: u32::from_str_radix(x.unwrap().as_str(), 2).unwrap(),
                is_o2_candidate: true,
                is_co2_candidate: true,
            })
            .collect()
    };
    for n in (0..12).rev() {
        let bit_mask = 1 << n;
        let o2_most_commonly_set = is_most_commonly_set(
            line_list
                .iter()
                .filter(|x| x.is_o2_candidate)
                .map(|x| x.val),
            bit_mask,
        );
        line_list
            .iter_mut()
            .filter(|x| x.is_o2_candidate)
            .for_each(|x| {
                if ((x.val & bit_mask) != 0) != o2_most_commonly_set {
                    x.is_o2_candidate = false
                }
            });
    }
    for n in (0..12).rev() {
        let bit_mask = 1 << n;
        let co2_most_commonly_set = is_most_commonly_set(
            line_list
                .iter()
                .filter(|x| x.is_co2_candidate)
                .map(|x| x.val),
            bit_mask,
        );
        println!("for bit {}, most_commonly_set={}", n, co2_most_commonly_set);
        line_list
            .iter_mut()
            .filter(|x| x.is_co2_candidate)
            .for_each(|x| {
                if ((x.val & bit_mask) != 0) == co2_most_commonly_set {
                    x.is_co2_candidate = false
                }
            });
        // I'm not happy with a lot of this, but this line is the bit I'm least happy with.
        if line_list.iter().filter(|x| x.is_co2_candidate).count() == 1 {
            break;
        }
    }
    let o2 = only(line_list.iter().filter(|x| x.is_o2_candidate)).val;
    let co2 = only(line_list.iter().filter(|x| x.is_co2_candidate)).val;
    println!("o2={:b}, co2={:b}, product={}", o2, co2, o2 * co2);
}

fn is_most_commonly_set<V: Iterator<Item = u32>>(values: V, bit_mask: u32) -> bool {
    let mut most_commonly_set: i32 = 0;
    for value in values {
        if (value & bit_mask) != 0 {
            most_commonly_set += 1
        } else {
            most_commonly_set -= 1
        }
    }
    return most_commonly_set >= 0;
}

fn only<T, I: Iterator<Item = T>>(mut iter: I) -> T {
    let val = iter.next().expect("Iterator was empty");
    if let Some(_) = iter.next() {
        panic!("Iterator had more than one item");
    }
    val
}
