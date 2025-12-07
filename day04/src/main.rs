use ndarray::{s, Array2, Zip};
use shared::{dbg_and_time, get_day_input, lazy_input, Lazy};
lazy_input!(4);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

const SAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn part1() -> usize {
    let input = &INPUT;
    // let input = &SAMPLE_INPUT;

    let (rows, cols, data) = input.lines().fold(
        (0, 0, Vec::with_capacity(input.len())),
        |(rows, _cols, mut data), line| {
            let iter = line.chars().map(|char| match char {
                '.' => 0,
                '@' => 1,
                _ => unreachable!(),
            });
            data.extend(iter);
            (rows + 1, line.len(), data)
        },
    );

    let image = Array2::from_shape_vec((rows, cols), data).unwrap();
    let padded_size = (rows + 2, cols + 2);
    let mut padded = Array2::zeros(padded_size);
    padded.slice_mut(s![1..-1, 1..-1]).assign(&image);
    let unpad = s![1..-1, 1..-1];

    let mut output = Array2::<i32>::zeros(padded_size);

    for i_offset in [-1i32, 0, 1] {
        for j_offset in [-1i32, 0, 1] {
            if i_offset == 0 && j_offset == 0 {
                continue;
            }
            let slice = s![
                i_offset + 1..rows as i32 + i_offset + 1,
                j_offset + 1..cols as i32 + j_offset + 1
            ];
            let padded_slice = padded.slice(slice);
            let out_slice = output.slice_mut(s![1..-1, 1..-1]);
            Zip::from(out_slice).and(padded_slice).for_each(|out, inp| {
                *out += *inp;
            });
        }
    }

    output
        .slice(unpad)
        .iter()
        .zip(image.iter())
        .filter(|(count, valid)| **valid == 1 && **count < 4)
        .count()
}

fn part2() -> usize {
    let input = &INPUT;
    // let input = &SAMPLE_INPUT;

    let (rows, cols, data) = input.lines().fold(
        (0, 0, Vec::with_capacity(input.len())),
        |(rows, _cols, mut data), line| {
            let iter = line.chars().map(|char| match char {
                '.' => 0,
                '@' => 1,
                _ => unreachable!(),
            });
            data.extend(iter);
            (rows + 1, line.len(), data)
        },
    );

    let image = Array2::from_shape_vec((rows, cols), data).unwrap();
    let padded_size = (rows + 2, cols + 2);
    let mut padded = Array2::zeros(padded_size);
    padded.slice_mut(s![1..-1, 1..-1]).assign(&image);
    drop(image);
    let unpad = s![1..-1, 1..-1];
    let mut output = Array2::<i32>::zeros(padded_size);

    let mut total_removed = 0;
    loop {
        for i_offset in [-1i32, 0, 1] {
            for j_offset in [-1i32, 0, 1] {
                if i_offset == 0 && j_offset == 0 {
                    continue;
                }
                let slice = s![
                    i_offset + 1..rows as i32 + i_offset + 1,
                    j_offset + 1..cols as i32 + j_offset + 1
                ];
                let padded_slice = padded.slice(slice);
                let out_slice = output.slice_mut(s![1..-1, 1..-1]);
                Zip::from(out_slice).and(padded_slice).for_each(|out, inp| {
                    *out += *inp;
                });
            }
        }

        let mut n_removed = 0;
        Zip::from(output.slice(unpad))
            .and(padded.slice_mut(unpad))
            .for_each(|count, has_roll| {
                if *has_roll == 1 && *count < 4 {
                    *has_roll = 0;
                    n_removed += 1;
                }
            });

        if n_removed == 0 {
            break total_removed;
        }
        total_removed += n_removed;
        output.fill(0);
        // output
        //     .slice(unpad)
        //     .iter()
        //     .zip(image.iter())
        //     .filter(|(count, valid)| **valid == 1 && **count < 4)
        //     .count()
    }
}
