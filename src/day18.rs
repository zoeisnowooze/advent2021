use std::fmt;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone)]
enum Value {
    Number(u32, usize),
    Pair(Box<Value>, Box<Value>),
}

impl Value {
    fn new_pair(left: Value, right: Value) -> Self {
        Value::Pair(Box::new(left), Box::new(right))
    }
}

impl Value {
    fn is_number_pair(&self) -> bool {
        match self {
            Value::Pair(left, right) => left.is_number() && right.is_number(),
            Value::Number(_, _) => false,
        }
    }

    fn is_number(&self) -> bool {
        matches!(*self, Value::Number(_, _))
    }

    fn number(&self) -> Option<u32> {
        match self {
            Self::Number(n, _) => Some(*n),
            _ => None
        }
    }

    fn index(&self) -> Option<usize> {
        match self {
            Self::Number(_, i) => Some(*i),
            _ => None
        }
    }

    fn add_number(&mut self, n: u32) {
        if let Self::Number(m, i) = self {
            *self = Value::Number(*m + n, *i);
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut v = Value::new_pair(self, other);
        reduce(&mut v);
        v
    }
}

fn parse_pair(iter: &mut std::iter::Peekable<std::str::Chars>) -> Value {
    if iter.next().unwrap() != '[' {
        panic!();
    }
    let left = match iter.peek().unwrap() {
        '[' => parse_pair(iter),
        '0'..='9' => Value::Number(iter.next().unwrap().to_digit(10).unwrap(), 0),
        _ => panic!(),
    };
    if iter.next().unwrap() != ',' {
        panic!();
    }
    let right = match iter.peek().unwrap() {
        '[' => parse_pair(iter),
        '0'..='9' => Value::Number(iter.next().unwrap().to_digit(10).unwrap(), 0),
        _ => panic!(),
    };
    if iter.next().unwrap() != ']' {
        panic!();
    }
    Value::new_pair(left, right)
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = parse_pair(&mut s.chars().peekable());
        renumber(&mut v);
        Ok(v)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n, _) => write!(f, "{}", n),
            Value::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

fn find_exploded(tree: &Value, depth: usize) -> Option<&Value> {
    if let Value::Pair(left, right) = tree {
        if depth == 4 && tree.is_number_pair() {
            Some(tree)
        } else {
            find_exploded(left, depth + 1)
                .or_else(|| find_exploded(right, depth + 1))
        }
    } else {
        None
    }
}

fn find_number(tree: &mut Value, index: usize, n: u32) -> Option<&Value> {
    match tree {
        Value::Pair(left, right) => find_number(left, index, n).or_else(|| find_number(right, index, n)),
        Value::Number(_, i) if *i == index => {
            tree.add_number(n);
            Some(tree)
        },
        _ => None,
    }
}

fn zero_pair(tree: &mut Value, index: usize) {
    if let Value::Pair(left, right) = tree {
        if let Value::Pair(ref n, _) = **left {
            if let Value::Number(_, i) = **n {
                if i == index {
                    *left = Box::new(Value::Number(0, 0));
                    return;
                }
            }
        }
        if let Value::Pair(ref n, _) = **right {
            if let Value::Number(_, i) = **n {
                if i == index {
                    *right = Box::new(Value::Number(0, 0));
                    return;
                }
            }
        }
        zero_pair(left, index);
        zero_pair(right, index);
    }
}

fn renumber_pair(value: &mut Value, counter: &mut usize) {
    if let Value::Pair(left, right) = value {
        renumber_pair(left.as_mut(), counter);
        renumber_pair(right.as_mut(), counter);
    }
    if let Value::Number(n, _) = value {
        *value = Value::Number(*n, *counter);
        *counter += 1;
    }
}

fn renumber(tree: &mut Value) {
    let mut counter = 0;
    renumber_pair(tree, &mut counter);
}

fn explode(tree: &mut Value) {
    let exploded = &mut find_exploded(tree, 0);
    if let Some(value) = exploded {
        if let Value::Pair(left, right) = value {
            let index = left.index().unwrap();
            let left_n = left.number().unwrap();
            let right_n = right.number().unwrap();

            zero_pair(tree, index);

            if index > 0 {
                find_number(tree, index - 1, left_n);
            }
            find_number(tree, index + 2, right_n);
            renumber(tree);
        } else {
            panic!();
        }
    }
}

fn reduce(value: &mut Value) -> &Value {
    explode(value);
    value
}

fn magnitude(value: Value) -> u64 {
    match value {
        Value::Number(n, _) => n as u64,
        Value::Pair(left, right) => 3 * magnitude(*left) + 2 * magnitude(*right),
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day18.txt");
    let reduced = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .reduce(|u: Value, v| u + v)
        .unwrap();
    println!("solution {}", magnitude(reduced));
}

#[test]
fn test_explode_left() {
    let mut v: Value = "[[[[[9,8],1],2],3],4]".parse().unwrap();
    assert_eq!(format!("{}", reduce(&mut v)), "[[[[0,9],2],3],4]");
}

#[test]
fn test_explode_right() {
    let mut v: Value = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
    assert_eq!(format!("{}", reduce(&mut v)), "[7,[6,[5,[7,0]]]]");
}

#[test]
fn test_explode_left_again() {
    let mut v: Value = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
    assert_eq!(format!("{}", reduce(&mut v)), "[[6,[5,[7,0]]],3]");
}

#[test]
fn test_explode() {
    let mut v: Value = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
    assert_eq!(
        format!("{}", reduce(&mut v)),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    );
}

#[test]
fn test_magnitude() {
    let v: Value = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        .parse()
        .unwrap();
    assert_eq!(magnitude(v), 3488);
}
