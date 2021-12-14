use std::collections::HashMap;

fn expand(depth: usize, left: char, right: char, rules: &[(char, char, char)], elements: &mut HashMap<char, usize>) {
    if depth == 0 {
        return;
    }
    match rules.iter().find(|rule| rule.0 == left && rule.1 == right) {
        Some(rule) => {
            let counter = elements.entry(rule.2).or_insert(0);
            *counter += 1;
            expand(depth - 1, left, rule.2, rules, elements);
            expand(depth - 1, rule.2, right, rules, elements);
        }
        None => {}
    }
}

fn polymerize(depth: usize, template: &str, rules: &[(char, char, char)]) -> (usize, usize) {
    let mut elements: HashMap<char, usize> = HashMap::new();

    let chars = template.chars().collect::<Vec<char>>();
    for pair in chars.windows(2) {
        let (left, right) = (pair[0], pair[1]);
        let counter = elements.entry(left).or_insert(0);
        *counter += 1;
        expand(depth, left, right, rules, &mut elements);
    }

    // Add the last (right-side) element.
    let counter = elements.entry(*chars.last().unwrap()).or_insert(0);
    *counter += 1;

    let least_common = elements.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    let most_common = elements.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    (*least_common, *most_common)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day14.txt");
    let (template, rules) = INPUT.split_once("\n\n").unwrap();
    let (least_common, most_common) = polymerize(
        10,
        template,
        &rules
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
            .collect::<Vec<(char, char, char)>>(),
    );
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
}
