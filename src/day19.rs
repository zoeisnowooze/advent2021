use std::collections::HashSet;
use std::str::FromStr;

struct Report(i32, i32, i32);

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(3, ',').map(|n| n.parse().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Ok(Self(x, y, z))
    }
}

fn make_diffs(reports: &Vec<Report>) -> HashSet<Report> {
    let mut set = HashSet::new();
    for (i, report) in reports.iter().enumerate() {
        
    }
    set
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day19.txt");
    let reports = INPUT.split("\n\n").map(|scanner| scanner.lines().map(|line| line.parse().unwrap()).collect()).collect::<Vec<Vec<Report>>>();
    let diffs: Vec<HashSet<Report>> = reports.iter().map(make_diffs).collect();
    println!("solution {}", diffs.len());
}
