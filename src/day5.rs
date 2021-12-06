use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

struct Vent {
    from: (u32, u32),
    to: (u32, u32),
}

impl IntoIterator for Vent {
    type Item = (u32, u32);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let (x1, y1) = self.from;
        let (x2, y2) = self.to;
        if self.from.0 == self.to.0 {
            let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            (start..=end).map(|i| (x1, i)).collect()
        } else if self.from.1 == self.to.1 {
            let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            (start..=end).map(|i| (i, y1)).collect()
        } else {
            let (x_start, y_start, x_end, y_end) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            if y_start < y_end {
                (x_start..=x_end).zip(y_start..=y_end).collect()
            } else {
                (x_start..=x_end)
                    .zip((y_end..=y_start).rev())
                    .collect::<Vec<Self::Item>>()
            }
        }
        .into_iter()
    }
}

impl FromStr for Vent {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Vent, Self::Err> {
        let (from, to) = s
            .split_once(" -> ")
            .map(|(from, to)| {
                (
                    from.split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
                    to.split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
                )
            })
            .unwrap();
        Ok(Vent {
            from: from.unwrap(),
            to: to.unwrap(),
        })
    }
}

impl fmt::Display for Vent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.from.0, self.from.1, self.to.0, self.to.1
        )
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day5.txt");
    let vents: Vec<Vent> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let mut map = HashMap::<(u32, u32), u32>::new();
    for vent in vents {
        for (x, y) in vent {
            let counter = map.entry((x, y)).or_insert(0);
            *counter += 1;
        }
    }
    let solution = map.values().filter(|&count| *count >= 2).count();
    println!("solution {}", solution);
}
