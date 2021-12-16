use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
enum PacketPayload {
    Literal(u64),
    Operator(u32, Vec<Packet>),
}

#[derive(Clone, Debug)]
struct Packet {
    version: u32,
    payload: PacketPayload,
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.payload == other.payload
    }
}

type ParsePacketResult = Result<Packet, ParseIntError>;

struct PacketParser<'a> {
    s: std::str::Chars<'a>,
}

impl PacketParser<'_> {
    fn new(s: &str) -> PacketParser {
        PacketParser { s: s.chars() }
    }

    fn take_string(&mut self, n: usize) -> String {
        (&mut self.s).take(n).collect()
    }

    fn parse_number(&mut self, bits: usize) -> Result<u32, ParseIntError> {
        u32::from_str_radix(&self.take_string(bits), 2)
    }

    fn parse_operator(&mut self, id: u32) -> PacketPayload {
        let mut packets;
        match self.parse_number(1).unwrap() {
            0 => {
                let len = self.parse_number(15).unwrap() as usize;
                let substring = self.take_string(len);
                let sub_parser = &mut PacketParser::new(&substring);
                packets = Vec::new();
                while let Ok(packet) = sub_parser.parse_packet() {
                    packets.push(packet);
                }
            }
            1 => {
                let n = self.parse_number(11).unwrap();
                packets = Vec::with_capacity(n as usize);
                for _ in 0..n {
                    packets.push(self.parse_packet().unwrap());
                }
            }
            _ => unreachable!(),
        }
        PacketPayload::Operator(id, packets)
    }

    fn parse_literal(&mut self) -> PacketPayload {
        let mut n = 0_u64;
        loop {
            let first_bit = self.s.next().unwrap();
            n = (n << 1) + self.s.next().unwrap().to_digit(2).unwrap() as u64;
            n = (n << 1) + self.s.next().unwrap().to_digit(2).unwrap() as u64;
            n = (n << 1) + self.s.next().unwrap().to_digit(2).unwrap() as u64;
            n = (n << 1) + self.s.next().unwrap().to_digit(2).unwrap() as u64;
            if first_bit == '0' {
                break;
            }
        }
        PacketPayload::Literal(n)
    }

    fn parse_packet(&mut self) -> ParsePacketResult {
        let version = self.parse_number(3)?;
        let payload = match self.parse_number(3)? {
            4 => self.parse_literal(),
            id => self.parse_operator(id),
        };
        Ok(Packet { version, payload })
    }
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> ParsePacketResult {
        let s = s
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .map(|n| format!("{:04b}", n))
            .collect::<String>();
        let mut parser = PacketParser::new(&s);
        parser.parse_packet()
    }
}

fn sum_version_numbers(p: &Packet) -> u32 {
    match &p.payload {
        PacketPayload::Literal(_) => p.version as u32,
        PacketPayload::Operator(_, subpackets) => {
            p.version + subpackets.iter().map(sum_version_numbers).sum::<u32>()
        }
    }
}

fn eval_packet(p: &Packet) -> u64 {
    match &p.payload {
        PacketPayload::Literal(n) => *n as u64,
        PacketPayload::Operator(id, subpackets) => match id {
            0 => subpackets.iter().map(eval_packet).sum::<u64>(),
            1 => subpackets.iter().map(eval_packet).product::<u64>(),
            2 => subpackets.iter().map(eval_packet).min().unwrap(),
            3 => subpackets.iter().map(eval_packet).max().unwrap(),
            5 => {
                assert_eq!(subpackets.len(), 2);
                if eval_packet(&subpackets[0]) > eval_packet(&subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                assert_eq!(subpackets.len(), 2);
                if eval_packet(&subpackets[0]) < eval_packet(&subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                assert_eq!(subpackets.len(), 2);
                if eval_packet(&subpackets[0]) == eval_packet(&subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!("invalid operator ID {}", id),
        },
    }
}

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("../inputs/day16.txt").trim_end();
    let packet: Packet = input.parse()?;
    println!("solution {}", sum_version_numbers(&packet));
    println!("solution {}", eval_packet(&packet));
    Ok(())
}

#[test]
fn test_literal() {
    let packet: Packet = "D2FE28".parse().unwrap();
    assert_eq!(
        packet,
        Packet {
            version: 6,
            payload: PacketPayload::Literal(2021)
        }
    );
}

#[test]
fn test_parse_type_0_operator() {
    let packet: Packet = "38006F45291200".parse().unwrap();
    let packets = vec![
        Packet {
            version: 6,
            payload: PacketPayload::Literal(10),
        },
        Packet {
            version: 2,
            payload: PacketPayload::Literal(20),
        },
    ];
    assert_eq!(
        packet,
        Packet {
            version: 1,
            payload: PacketPayload::Operator(6, packets)
        }
    );
}

#[test]
fn test_parse_type_1_operator() {
    let packet: Packet = "EE00D40C823060".parse().unwrap();
    assert_eq!(
        packet,
        Packet {
            version: 7,
            payload: PacketPayload::Operator(
                3,
                vec![
                    Packet {
                        version: 2,
                        payload: PacketPayload::Literal(1)
                    },
                    Packet {
                        version: 4,
                        payload: PacketPayload::Literal(2)
                    },
                    Packet {
                        version: 1,
                        payload: PacketPayload::Literal(3)
                    },
                ]
            )
        }
    );
}

#[test]
fn test_sum_version_numbers() {
    let transmissions = [
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];
    let mut iter = transmissions
        .iter()
        .map(|t| sum_version_numbers(&t.parse().unwrap()));
    assert_eq!(iter.next(), Some(16));
    assert_eq!(iter.next(), Some(12));
    assert_eq!(iter.next(), Some(23));
    assert_eq!(iter.next(), Some(31));
}

#[test]
fn test_sum_operator() {
    let packet: Packet = "C200B40A82".parse().unwrap();
    assert_eq!(eval_packet(&packet), 3);
}

#[test]
fn test_product_operator() {
    let packet: Packet = "04005AC33890".parse().unwrap();
    assert_eq!(eval_packet(&packet), 54);
}

#[test]
fn test_minimum_operator() {
    let packet: Packet = "880086C3E88112".parse().unwrap();
    assert_eq!(eval_packet(&packet), 7);
}

#[test]
fn test_maximum_operator() {
    let packet: Packet = "CE00C43D881120".parse().unwrap();
    assert_eq!(eval_packet(&packet), 9);
}

#[test]
fn test_less_than_operator() {
    let packet: Packet = "D8005AC2A8F0".parse().unwrap();
    assert_eq!(eval_packet(&packet), 1);
}

#[test]
fn test_greater_than_operator() {
    let packet: Packet = "F600BC2D8F".parse().unwrap();
    assert_eq!(eval_packet(&packet), 0);
}

#[test]
fn test_equal_operator() {
    let packet: Packet = "9C005AC2F8F0".parse().unwrap();
    assert_eq!(eval_packet(&packet), 0);
}

#[test]
fn test_nested_equal_operator() {
    let packet: Packet = "9C0141080250320F1802104A08".parse().unwrap();
    assert_eq!(eval_packet(&packet), 1);
}
