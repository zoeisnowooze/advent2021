use std::{ops::RangeInclusive, str::FromStr};

#[derive(Clone, Debug)]
struct Cuboid {
    positive: bool,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

fn contains_range(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    b.start() >= a.start() && b.end() <= a.end()
}

impl Cuboid {
    fn contains(&self, other: &Cuboid) -> bool {
        contains_range(&self.x, &other.x)
            && contains_range(&self.y, &other.y)
            && contains_range(&self.z, &other.z)
    }

    fn volume(&self) -> u64 {
        ((self.x.end() + 1 - self.x.start())
            * (self.y.end() + 1 - self.y.start())
            * (self.z.end() + 1 - self.z.start())) as u64
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positive = match &s[0..3] {
            "on " => true,
            "off" => false,
            _ => return Err(()),
        };
        let w = s.splitn(12, &['=', '.', ','][..]).collect::<Vec<&str>>();
        let x = w[1].parse().unwrap()..=w[3].parse().unwrap();
        let y = w[5].parse().unwrap()..=w[7].parse().unwrap();
        let z = w[9].parse().unwrap()..=w[11].parse().unwrap();
        Ok(Cuboid { positive, x, y, z })
    }
}

#[derive(Debug)]
struct Segments {
    inner: Vec<RangeInclusive<i64>>,
}

impl Segments {
    fn new(ranges: &[RangeInclusive<i64>]) -> Segments {
        let mut starts = ranges.iter().map(|r| *r.start()).collect::<Vec<i64>>();
        starts.sort_unstable();
        starts.dedup();
        let mut starts_iter = starts.into_iter().peekable();

        let mut ends = ranges.iter().map(|r| *r.end()).collect::<Vec<i64>>();
        ends.sort_unstable();
        ends.dedup();
        let mut ends_iter = ends.into_iter().peekable();

        let mut inner = Vec::new();
        let mut start = starts_iter.next().unwrap();
        while let Some(maybe_end) = ends_iter.peek() {
            if maybe_end < &start {
                ends_iter.next();
                continue;
            }
            if let Some(maybe_start) = starts_iter.peek() {
                if maybe_start <= &start {
                    starts_iter.next();
                    continue;
                }

                if maybe_start <= maybe_end {
                    let end = starts_iter.next().unwrap();
                    inner.push(start..=end - 1);
                    start = end;
                } else {
                    let end = ends_iter.next().unwrap();
                    inner.push(start..=end);
                    start = end + 1;
                }
            } else {
                let end = ends_iter.next().unwrap();
                inner.push(start..=end);
                start = end + 1;
            }
        }

        Segments { inner }
    }
}

fn reboot(steps: &[Cuboid]) -> u64 {
    let x_segments = Segments::new(
        &steps
            .iter()
            .map(|s| s.x.clone())
            .collect::<Vec<RangeInclusive<i64>>>(),
    );
    let y_segments = Segments::new(
        &steps
            .iter()
            .map(|s| s.y.clone())
            .collect::<Vec<RangeInclusive<i64>>>(),
    );
    let z_segments = Segments::new(
        &steps
            .iter()
            .map(|s| s.z.clone())
            .collect::<Vec<RangeInclusive<i64>>>(),
    );

    let mut count = 0;
    for x in x_segments.inner {
        let mut cuboid = Cuboid {
            positive: true,
            x: x.clone(),
            y: 0..=0,
            z: 0..=0,
        };

        for y in &y_segments.inner {
            cuboid.y.clone_from(y);

            for z in &z_segments.inner {
                cuboid.z.clone_from(z);

                let mut on = false;
                for step in steps.iter() {
                    if step.contains(&cuboid) {
                        on = step.positive;
                    }
                }
                if on {
                    count += cuboid.volume()
                }
            }
        }
    }

    count
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day22.txt");
    let steps = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Cuboid>>();
    println!("solution {}", reboot(&steps[..20]));
    println!("solution {}", reboot(&steps));
}

#[test]
fn test_example() {
    const INPUT: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let steps = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Cuboid>>();
    assert_eq!(reboot(&steps), 39);
}

#[test]
fn test_reboot() {
    const INPUT: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
";
    let steps = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Cuboid>>();
    assert_eq!(reboot(&steps), 590784);
}
