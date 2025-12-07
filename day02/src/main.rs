use std::collections::BTreeSet;

use shared::{dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(2);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn part1() -> u64 {
    let input = &INPUT;
    input
        .split(",")
        .flat_map(|sequence| {
            let (start, end) = sequence.split_once("-").unwrap();

            let start_int = start.trim().parse::<u64>().unwrap();
            let end_int = end.trim().parse::<u64>().unwrap();

            let start_len = (start.len() / 2) as u32;
            let end_len = (end.len() / 2) as u32;

            let mut ids = vec![];
            'outer: for len in start_len..=end_len {
                for i in 10u64.pow(len - 1)..10u64.pow(len) {
                    let repeated = i * 10u64.pow(len) + i;
                    if repeated >= start_int {
                        if repeated <= end_int {
                            ids.push(repeated);
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
            ids
        })
        .sum::<u64>()
}

fn part2() -> u64 {
    // let input = &SAMPLE_INPUT;
    let input = &INPUT;
    input
        .split(",")
        .flat_map(|sequence| {
            let sequence = sequence.trim();
            let (start, end) = sequence.split_once("-").unwrap();

            let start_int = start.trim().parse::<u64>().unwrap();
            let end_int = end.trim().parse::<u64>().unwrap();

            let end_len = end.len() as u32;

            let mut ids = BTreeSet::new();
            'rep: for repetition in 2..=end_len + 1 {
                let power = end_len / repetition;

                for i in 10u64.pow(power - 1)..10u64.pow(power) {
                    let mut full_number = 0;
                    for _ in 0..repetition {
                        full_number = full_number * 10u64.pow(power) + i;
                    }
                    if full_number >= start_int {
                        if full_number <= end_int {
                            ids.insert(full_number);
                        } else {
                            continue 'rep;
                        }
                    }
                }
            }
            ids
        })
        .sum::<u64>()
}
