use std::num::ParseIntError;
use std::str::FromStr;

struct Square {
    number: u8,
    marked: bool,
}

struct Board {
    grid: Vec<Vec<Square>>,
}

impl Board {
    fn mark(&mut self, number: u8) {
        for row in self.grid.iter_mut() {
            if let Some(square) = row.iter_mut().find(|square| square.number == number) {
                square.marked = true;
            }
        }
    }

    fn is_winning(&self) -> bool {
        for row in self.grid.iter() {
            if row.iter().all(|s| s.marked) {
                return true;
            }
        }
        for col in 0..5 {
            if self.grid.iter().all(|row| row[col].marked) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        self.grid.iter().fold(0u32, |acc, row| {
            acc + row
                .iter()
                .map(|s| if s.marked { 0 } else { s.number as u32 })
                .sum::<u32>()
        })
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Board, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| Square {
                        number: s.parse().unwrap(),
                        marked: false,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Square>>>();
        Ok(Board { grid })
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day4.txt");
    solve_part1(INPUT);
    solve_part2(INPUT);
}

fn solve_part1(input: &str) {
    let mut blocks = input.split("\n\n");
    let drawn_numbers = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap());
    let mut boards: Vec<Board> = blocks.map(|b| b.parse().unwrap()).collect();

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.is_winning() {
                println!(
                    "board wins after number {}, score {}",
                    number,
                    board.score()
                );
                println!("solution {}", board.score() * number as u32);
                return;
            }
        }
    }
}

fn solve_part2(input: &str) {
    let mut blocks = input.split("\n\n");
    let drawn_numbers = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap());
    let mut boards: Vec<Board> = blocks.map(|b| b.parse().unwrap()).collect();
    let mut winning_boards = vec![false; boards.len()];

    for number in drawn_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(number);
            if board.is_winning() {
                winning_boards[i] = true;
            }
            if winning_boards.iter().all(|b| *b) {
                println!(
                    "board wins after number {}, score {}",
                    number,
                    board.score()
                );
                println!("solution {}", board.score() * number as u32);
                return;
            }
        }
    }
}
