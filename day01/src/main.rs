use shared::{dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(1);

fn parse_direction(inp: &str) -> i32 {
    match inp {
        "L" => -1,
        "R" => 1,
        _ => panic!("invalid input"),
    }
}

static SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

fn part1() -> usize {
    let input = &INPUT;
    let result = input
        .lines()
        .scan(50, |state, line| {
            let (direction, amount) = line.split_at(1);
            let dir = parse_direction(direction);
            let amount = amount.parse::<i32>().unwrap();

            let turn = dir * amount;
            *state = (*state + turn) % 100;
            Some(*state)
        })
        .filter(|state| *state == 0)
        .count();

    result
}

fn part2() -> usize {
    let input = &INPUT;
    let result = input
        .lines()
        .flat_map(|line| {
            let (direction, amount) = line.split_at(1);
            let dir = parse_direction(direction);
            let amount = amount.parse::<i32>().unwrap();
            (0..amount).map(move |_| dir)
        })
        .scan(50i32, |state, step| {
            *state = (*state + step).rem_euclid(100);
            Some(*state)
        })
        .filter(|&state| state == 0)
        .count();

    result
}
