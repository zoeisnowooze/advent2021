use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct CavePath {
    caves: (String, String),
}

impl CavePath {
    fn from(&self, cave: &str) -> Option<&str> {
        if cave == self.caves.0 {
            Some(&self.caves.1)
        } else if cave == self.caves.1 {
            Some(&self.caves.0)
        } else {
            None
        }
    }
}

impl FromStr for CavePath {
    type Err = ();

    fn from_str(s: &str) -> Result<CavePath, Self::Err> {
        let caves = s.split_once('-').unwrap();
        Ok(CavePath {
            caves: (caves.0.to_string(), caves.1.to_string()),
        })
    }
}

fn is_small_cave(cave: &str) -> bool {
    if cave == "start" {
        false
    } else {
        cave.chars().all(|c| matches!(c, 'a'..='z'))
    }
}

fn count_paths(input: &str, allow_twice: bool) -> usize {
    let paths = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<CavePath>>();
    count_paths_recursive("start", &paths, &HashMap::from([("start", 2)]), allow_twice)
}

fn count_paths_recursive(
    root: &str,
    paths: &[CavePath],
    visited: &HashMap<&str, usize>,
    allow_twice: bool,
) -> usize {
    if root == "end" {
        return 1;
    }

    let mut new_visited = visited.clone();
    if is_small_cave(root) {
        let counter = new_visited.entry(root).or_insert(0);
        *counter += 1;
    }

    let mut sum = 0;
    for path in paths {
        if let Some(cave) = path.from(root) {
            let visited_count = visited.get(cave);
            let is_twice = allow_twice && is_small_cave(cave) && visited_count == Some(&1);
            if visited_count.is_none() || is_twice {
                sum += count_paths_recursive(cave, paths, &new_visited, allow_twice && !is_twice);
            }
        }
    }
    sum
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day12.txt");
    println!("solution {}", count_paths(INPUT, false));
    println!("solution {}", count_paths(INPUT, true));
}

#[test]
fn test_count_single_paths() {
    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    assert_eq!(count_paths(INPUT, false), 10);
}

#[test]
fn test_count_double_paths() {
    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    assert_eq!(count_paths(INPUT, true), 36);
}

#[test]
fn test_count_larger_single_paths() {
    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    assert_eq!(count_paths(INPUT, false), 226);
}

#[test]
fn test_count_larger_double_paths() {
    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    assert_eq!(count_paths(INPUT, true), 3509);
}
