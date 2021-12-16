fn main() {
    const INPUT: &str = include_str!("../inputs/day6.txt");
    let fishes: Vec<usize> = INPUT
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|timer| timer.parse::<usize>().unwrap())
        .collect();
    println!("solution {}", solve(fishes.clone(), 80));
    println!("solution {}", solve(fishes, 256));
}

const BIRTH_RATE: usize = 7;
const MATURITY: usize = 9;

fn solve(fishes: Vec<usize>, days: usize) -> u64 {
    let mut timers = [0_u64; BIRTH_RATE];
    let mut new_timers = [0_u64; MATURITY];

    for fish in fishes {
        timers[fish] += 1;
    }

    for day in 0..days {
        let spawns = timers[day % BIRTH_RATE];
        let adults = new_timers[day % MATURITY];
        new_timers[day % MATURITY] += spawns;
        timers[day % BIRTH_RATE] += adults;
    }
    timers.iter().sum::<u64>() + new_timers.iter().sum::<u64>()
}

#[test]
fn test_initial_state() {
    let fishes = vec![3, 4, 3, 1, 2];
    assert_eq!(solve(fishes, 0), 5);
}

#[test]
fn test_18days() {
    let fishes = vec![3, 4, 3, 1, 2];
    assert_eq!(solve(fishes, 18), 26);
}

#[test]
fn test_80days() {
    let fishes = vec![3, 4, 3, 1, 2];
    assert_eq!(solve(fishes, 80), 5934);
}

#[test]
fn test_256days() {
    let fishes = vec![3, 4, 3, 1, 2];
    assert_eq!(solve(fishes, 256), 26984457539);
}
