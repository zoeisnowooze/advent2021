struct Tokenizer {
    stack: Option<Vec<char>>,
    corrupted: Option<char>,
}

enum Chunk {
    Opening,
    Closing,
    Corrupted,
}

impl Tokenizer {
    fn tokenize(s: &str) -> Tokenizer {
        let mut stack = Vec::new();
        let mut corrupted = None;
        for c in s.chars() {
            let top_of_stack = stack.last();
            let token = match c {
                '(' | '<' | '[' | '{' => Chunk::Opening,
                ')' if top_of_stack == Some(&'(') => Chunk::Closing,
                '>' if top_of_stack == Some(&'<') => Chunk::Closing,
                ']' if top_of_stack == Some(&'[') => Chunk::Closing,
                '}' if top_of_stack == Some(&'{') => Chunk::Closing,
                ')' | '>' | ']' | '}' => Chunk::Corrupted,
                _ => panic!("unexpected chunk {}", c),
            };

            match token {
                Chunk::Opening => {
                    stack.push(c);
                }
                Chunk::Closing => {
                    stack.pop();
                }
                Chunk::Corrupted => {
                    corrupted = Some(c);
                    break;
                }
            }
        }

        match corrupted {
            Some(_) => Tokenizer {
                stack: None,
                corrupted,
            },
            None => Tokenizer {
                stack: Some(stack),
                corrupted,
            },
        }
    }

    fn score(self) -> Option<u64> {
        self.stack.map(|stack| {
            stack.iter().rev().fold(0_u64, |acc, c| {
                acc * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("{}", c),
                    }
            })
        })
    }
}

fn first_illegal_character(line: &str) -> Option<char> {
    let tokenizer = Tokenizer::tokenize(line);
    tokenizer.corrupted
}

fn completion_score(line: &str) -> Option<u64> {
    let tokenizer = Tokenizer::tokenize(line);
    tokenizer.score()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day10.txt");

    let score = INPUT
        .lines()
        .map(|line| {
            let c = first_illegal_character(line);
            match c {
                Some(')') => 3,
                Some(']') => 57,
                Some('}') => 1197,
                Some('>') => 25137,
                _ => 0,
            }
        })
        .sum::<u32>();
    println!("solution {}", score);

    let mut scores = INPUT
        .lines()
        .filter_map(completion_score)
        .collect::<Vec<u64>>();
    scores.sort_unstable();
    let score = scores[(scores.len() - 1) / 2];
    println!("solution {}", score);
}

#[test]
fn test_completion_score() {
    assert_eq!(completion_score("[({(<(())[]>[[{[]{<()<>>"), Some(288957));
    assert_eq!(completion_score("[(()[<>])]({[<{<<[]>>("), Some(5566));
    assert_eq!(completion_score("{([(<{}[<>[]}>{[]{[(<()>"), None);
    assert_eq!(completion_score("<{([{{}}[<[[[<>{}]]]>[]]"), Some(294));
}
