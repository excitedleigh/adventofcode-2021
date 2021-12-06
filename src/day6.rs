use std::io;
use std::io::Read;

pub(crate) fn day6() {
    // day 6 and 6a are the same code
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut lanternfish_by_day = [0u64; 9];
    // read in current state
    let mut input = "".to_owned();
    stdin_lock.read_to_string(&mut input).unwrap();
    for num in input.split(',') {
        match num.trim().parse::<usize>() {
            Ok(v) => lanternfish_by_day[v] += 1,
            Err(_) => {}
        }
    }
    dbg!(lanternfish_by_day);
    for day in 0..256 {
        let reproducing_fish = lanternfish_by_day[0];
        for n in 1..9 {
            lanternfish_by_day[n - 1] = lanternfish_by_day[n];
        }
        lanternfish_by_day[8] = reproducing_fish;
        lanternfish_by_day[6] += reproducing_fish;
        dbg!(day, lanternfish_by_day);
    }
    dbg!(lanternfish_by_day.iter().sum::<u64>());
}
