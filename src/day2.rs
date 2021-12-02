use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;

enum Action {
    Forward,
    Down,
    Up,
}

struct Command {
    action: Action,
    units: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let words = s.split_once(' ').unwrap();
        let action = match words.0 {
            "forward" => Ok(Action::Forward),
            "down" => Ok(Action::Down),
            "up" => Ok(Action::Up),
            _ => Err(()),
        };
        let units = words.1.parse()?;
        Ok(Command {
            action: action.unwrap(),
            units: units,
        })
    }
}

fn main() {
    const INPUT: &'static str = include_str!("../inputs/day2.txt");
    let commands: Vec<Command> = io::Cursor::new(INPUT)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command.action {
            Action::Forward => {
                position += command.units;
                depth += aim * command.units;
            }
            Action::Down => {
                aim += command.units;
            }
            Action::Up => {
                aim -= command.units;
            }
        }
    }

    println!(
        "horizontal position {}, depth {}, aim {}",
        position, depth, aim
    );
    println!("solution {}", position * depth);
}
