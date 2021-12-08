use std::io::{self, BufRead};

fn increases(measurements: &[u64]) -> u64 {
    measurements
        .windows(2)
        .fold(0, |acc, x| if x[1] > x[0] { acc + 1 } else { acc })
}

fn convolve(measurements: &[u64]) -> Vec<u64> {
    measurements.windows(3).map(|x| x.iter().sum()).collect()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day1.txt");
    let measurements: Vec<u64> = io::Cursor::new(INPUT)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("increased {} times", increases(&measurements));
    println!("increased {} times", increases(&convolve(&measurements)));
}

#[test]
fn part1_example() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(increases(&measurements), 7);
}

#[test]
fn part2_example() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(increases(&convolve(&measurements)), 5);
}
