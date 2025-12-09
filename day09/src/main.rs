use shared::{dbg_and_time, get_day_input, glam::I64Vec2, lazy_input, Lazy};
lazy_input!(9);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}
const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

fn part1() -> i64 {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            I64Vec2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let mut area = 0;
    for &point0 in &points {
        for &point1 in &points {
            let diff = point1 - point0;
            let new_area = (diff.x.abs() + 1) * (diff.y.abs() + 1);
            if new_area > area {
                area = new_area;
            }
        }
    }
    area
}

fn part2() {}
