use std::collections::HashMap;

struct DeterministicDice {
    rolls: usize,
    last: usize,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice { rolls: 0, last: 0 }
    }
}

impl Iterator for DeterministicDice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 3;
        let x = self.last % 100 + 1;
        let y = (self.last + 1) % 100 + 1;
        let z = (self.last + 2) % 100 + 1;
        self.last = z;
        Some(x + y + z)
    }
}

fn turns_to_win(positions: (usize, usize)) -> (usize, usize) {
    let mut dice = DeterministicDice::new();
    let mut positions = positions;
    let mut scores = (0, 0);

    loop {
        positions.0 = (positions.0 + dice.next().unwrap() - 1) % 10 + 1;
        scores.0 += positions.0;
        if scores.0 >= 1000 {
            return (dice.rolls, scores.1);
        }

        positions.1 = (positions.1 + dice.next().unwrap() - 1) % 10 + 1;
        scores.1 += positions.1;
        if scores.1 >= 1000 {
            return (dice.rolls, scores.0);
        }
    }
}

fn simulate_universes(positions: (usize, usize)) -> usize {
    type GameState = ((usize, usize), (usize, usize));

    let mut games: HashMap<GameState, usize> = HashMap::new();
    let mut wins = (0, 0);

    games.insert(((positions.0 - 1, positions.1 - 1), (0, 0)), 1);

    let dice_rolls = [
        3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
    ];
    loop {
        if games.is_empty() {
            break;
        }
        let mut next_turn = HashMap::new();
        for ((positions, scores), count) in games {
            for player1_dice in dice_rolls {
                let player1_position = (positions.0 + player1_dice) % 10;
                let player1_score = scores.0 + player1_position + 1;
                if player1_score < 21 {
                    for player2_dice in dice_rolls {
                        let player2_position = (positions.1 + player2_dice) % 10;
                        let player2_score = scores.1 + player2_position + 1;
                        if player2_score < 21 {
                            let new_count = next_turn
                                .entry((
                                    (player1_position, player2_position),
                                    (player1_score, player2_score),
                                ))
                                .or_insert(0);
                            *new_count += count;
                        } else {
                            wins.1 += count;
                        }
                    }
                } else {
                    wins.0 += count;
                }
            }
        }
        games = next_turn;
    }

    if wins.0 > wins.1 {
        wins.0
    } else {
        wins.1
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day21.txt");
    let positions = INPUT
        .lines()
        .map(|line| line.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect::<Vec<usize>>();
    let (turns, losing_score) = turns_to_win((positions[0], positions[1]));
    println!("solution: {}", turns * losing_score);
    println!(
        "solution: {}",
        simulate_universes((positions[0], positions[1]))
    );
}

#[test]
fn test_turns_to_win() {
    let (turns, losing_score) = turns_to_win((4, 8));
    assert_eq!(turns, 993);
    assert_eq!(losing_score, 745);
}

#[test]
fn test_simulate_universes() {
    assert_eq!(simulate_universes((4, 8)), 444356092776315);
}
