use shared::{dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(6);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

const SAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

fn part1() -> u64 {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let mut rev_lines = input.lines().rev();

    let instructions = rev_lines.next().unwrap();

    let mut col_ranges = vec![];
    let mut parsed_instructions = vec![];
    let mut first = true;
    let mut range_start = 0;
    for (idx, char) in instructions.chars().enumerate() {
        let instruction = match char {
            '*' => Some(Instruction::Mul),
            '+' => Some(Instruction::Add),
            ' ' => None,
            _ => unreachable!(),
        };
        if let Some(instruction) = instruction {
            parsed_instructions.push(instruction);
            if !first {
                col_ranges.push(range_start..idx);
                range_start = idx;
            }
            first = false;
        }
    }
    col_ranges.push(range_start..instructions.len());

    let mut results = parsed_instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Add => 0u64,
            Instruction::Mul => 1u64,
        })
        .collect::<Vec<_>>();
    for line in rev_lines {
        for ((range, instruction), output) in col_ranges
            .iter()
            .zip(&parsed_instructions)
            .zip(results.iter_mut())
        {
            let subset = line[range.clone()].trim();
            let num = match subset.parse::<u64>() {
                Ok(num) => num,
                Err(err) => {
                    panic!("err: {err} for str {subset}")
                }
            };
            match instruction {
                Instruction::Add => {
                    *output += num;
                }
                Instruction::Mul => {
                    *output *= num;
                }
            }
        }
    }
    results.iter().sum::<u64>()
}

enum Instruction {
    Add,
    Mul,
}

fn part2() -> u64 {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let mut rev_lines = input.lines().rev();

    let instructions = rev_lines.next().unwrap();

    let mut col_ranges = vec![];
    let mut parsed_instructions = vec![];
    let mut first = true;
    let mut range_start = 0;
    for (idx, char) in instructions.chars().enumerate() {
        let instruction = match char {
            '*' => Some(Instruction::Mul),
            '+' => Some(Instruction::Add),
            ' ' => None,
            _ => unreachable!(),
        };
        if let Some(instruction) = instruction {
            parsed_instructions.push(instruction);
            if !first {
                col_ranges.push(range_start..idx - 1);
                range_start = idx;
            }
            first = false;
        }
    }
    col_ranges.push(range_start..instructions.len());

    let mut results = parsed_instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Add => 0u64,
            Instruction::Mul => 1u64,
        })
        .collect::<Vec<_>>();
    let mut sane_repr = col_ranges
        .iter()
        .map(|range| range.clone().map(|_| String::new()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for line in rev_lines.rev() {
        for (range, strings) in col_ranges.iter().zip(sane_repr.iter_mut()) {
            for (idx, string) in range.clone().zip(strings.iter_mut()) {
                string.push_str(&line[idx..=idx]);
            }
        }
    }
    sane_repr
        .into_iter()
        .zip(results.iter_mut())
        .zip(parsed_instructions.iter())
        .for_each(|((strings, output), instruction)| {
            for string in strings {
                let num = string.trim().parse::<u64>().unwrap();
                match instruction {
                    Instruction::Add => {
                        *output += num;
                    }
                    Instruction::Mul => {
                        *output *= num;
                    }
                }
            }
        });
    results.iter().sum::<u64>()
}
