use std::collections::{BTreeMap, HashMap, HashSet};

use shared::{char_array, dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(7);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

fn part1() -> usize {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let arr = char_array(input, |c| match c {
        'S' => GridElement::Start,
        '^' => GridElement::Splitter,
        '.' => GridElement::Space,
        _ => unreachable!(),
    });

    let mut sparse = BTreeMap::<usize, Vec<_>>::new();
    let mut start = None;
    arr.indexed_iter()
        .for_each(|((row, col), elem)| match elem {
            GridElement::Start => start = Some(col),
            GridElement::Space => {}
            GridElement::Splitter => {
                sparse.entry(row).or_default().push(col);
            }
        });

    let mut alive = HashSet::from([start.unwrap()]);

    let mut n_splits = 0;
    for (_row, cols) in sparse {
        let new_beams = alive
            .extract_if(|alive_col| cols.contains(&alive_col))
            .flat_map(|hit_beam| {
                n_splits += 1;
                [hit_beam - 1, hit_beam + 1]
            })
            .collect::<Vec<_>>();
        alive.extend(new_beams);
    }
    n_splits
}

enum GridElement {
    Start,
    Space,
    Splitter,
}

fn part2() -> usize {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let arr = char_array(input, |c| match c {
        'S' => GridElement::Start,
        '^' => GridElement::Splitter,
        '.' => GridElement::Space,
        _ => unreachable!(),
    });

    let mut sparse = BTreeMap::<usize, Vec<_>>::new();
    let mut start = None;
    arr.indexed_iter()
        .for_each(|((row, col), elem)| match elem {
            GridElement::Start => start = Some(col),
            GridElement::Space => {}
            GridElement::Splitter => {
                sparse.entry(row).or_default().push(col);
            }
        });

    let mut alive = HashMap::from([(start.unwrap(), 1)]);

    for (_row, cols) in sparse {
        let new_beams = alive
            .extract_if(|alive_col, _count| cols.contains(&alive_col))
            .flat_map(|(hit_col, count)| [(hit_col - 1, count), (hit_col + 1, count)])
            .collect::<Vec<_>>();
        for (new_col, count) in new_beams {
            *alive.entry(new_col).or_default() += count;
        }
    }
    alive.iter().map(|(_, count)| *count).sum()
}
