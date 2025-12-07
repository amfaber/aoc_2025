use std::collections::{BTreeSet, HashSet};

use shared::{dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(5);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

const SAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn part1() -> usize {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.trim().split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
        .count()
}

fn part2() -> u64 {
    let input = &INPUT;

    let (ranges, _ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.trim().split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let event_line = ranges
        .iter()
        .enumerate()
        .flat_map(|(idx, range)| {
            [
                (range.start(), PointKind::Start),
                (range.end(), PointKind::End),
            ]
            .into_iter()
            .map(move |(key, kind)| ((*key, kind, idx)))
        })
        .collect::<BTreeSet<_>>();

    let mut canonicalized_ranges = Vec::new();
    let mut currently_open = HashSet::new();
    let mut current_start = None;

    for (value, kind, idx) in event_line.iter() {
        if currently_open.is_empty() {
            assert_eq!(*kind, PointKind::Start);
            currently_open.insert(idx);
            current_start = Some(value);
        }
        match kind {
            PointKind::Start => {
                currently_open.insert(idx);
            }
            PointKind::End => {
                currently_open.remove(idx);
                if currently_open.is_empty() {
                    canonicalized_ranges.push(current_start.unwrap()..=value);
                    current_start = None;
                }
            }
        }
    }
    assert!(currently_open.is_empty());
    assert!(current_start.is_none());
    canonicalized_ranges
        .iter()
        .map(|range| **range.end() - **range.start() + 1)
        .sum::<u64>()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum PointKind {
    Start,
    End,
}
