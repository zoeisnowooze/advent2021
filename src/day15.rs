use std::collections::HashSet;

#[derive(Clone)]
struct Vertex {
    position: (u32, u32),
    risk: u32,
}

fn remove_min(v: &mut Vec<Vertex>) -> Option<Vertex> {
    let index = v
        .iter()
        .enumerate()
        .min_by(|(_, x), (_, y)| x.risk.cmp(&y.risk))
        .map(|(index, _)| index);
    index.map(|index| v.swap_remove(index))
}

struct RiskMap {
    levels: Vec<Vec<u32>>,
    target: (u32, u32),
    explored: HashSet<(u32, u32)>,
}

impl RiskMap {
    fn new(levels: Vec<Vec<u32>>) -> RiskMap {
        let target = (levels[0].len() as u32 - 1, levels.len() as u32 - 1);
        RiskMap {
            levels,
            target,
            explored: HashSet::new(),
        }
    }

    fn explore(&mut self, origin: &Vertex, x: i32, y: i32) -> Option<Vertex> {
        if x < 0 || y < 0 || x as u32 > self.target.0 || y as u32 > self.target.1 {
            return None;
        }

        let position = (x as u32, y as u32);
        if self.explored.contains(&position) {
            return None;
        }

        let risk = origin.risk + self.levels[y as usize][x as usize];
        Some(Vertex { position, risk })
    }

    fn shortest_path(&mut self) -> Option<u32> {
        let mut paths: Vec<Vertex> = Vec::new();
        paths.push(Vertex {
            position: (0, 0),
            risk: 0,
        });
        loop {
            match remove_min(&mut paths) {
                Some(path) => {
                    if path.position == self.target {
                        return Some(path.risk);
                    }
                    self.explored.insert(path.position);

                    let (x, y) = (path.position.0 as i32, path.position.1 as i32);
                    if let Some(neighbor) = self.explore(&path, x - 1, y) {
                        paths.push(neighbor);
                    }
                    if let Some(neighbor) = self.explore(&path, x + 1, y) {
                        paths.push(neighbor);
                    }
                    if let Some(neighbor) = self.explore(&path, x, y - 1) {
                        paths.push(neighbor);
                    }
                    if let Some(neighbor) = self.explore(&path, x, y + 1) {
                        paths.push(neighbor);
                    }
                }
                None => return None,
            }
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day15.txt");
    let risk_level = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let mut risk_map = RiskMap::new(risk_level);
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
