use shared::{dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(3);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}
const SAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

fn part1() -> u32 {
    let input = &INPUT;

    input
        .lines()
        .map(|line| {
            let (idx, first) =
                line[..line.len() - 1]
                    .chars()
                    .enumerate()
                    .fold((0usize, 0), |acc, (idx, c)| {
                        let digit = c.to_digit(10).unwrap();
                        if digit > acc.1 {
                            (idx, digit)
                        } else {
                            acc
                        }
                    });

            let second = line[idx + 1..].chars().fold(0, |acc, c| {
                let digit = c.to_digit(10).unwrap();
                if digit > acc {
                    digit
                } else {
                    acc
                }
            });
            first * 10 + second
        })
        .sum::<u32>()
}

fn part2() -> u64 {
    // let input = &SAMPLE_INPUT;
    let input = &INPUT;

    let n_digits = 12;

    input
        .lines()
        .map(|line| {
            let (_start, joltage) =
                (0..n_digits)
                    .rev()
                    .fold((0usize, 0u64), |(start, joltage), to_ignore| {
                        let (start, digit) = line[..line.len() - to_ignore]
                            .chars()
                            .enumerate()
                            .skip(start)
                            .fold((0usize, 0), |acc, (idx, c)| {
                                let digit = c.to_digit(10).unwrap();
                                if digit > acc.1 {
                                    (idx, digit)
                                } else {
                                    acc
                                }
                            });
                        (start + 1, joltage * 10 + digit as u64)
                    });
            joltage
        })
        .sum::<u64>()
}
