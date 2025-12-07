pub use glam;
pub use ndarray;
pub use once_cell::sync::Lazy;
pub use rayon;

use ndarray::{Array, Array2};

const YEAR: usize = 2025;

fn get_session() -> String {
    format!("session={}", env!("AOC_SESSION"))
}

pub fn get_day_input(day: usize) -> String {
    match std::fs::read_to_string("input.txt") {
        Ok(output) => return output,
        Err(_) => {}
    }

    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input")
        .parse::<reqwest::Url>()
        .unwrap();
    let client = reqwest::Client::new();
    let tokio_rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    let response = tokio_rt.block_on(async {
        client
            .get(url)
            .header("cookie", get_session())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });
    std::fs::write("input.txt", &response).unwrap();
    response
}

#[macro_export]
macro_rules! lazy_input {
    ($day:expr) => {
        static INPUT: Lazy<String> = Lazy::new(|| get_day_input($day));
    };
}

pub fn char_array<F: Fn(char) -> O, O>(inp: &str, func: F) -> Array2<O> {
    let (first_row, _) = inp.split_once('\n').unwrap();
    let cols = first_row.len();
    let mut rows = 0;
    let char_vec = inp
        .lines()
        .flat_map(|line| {
            rows += 1;
            line.chars().map(|char| func(char))
        })
        .collect::<Vec<_>>();
    let char_arr = Array::from_shape_vec((rows, cols), char_vec).unwrap();
    char_arr
}

#[macro_export]
macro_rules! dbg_and_time {
    ($inp:expr) => {
        {
            let now = std::time::Instant::now();
            let out = $inp;
            dbg!(now.elapsed());
            dbg!(out)
        }
    };
}

