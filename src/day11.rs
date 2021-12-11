use std::collections::HashSet;

fn square(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut sq = Vec::new();
    if y > 0 {
        if x > 0 {
            sq.push((x - 1, y - 1))
        }
        sq.push((x, y - 1));
        if x < 9 {
            sq.push((x + 1, y - 1))
        }
    }
    if x > 0 {
        sq.push((x - 1, y))
    }
    if x < 9 {
        sq.push((x + 1, y))
    }
    if y < 9 {
        if x > 0 {
            sq.push((x - 1, y + 1))
        }
        sq.push((x, y + 1));
        if x < 9 {
            sq.push((x + 1, y + 1))
        }
    }
    sq
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    let mut flashed = HashSet::new();

    loop {
        let mut new_flashes = false;
        for (y, row) in grid.iter().enumerate() {
            for (x, octopus) in row.iter().enumerate() {
                let adjacent = square(x, y)
                    .iter()
                    .filter(|(xx, yy)| flashed.contains(&(*xx, *yy)))
                    .count() as u32;
                if (octopus + adjacent + 1) >= 10 && !flashed.contains(&(x, y)) {
                    flashed.insert((x, y));
                    new_flashes = true;
                }
            }
        }
        if !new_flashes {
            break;
        }
    }

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, octopus) in row.iter_mut().enumerate() {
            *octopus = if flashed.contains(&(x, y)) {
                0
            } else {
                *octopus
                    + square(x, y)
                        .iter()
                        .filter(|(xx, yy)| flashed.contains(&(*xx, *yy)))
                        .count() as u32
                    + 1
            };
        }
    }

    flashed.len()
}

fn total_flashes(input: &str) -> usize {
    let mut octopuses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    (0..100).fold(0, |acc, _| acc + step(&mut octopuses))
}

fn first_simultaneous_step(input: &str) -> usize {
    let mut octopuses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut s = 0;
    loop {
        s += 1;
        if step(&mut octopuses) == 100 {
            break;
        }
    }
    s
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day11.txt");
    println!("solution {}", total_flashes(INPUT));
    println!("solution {}", first_simultaneous_step(INPUT));
}

#[test]
fn test_square() {
    assert_eq!(square(0, 0), vec![(1, 0), (0, 1), (1, 1)]);
}
