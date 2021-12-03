use std::io::{self, BufRead};

fn most_common_bits(report: &Vec<String>) -> String {
    (0..12)
        .map(|i| {
            let counts = report
                .iter()
                .fold((0, 0), |acc, x| match x.chars().nth(i).unwrap() {
                    '0' => (acc.0 + 1, acc.1),
                    '1' => (acc.0, acc.1 + 1),
                    _ => acc,
                });
            if counts.0 > counts.1 {
                '0'
            } else {
                '1'
            }
        })
        .collect()
}

fn most_common_value(report: Vec<String>, n: usize) -> Option<String> {
    (0..n)
        .fold(report, |r, i| {
            if r.len() > 1 {
                let counts = r
                    .iter()
                    .fold((0, 0), |acc, x| match x.chars().nth(i).unwrap() {
                        '0' => (acc.0 + 1, acc.1),
                        '1' => (acc.0, acc.1 + 1),
                        _ => acc,
                    });
                r.iter()
                    .filter(|x| match x.chars().nth(i).unwrap() {
                        '0' if counts.0 > counts.1 => true,
                        '1' if counts.0 <= counts.1 => true,
                        _ => false,
                    })
                    .cloned()
                    .collect()
            } else {
                r
            }
        })
        .first()
        .cloned()
}

fn least_common_value(report: Vec<String>) -> Option<String> {
    (0..12)
        .fold(report, |r, i| {
            if r.len() > 1 {
                let counts = r
                    .iter()
                    .fold((0, 0), |acc, x| match x.chars().nth(i).unwrap() {
                        '0' => (acc.0 + 1, acc.1),
                        '1' => (acc.0, acc.1 + 1),
                        _ => acc,
                    });
                r.iter()
                    .filter(|x| match x.chars().nth(i).unwrap() {
                        '0' if counts.0 <= counts.1 => true,
                        '1' if counts.0 > counts.1 => true,
                        _ => false,
                    })
                    .cloned()
                    .collect()
            } else {
                r
            }
        })
        .first()
        .cloned()
}

fn main() {
    const INPUT: &'static str = include_str!("../inputs/day3.txt");
    let report: Vec<String> = io::Cursor::new(INPUT).lines().map(|l| l.unwrap()).collect();
    let gamma_rate = u32::from_str_radix(&most_common_bits(&report), 2).unwrap();
    let epsilon_rate = 0xfff ^ gamma_rate;
    println!("gamma rate {} epsilon rate {}", gamma_rate, epsilon_rate);
    println!("solution {}", gamma_rate * epsilon_rate);

    let oxygen_generator_rating =
        u32::from_str_radix(&most_common_value(report.clone(), 12).unwrap(), 2).unwrap();
    println!("oxygen generator rating {}", oxygen_generator_rating);

    let co2_scrubber_rating = u32::from_str_radix(&least_common_value(report).unwrap(), 2).unwrap();
    println!("CO2 scrubber rating {}", co2_scrubber_rating);

    println!("solution {}", oxygen_generator_rating * co2_scrubber_rating);
}

#[test]
fn oxygen_generator_rating() {
    let report = vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
    ];
    assert_eq!(most_common_value(report, 5).unwrap(), "10111");
}
