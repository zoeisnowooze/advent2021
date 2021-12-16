use std::collections::HashMap;

fn polymerize(depth: usize, template: &str, rules: &[(char, char, char)]) -> (usize, usize) {
    let chars = &template.chars().collect::<Vec<char>>();

    let mut elements: HashMap<char, usize> = HashMap::new();
    for element in chars {
        *(elements.entry(*element).or_insert(0)) += 1;
    }

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for pair in chars.windows(2) {
        *(pairs.entry((pair[0], pair[1])).or_insert(0)) += 1;
    }

    for _ in 0..depth {
        let mut p = pairs.clone();
        for (pair, n) in &pairs {
            if let Some(rule) = rules
                .iter()
                .find(|rule| rule.0 == pair.0 && rule.1 == pair.1)
            {
                *(p.entry((pair.0, rule.2)).or_insert(0)) += n;
                *(p.entry((rule.2, pair.1)).or_insert(0)) += n;
                *(p.entry((pair.0, pair.1)).or_insert(0)) -= n;
                *(elements.entry(rule.2).or_insert(0)) += n;
            }
        }
        pairs = p;
    }

    let least_common = elements.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    let most_common = elements.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    (*least_common, *most_common)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day14.txt");
    let (template, rules) = INPUT.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|rule| rule.split_once(" -> ").unwrap())
        .map(|(a, b)| {
            let mut pair = a.chars();
            (
                pair.next().unwrap(),
                pair.next().unwrap(),
                b.chars().next().unwrap(),
            )
        })
        .collect::<Vec<(char, char, char)>>();

    let (least_common, most_common) = polymerize(10, template, &rules);
    println!("solution {}", most_common - least_common);

    let (least_common, most_common) = polymerize(40, template, &rules);
    println!("solution {}", most_common - least_common);
}

#[test]
fn test_polymerize() {
    let rules = [
        ('C', 'H', 'B'),
        ('H', 'H', 'N'),
        ('C', 'B', 'H'),
        ('N', 'H', 'C'),
        ('H', 'B', 'C'),
        ('H', 'C', 'B'),
        ('H', 'N', 'C'),
        ('N', 'N', 'C'),
        ('B', 'H', 'H'),
        ('N', 'C', 'B'),
        ('N', 'C', 'B'),
        ('N', 'B', 'B'),
        ('B', 'N', 'B'),
        ('B', 'B', 'N'),
        ('B', 'C', 'B'),
        ('C', 'C', 'N'),
        ('C', 'N', 'C'),
    ];
    assert_eq!(polymerize(10, "NNCB", &rules), (161, 1749));
    assert_eq!(polymerize(40, "NNCB", &rules), (3849876073, 2192039569602));
}
