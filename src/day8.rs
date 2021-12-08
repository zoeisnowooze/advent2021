fn contains(a: &str, b: &str) -> bool {
    b.chars().all(|c| a.contains(c))
}

fn find_bd(one: &str, four: &str) -> String {
    four.chars()
        .filter(|c| !one.contains(*c))
        .collect::<String>()
}

fn hash(s: &str) -> u32 {
    s.chars()
        .map(|c| match c {
            'a' => 2,
            'b' => 3,
            'c' => 5,
            'd' => 7,
            'e' => 11,
            'f' => 13,
            'g' => 17,
            _ => panic!(),
        })
        .product()
}

fn decode_digit(patterns: &[&str], digit_positions: &[u32], output: &str) -> u32 {
    *digit_positions
        .iter()
        .zip(patterns)
        .find(|(_i, p)| hash(p) == hash(output))
        .unwrap()
        .0
}

fn decode(patterns: Vec<&str>, outputs: Vec<&str>) -> u32 {
    let one = patterns.iter().find(|p| p.len() == 2).unwrap();
    let four = patterns.iter().find(|p| p.len() == 4).unwrap();
    let bd = find_bd(one, four);

    let digit_positions: Vec<u32> = patterns
        .iter()
        .map(|p| match p.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            5 if !contains(p, one) && !contains(p, &bd) => 2,
            5 if contains(p, one) => 3,
            5 if contains(p, &bd) => 5,
            6 if !contains(p, one) => 6,
            6 if contains(p, four) => 9,
            6 if contains(p, one) && !contains(p, four) => 0,
            _ => panic!(),
        })
        .collect();

    let mut iter = outputs.iter();
    decode_digit(&patterns, &digit_positions, iter.next().unwrap()) * 1000
        + decode_digit(&patterns, &digit_positions, iter.next().unwrap()) * 100
        + decode_digit(&patterns, &digit_positions, iter.next().unwrap()) * 10
        + decode_digit(&patterns, &digit_positions, iter.next().unwrap())
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day8.txt");
    let easy_digits: usize = INPUT
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .rev()
                .take(4)
                .filter(|d| d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7)
                .count()
        })
        .sum::<usize>();
    println!("solution {}", easy_digits);

    let digits = INPUT
        .lines()
        .map(|line| {
            let (unique_patterns, outputs) = line
                .split_once(" | ")
                .map(|(s, t)| {
                    (
                        s.split_ascii_whitespace().collect(),
                        t.split_ascii_whitespace().collect(),
                    )
                })
                .unwrap();
            decode(unique_patterns, outputs)
        })
        .sum::<u32>();
    println!("solution {}", digits);
}

#[test]
fn test_decode() {
    let unique_patterns = vec![
        "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb",
    ];
    let outputs = vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"];
    assert_eq!(decode(unique_patterns, outputs), 8394);
}

#[test]
fn test_hash() {
    assert_eq!(hash("cefbgd"), 3 * 5 * 7 * 11 * 13 * 17);
}
