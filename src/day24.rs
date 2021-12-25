use std::str::FromStr;

#[derive(Copy, Clone)]
enum Operand {
    W,
    X,
    Y,
    Z,
    Literal(i64),
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Operand::W),
            "x" => Ok(Operand::X),
            "y" => Ok(Operand::Y),
            "z" => Ok(Operand::Z),
            _ => Ok(Operand::Literal(s.parse().unwrap())),
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        match iter.next().unwrap() {
            "inp" => Ok(Instruction::Inp(iter.next().unwrap().parse().unwrap())),
            "add" => Ok(Instruction::Add(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )),
            "mul" => Ok(Instruction::Mul(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )),
            "div" => Ok(Instruction::Div(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )),
            "mod" => Ok(Instruction::Mod(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )),
            "eql" => Ok(Instruction::Eql(
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}

struct ArithmeticLogicUnit {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    inputs: Vec<i64>,
}

impl ArithmeticLogicUnit {
    fn new(inputs: &[i64]) -> ArithmeticLogicUnit {
        ArithmeticLogicUnit {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            inputs: inputs.to_vec(),
        }
    }

    fn load(&self, register: Operand) -> i64 {
        match register {
            Operand::W => self.w,
            Operand::X => self.x,
            Operand::Y => self.y,
            Operand::Z => self.z,
            Operand::Literal(a) => a,
        }
    }

    fn store(&mut self, register: Operand, value: i64) {
        match register {
            Operand::W => {
                self.w = value;
            }
            Operand::X => {
                self.x = value;
            }
            Operand::Y => {
                self.y = value;
            }
            Operand::Z => {
                self.z = value;
            }
            Operand::Literal(_) => panic!("can't store in literal"),
        }
    }

    fn eval(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inp(a) => {
                let value = self.inputs.pop().unwrap();
                self.store(a, value);
            }
            Instruction::Add(a, b) => {
                let a_val = self.load(a);
                let b = self.load(b);
                self.store(a, a_val + b);
            }
            Instruction::Mul(a, b) => {
                let a_val = self.load(a);
                let b = self.load(b);
                self.store(a, a_val * b);
            }
            Instruction::Div(a, b) => {
                let a_val = self.load(a);
                let b = self.load(b);
                self.store(a, a_val / b);
            }
            Instruction::Mod(a, b) => {
                let a_val = self.load(a);
                let b = self.load(b);
                self.store(a, a_val % b);
            }
            Instruction::Eql(a, b) => {
                let a_val = self.load(a);
                let b = self.load(b);
                self.store(a, if a_val == b { 1 } else { 0 });
            }
        }
    }
}

struct ModelNumber {
    digits: [i64; 14],
}

impl ModelNumber {
    fn new() -> ModelNumber {
        ModelNumber { digits: [9; 14] }
    }
}

impl Iterator for ModelNumber {
    type Item = [i64; 14];

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        loop {
            self.digits[i] -= 1;
            if self.digits[i] == 0 {
                self.digits[i] = 9;
                i += 1;
            } else {
                break;
            }
        }
        Some(self.digits)
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day24.txt");
    let instructions = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Instruction>>();
    let model_number = ModelNumber::new();
    for digits in model_number {
        let mut alu = ArithmeticLogicUnit::new(&digits);
        for instruction in instructions.iter() {
            alu.eval(*instruction);
        }
        if alu.z == 0 {
            println!(
                "solution {}",
                digits
                    .iter()
                    .rev()
                    .map(|d| char::from_digit(*d as u32, 10).unwrap())
                    .collect::<String>()
            );
            break;
        }
    }
}

#[test]
fn test_negate() {
    let mut alu = ArithmeticLogicUnit::new(&[42]);
    alu.eval(Instruction::Inp(Operand::X));
    alu.eval(Instruction::Mul(Operand::X, Operand::Literal(-1)));
    assert_eq!(alu.x, -42);
}

#[test]
fn test_equals_triple() {
    let mut alu = ArithmeticLogicUnit::new(&[12, 4]);
    alu.eval(Instruction::Inp(Operand::Z));
    alu.eval(Instruction::Inp(Operand::X));
    alu.eval(Instruction::Mul(Operand::Z, Operand::Literal(3)));
    alu.eval(Instruction::Eql(Operand::Z, Operand::X));
    assert_eq!(alu.z, 1);
}
