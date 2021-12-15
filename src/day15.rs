use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vertex {
    position: (usize, usize),
    risk: u32,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct RiskMap {
    levels: Vec<Vec<u32>>,
    size: usize,
    target: (usize, usize),
    risks: Vec<u32>,
}

impl RiskMap {
    fn new(levels: Vec<Vec<u32>>) -> RiskMap {
        let size = levels.len();
        RiskMap {
            levels,
            size,
            target: (size - 1, size - 1),
            risks: (0..size * size).map(|_| u32::MAX).collect(),
        }
    }

    fn explore(&mut self, position: (usize, usize), risk: u32) -> Option<Vertex> {
        let next_risk = risk + self.levels[position.1][position.0];
        if next_risk < self.risks[position.1 * self.size + position.0] {
            self.risks[position.1 * self.size + position.0] = next_risk;
            Some(Vertex {
                position,
                risk: next_risk,
            })
        } else {
            None
        }
    }

    fn shortest_path(&mut self) -> Option<u32> {
        let mut paths: BinaryHeap<Vertex> = BinaryHeap::new();

        paths.push(Vertex {
            position: (0, 0),
            risk: 0,
        });

        while let Some(Vertex { position, risk }) = paths.pop() {
            if position == self.target {
                return Some(risk);
            }

            if risk > self.risks[position.1 * self.size + position.0] {
                continue;
            }

            if position.0 > 0 {
                if let Some(next) = self.explore((position.0 - 1, position.1), risk) {
                    paths.push(next);
                }
            }
            if position.0 < self.size - 1 {
                if let Some(next) = self.explore((position.0 + 1, position.1), risk) {
                    paths.push(next);
                }
            }
            if position.1 > 0 {
                if let Some(next) = self.explore((position.0, position.1 - 1), risk) {
                    paths.push(next);
                }
            }
            if position.1 < self.size - 1 {
                if let Some(next) = self.explore((position.0, position.1 + 1), risk) {
                    paths.push(next);
                }
            }
        }

        None
    }
}

fn expand_map(risk_map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for i in 0..5 {
        for row in risk_map.iter() {
            let rows = [
                row.iter()
                    .map(|x| (x + i - 1) % 9 + 1)
                    .collect::<Vec<u32>>(),
                row.iter().map(|x| (x + i) % 9 + 1).collect(),
                row.iter().map(|x| (x + i + 1) % 9 + 1).collect(),
                row.iter().map(|x| (x + i + 2) % 9 + 1).collect(),
                row.iter().map(|x| (x + i + 3) % 9 + 1).collect(),
            ];
            result.push(rows.concat());
        }
    }
    result
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day15.txt");
    let risk_level = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let mut risk_map = RiskMap::new(risk_level);
    println!("solution {}", risk_map.shortest_path().unwrap());

    let risk_level = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let mut risk_map = RiskMap::new(expand_map(risk_level));
    println!("solution {}", risk_map.shortest_path().unwrap());
}

#[test]
fn test_shortest_path() {
    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let risk_level = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let mut risk_map = RiskMap::new(risk_level);
    assert_eq!(risk_map.shortest_path().unwrap(), 40);
}

#[test]
fn test_expand_map() {
    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let risk_level = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let mut risk_map = RiskMap::new(expand_map(risk_level));
    assert_eq!(risk_map.shortest_path().unwrap(), 315);
}
