use std::collections::HashSet;

fn risk_level(low_point: u32, neighbors: &[u32]) -> u32 {
    if neighbors.iter().all(|p| *p > low_point) {
        low_point + 1
    } else {
        0
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day9.txt");
    println!("solution: {}", risk(INPUT));
    println!("solution: {}", largest_basins(INPUT));
}

fn neighbors(x: usize, y: usize, row: &[u32], map: &[Vec<u32>]) -> Vec<u32> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push(row[x - 1])
    }
    if x < row.len() - 1 {
        neighbors.push(row[x + 1])
    }
    if y > 0 {
        neighbors.push(map[y - 1][x])
    }
    if y < map.len() - 1 {
        neighbors.push(map[y + 1][x])
    }
    neighbors
}

fn risk(input: &str) -> u32 {
    let heatmap = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    heatmap
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, p)| risk_level(*p, &neighbors(x, y, row, &heatmap)))
                .sum::<u32>()
        })
        .sum()
}

struct Basin {
    x: usize,
    y: usize,
    depth: u32,
}

fn floods(from: u32, to: u32) -> bool {
    from != 9 && from > to
}

fn largest_basins(input: &str) -> u32 {
    let heatmap = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut basins = Vec::new();
    for (y, row) in heatmap.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if risk_level(*p, &neighbors(x, y, row, &heatmap)) > 0 {
                basins.push(Basin { x, y, depth: *p });
            }
        }
    }
    let mut basin_sizes = basins
        .iter()
        .map(|basin| {
            let mut flood = Vec::new();
            let mut flooded = HashSet::new();
            flood.push((basin.x, basin.y, basin.depth));
            loop {
                let p = flood.pop();
                match p {
                    Some((x, y, height)) if !flooded.contains(&(x, y)) => {
                        flooded.insert((x, y));
                        let row = &heatmap[y];
                        if x > 0 && floods(row[x - 1], height) {
                            flood.push((x - 1, y, row[x - 1]));
                        }
                        if x < row.len() - 1 && floods(row[x + 1], height) {
                            flood.push((x + 1, y, row[x + 1]));
                        }
                        if y > 0 && floods(heatmap[y - 1][x], height) {
                            flood.push((x, y - 1, heatmap[y - 1][x]));
                        }
                        if y < heatmap.len() - 1 && floods(heatmap[y + 1][x], height) {
                            flood.push((x, y + 1, heatmap[y + 1][x]));
                        }
                    }
                    Some(_) => continue,
                    None => break,
                }
            }
            flooded.len() as u32
        })
        .collect::<Vec<u32>>();

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes.drain(..3).product()
}

#[test]
fn test_risk_level() {
    assert_eq!(risk_level(1, &[2, 9, 9]), 2);
    assert_eq!(risk_level(0, &[1, 1]), 1);
    assert_eq!(risk_level(5, &[8, 8, 6, 6]), 6);
    assert_eq!(risk_level(8, &[9, 9, 7, 5]), 0);
}

#[test]
fn test_largest_basins() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(largest_basins(input), 1134);
}
