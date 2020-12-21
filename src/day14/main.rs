use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::collections::HashMap;
use std::env;

enum Part {
    Part1,
    Part2
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Write((u64, u64)),
}

type Memory = HashMap<u64, u64>;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let part = parse_part(&args);

    let mut f = File::open("src/day14/input_day14.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let instructions = parse(&s);

    let mut memory = HashMap::new();
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    match part {
        Part::Part1 => {
            for instruction in instructions {
                match instruction {
                    Instruction::Mask(m) => mask = m,
                    Instruction::Write((address, value)) => write(address, value, &mask, &mut memory),
                }
            }
        },
        Part::Part2 => {
            for instruction in instructions {
                match instruction {
                    Instruction::Mask(m) => mask = m,
                    Instruction::Write((address, value)) => write2(address, value, &mask, &mut memory),
                }
            }
        }
    }

    let sum = memory.values().fold(0, |acc, v| acc + v);

    println!("{}", sum);

    Ok(())
}

fn parse_part(args: &Vec<String>) -> Part {
    let mut part = Part::Part1;
    if args.len() > 1 {
        let arg = &args[1];
        match arg.as_str() {
            "1" => part = Part::Part1,
            "2" => part = Part::Part2,
            _ => ()
        }
    }

    part
}

fn parse(s: &String) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in s.lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        if parts[0] == "mask" {
            let mask = Instruction::Mask(String::from(parts[1]));
            instructions.push(mask);
        } else if parts[0].starts_with("mem") {
            let op: Vec<&str> = parts[0].split("[").collect();
            let op: Vec<&str> = op[1].split("]").collect();
            let n: u64 = op[0].parse().unwrap();

            let i: u64 = parts[1].parse().unwrap();
            instructions.push(Instruction::Write((n, i)));
        }
    }

    instructions
}

fn write(address: u64, value: u64, mask: &String, memory: &mut Memory) {
    let value = apply_mask(value, mask);

    memory.insert(address, value);
}

fn write2(address: u64, value: u64, mask: &String, memory: &mut Memory) {
    let address = address_to_binary_string(address);

    let addresses = apply_mask2(&address, mask);
    for addr in addresses {
        memory.insert(addr, value);
    }
}

fn apply_mask(value: u64, mask: &String) -> u64 {
    let mut v = value;
    for (i, c) in mask.chars().enumerate() {
        let bit = 35 - i;

        match c {
            'X' => continue,
            '0' => {
                let mut b = u64::MAX;
                b ^= 1 << bit;
                v &= b;
            },
            '1' => v |= 1 << bit,
            _ => panic!("Illegal character in mask"),
        }
    }

    v
}

fn apply_mask2(address: &String, mask: &String) -> Vec<u64> {
    let mut m = String::new();

    let addr_chars = address.chars();
    let mask_chars = mask.chars();

    for (ac, mc) in addr_chars.zip(mask_chars) {
        match mc {
            'X' => m.push('X'),
            '1' => m.push('1'),
            '0' => m.push(ac),
            k => panic!("Unknown letter: {}", k),
        }
    }

    let mut places = vec![];
    for (i, c) in mask.chars().enumerate() {
        if c == 'X' {
            places.push(i);
        }
    }

    let copy = m.replace("X", "0");
    let mask = usize::from_str_radix(&copy, 2).unwrap() as u64;

    generate_mask2(mask, &places)
}

fn address_to_binary_string(addr: u64) -> String {
    let mut a = String::new();

    for i in (0..36).rev() {
        let bit = addr >> i & 1;

        match bit {
            0 => a.push('0'),
            1 => a.push('1'),
            n => panic!("Illegal value: {}", n),
        }
    }

    a
}

fn generate_mask2(mask: u64, places: &Vec<usize>) -> Vec<u64> {
    let mut res1 = generate_mask_r(mask, 0, 0, places);
    let mut res2 = generate_mask_r(mask, 1, 0, places);

    res1.append(&mut res2);

    res1
}

fn generate_mask_r(mask: u64, replace: u64, number: usize, places: &Vec<usize>) -> Vec<u64> {
    let mut v = vec![];
    let len = places.len();

    let maskn = match replace {
        0 => mask,
        1 => mask | 1 << 35 - places[number],
        n => panic!("Unexpected number in generate_mask_r: {}", n),
    };

    if number == len - 1 {
        v.push(maskn);
    } else {
        let mut res1 = generate_mask_r(maskn, 0, number + 1, places);
        let mut res2 = generate_mask_r(maskn, 1, number + 1, places);

        v.append(&mut res1);
        v.append(&mut res2);
    }

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mask_generation() {
        let addr = String::from("000000000000000000000000000000101010");
        let mask = String::from("000000000000000000000000000000X1101X");

        let result = apply_mask2(&addr, &mask);

        assert_eq!(result.len(), 4);
        assert_eq!(result, vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_mask_generation_2() {
        let addr = String::from("000000000000000000000000000000011010");
        let mask = String::from("00000000000000000000000000000000X0XX");

        let result = apply_mask2(&addr, &mask);

        assert_eq!(result.len(), 8);
        assert_eq!(result, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
