use anyhow::Result;
use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug,Clone, PartialEq, Eq)]
enum Packet {
    Int(u32),
    Packet(Vec<Packet>)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(s), Packet::Int(o)) => s.cmp(o),
            (Packet::Int(s), Packet::Packet(_)) => Packet::Packet(vec![Packet::Int(*s)]).cmp(other),
            (Packet::Packet(_), Packet::Int(o)) => self.cmp(&Packet::Packet(vec![Packet::Int(*o)])),
            (Packet::Packet(s), Packet::Packet(o)) => s.cmp(o),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Packet::Int(n) => write!(f, "{}", n),
            Packet::Packet(v) => {
                write!(f, "[")?;
                for p in v.iter() {
                    write!(f, "{},", p)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl FromStr for Packet {
    fn from_str(line: &str) ->  Result<Self, Self::Err> {

        fn parse_tokens(tokens: &mut Vec<&str>) -> Result<Vec<Packet>, ParseIntError> {
            let mut result = Vec::new();
            while !tokens.is_empty() {
                match tokens.pop() {
                    Some("]") => return Ok(result),
                    Some("[") => result.push(Packet::Packet(parse_tokens(tokens)?)),
                    Some("") => {},
                    Some(number) => result.push(Packet::Int(number.parse()?)),
                    None => unreachable!()
                }
            }
            Ok(result)
        }
        let tokenified_line = line.replace('[', "[,").replace(']', ",]");
        let mut tokens = tokenified_line.split(',').rev().collect::<Vec<_>>();
        Ok(Packet::Packet(parse_tokens(&mut tokens)?))
    }
    type Err = ParseIntError;
}

fn part01() -> Result<u32> {
    let mut idx = 0;
    Ok(std::fs::read_to_string("./data/day13.input")?
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let left_package = lines.next().unwrap().parse::<Packet>().unwrap();
            let right_package = lines.next().unwrap().parse::<Packet>().unwrap();
            idx += 1;
            match left_package < right_package {
                true => idx,
                false => 0
            }
        }).sum())
}

fn part02() -> Result<u32> {
    let mut packets = std::fs::read_to_string("./data/day13.input")?
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();
    let sep_2 = "[[2]]".parse::<Packet>().unwrap();
    let sep_6 = "[[6]]".parse::<Packet>().unwrap();
    packets.push(sep_2.clone());
    packets.push(sep_6.clone());
    packets.sort();
    // for p in packets.iter() {
    //     println!("{}", p);
    // }
    Ok(((packets.iter().position(|packet| packet == &sep_2).unwrap() + 1) *
        (packets.iter().position(|packet| packet == &sep_6).unwrap() + 1)) as u32)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
