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
    type Err = ();

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        let words = s.split_once(' ').unwrap();
        let action = match words.0 {
            "forward" => Action::Forward,
            "down" => Action::Down,
            "up" => Action::Up,
            _ => return Err(()),
        };
        let units = words.1.parse().unwrap();
        Ok(Command { action, units })
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day2.txt");
    let commands = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Command>>();

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
