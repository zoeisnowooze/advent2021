use std::io::{self, BufRead};

struct Diagnostics<const N: usize> {
    report: Vec<String>,
}

impl<const N: usize> Diagnostics<N> {
    fn gamma_rate(&self) -> u32 {
        u32::from_str_radix(&self.most_common_bits(), 2).unwrap()
    }

    fn epsilon_rate(&self) -> u32 {
        0xfff ^ self.gamma_rate()
    }

    fn oxygen_generator_rating(&self) -> u32 {
        u32::from_str_radix(&self.most_common_value(), 2).unwrap()
    }

    fn co2_scrubber_rating(&self) -> u32 {
        u32::from_str_radix(&self.least_common_value(), 2).unwrap()
    }

    fn count_bit(&self, values: &[String], bit: usize) -> (usize, usize) {
        values
            .iter()
            .fold((0, 0), |acc, x| match x.chars().nth(bit).unwrap() {
                '0' => (acc.0 + 1, acc.1),
                '1' => (acc.0, acc.1 + 1),
                _ => acc,
            })
    }

    fn most_common_bits(&self) -> String {
        (0..N)
            .map(|i| {
                let (zeros, ones) = self.count_bit(&self.report, i);
                if zeros > ones {
                    '0'
                } else {
                    '1'
                }
            })
            .collect()
    }

    fn filter_values(
        &self,
        values: Vec<String>,
        bit: usize,
        predicate: impl Fn(usize, usize) -> bool,
    ) -> Vec<String> {
        if values.len() <= 1 {
            return values;
        }
        let (zeros, ones) = self.count_bit(&values, bit);
        values
            .iter()
            .filter(|x| match x.chars().nth(bit).unwrap() {
                '0' if !predicate(zeros, ones) => true,
                '1' if predicate(zeros, ones) => true,
                _ => false,
            })
            .cloned()
            .collect()
    }

    fn most_common_value(&self) -> String {
        (0..N)
            .fold(self.report.clone(), |r, i| {
                self.filter_values(r, i, |zeros, ones| ones >= zeros)
            })
            .first()
            .unwrap()
            .clone()
    }

    fn least_common_value(&self) -> String {
        (0..N)
            .fold(self.report.clone(), |r, i| {
                self.filter_values(r, i, |zeros, ones| ones < zeros)
            })
            .first()
            .unwrap()
            .clone()
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day3.txt");
    let report: Vec<String> = io::Cursor::new(INPUT).lines().map(|l| l.unwrap()).collect();
    let diagnostics = Diagnostics::<12> { report };

    println!(
        "gamma rate {} epsilon rate {}",
        diagnostics.gamma_rate(),
        diagnostics.epsilon_rate()
    );
    println!(
        "solution {}",
        diagnostics.gamma_rate() * diagnostics.epsilon_rate()
    );

    println!(
        "oxygen generator rating {} CO2 scrubber rating {}",
        diagnostics.oxygen_generator_rating(),
        diagnostics.co2_scrubber_rating()
    );
    println!(
        "solution {}",
        diagnostics.oxygen_generator_rating() * diagnostics.co2_scrubber_rating()
    );
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
    let diagnostics = Diagnostics::<5> { report };
    assert_eq!(diagnostics.most_common_value(), "10111");
}
