use std::collections::HashSet;
use std::str::FromStr;

enum Axis {
    X,
    Y,
}

struct FoldInstruction {
    axis: Axis,
    position: usize,
}

impl FoldInstruction {
    fn wrap(&self, dot: (usize, usize)) -> (usize, usize) {
        match self.axis {
            Axis::X if dot.0 >= self.position => (2 * self.position - dot.0, dot.1),
            Axis::Y if dot.1 >= self.position => (dot.0, 2 * self.position - dot.1),
            _ => (dot.0, dot.1),
        }
    }
}

impl FromStr for FoldInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<FoldInstruction, Self::Err> {
        let (a, p) = s[11..].split_once('=').unwrap();
        let axis = match a {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => return Err(()),
        };
        let position = p.parse().unwrap();
        Ok(FoldInstruction { axis, position })
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day13.txt");
    let sections = INPUT.split_once("\n\n").unwrap();
    let mut dots = sections
        .0
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();
    let instructions = sections
        .1
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<FoldInstruction>>();

    let mut all_dots: HashSet<(usize, usize)> = HashSet::new();
    for dot in &dots {
        all_dots.insert(instructions[0].wrap(*dot));
    }
    println!("solution {}", all_dots.len());

    all_dots.drain();
    for instr in instructions {
        for dot in &dots {
            all_dots.insert(instr.wrap(*dot));
        }
        dots = all_dots.drain().collect();
    }
    for y in 0..6 {
        println!(
            "{}",
            (0..40)
                .map(|x| if dots.contains(&(x, y)) {
                    '🟪'
                } else {
                    '⬛'
                })
                .collect::<String>()
        );
    }
}

#[test]
fn test_wrap_instruction() {
    let instruction = FoldInstruction {
        axis: Axis::Y,
        position: 7,
    };
    assert_eq!(instruction.wrap((3, 0)), (3, 0));
    assert_eq!(instruction.wrap((1, 10)), (1, 4));
}
