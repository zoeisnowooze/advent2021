use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
enum Herd {
    East,
    South,
}

struct SeaCucumberMap {
    inner: Vec<Vec<Option<Herd>>>,
}

impl SeaCucumberMap {
    fn stops_moving(&mut self) -> usize {
        let mut steps = 0;
        while self.step() {
            steps += 1;
        }
        steps + 1
    }

    fn step(&mut self) -> bool {
        let shifted: Vec<Vec<Option<Herd>>> =
            (0..self.inner.len()).map(|i| self.shift_right(i)).collect();
        let new_map = (0..self.inner.len())
            .map(|i| self.shift_down(i, &shifted))
            .collect();
        let changed = self.inner != new_map;
        self.inner = new_map;
        changed
    }

    fn shift_right(&self, i: usize) -> Vec<Option<Herd>> {
        let row = &self.inner[i];
        let mut new_row = Vec::with_capacity(row.len());
        for (i, loc) in row.iter().enumerate() {
            let left = row[(i + row.len() - 1) % row.len()];
            let right = row[(i + row.len() + 1) % row.len()];
            new_row.push(match (left, loc, right) {
                (Some(Herd::East), None, _) => left,
                (_, Some(Herd::East), None) => None,
                _ => *loc,
            });
        }
        new_row
    }

    fn shift_down(&self, i: usize, shifted: &[Vec<Option<Herd>>]) -> Vec<Option<Herd>> {
        let height = self.inner.len();
        let row = &shifted[i];
        let above_row = &shifted[(i + height - 1) % height];
        let below_row = &shifted[(i + height + 1) % height];
        let mut new_row = Vec::with_capacity(row.len());
        for (above, (loc, below)) in above_row.iter().zip(row.iter().zip(below_row)) {
            new_row.push(match (*above, *loc, below) {
                (Some(Herd::South), None, _) => *above,
                (_, Some(Herd::South), None) => None,
                _ => *loc,
            });
        }
        new_row
    }
}

impl FromStr for SeaCucumberMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '>' => Some(Herd::East),
                        'v' => Some(Herd::South),
                        '.' => None,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Ok(SeaCucumberMap { inner })
    }
}

impl fmt::Debug for SeaCucumberMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.inner.iter() {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|c| match c {
                        Some(Herd::East) => '>',
                        Some(Herd::South) => 'v',
                        None => '.',
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day25.txt");
    let mut map: SeaCucumberMap = INPUT.parse().unwrap();
    println!("solution {}", map.stops_moving());
}

#[test]
fn test_stops_moving() {
    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let mut map: SeaCucumberMap = INPUT.parse().unwrap();
    assert_eq!(map.stops_moving(), 58);
}
