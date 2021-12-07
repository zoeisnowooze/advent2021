fn fuel(crabs: &[u32], position: u32, d: fn(i32, i32) -> i32) -> u32 {
    crabs
        .iter()
        .map(|crab| d(*crab as i32, position as i32))
        .sum::<i32>() as u32
}

fn cheapest_fuel(crabs: &[u32], d: fn(i32, i32) -> i32) -> u32 {
    let position = (0..2000)
        .min_by(|a, b| fuel(crabs, *a, d).cmp(&fuel(crabs, *b, d)))
        .unwrap() as u32;
    fuel(crabs, position, d)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day7.txt");
    let crabs: Vec<u32> = INPUT
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|crab| crab.parse().unwrap())
        .collect();
    println!("solution {}", cheapest_fuel(&crabs, |a, b| (a - b).abs()));
    println!(
        "solution {}",
        cheapest_fuel(&crabs, |a, b| {
            let d = (a - b).abs();
            (d * (d + 1)) / 2
        })
    );
}
