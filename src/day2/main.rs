use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("src/day2/input_day2.txt")?;
    let f = BufReader::new(f);

    let mut valid_part1 = 0;
    let mut valid_part2 = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(": ").collect();
        let requirements = parts[0];
        let password = parts[1];

        let parts: Vec<&str> = requirements.split(" ").collect();
        let count = parts[0];
        let chars = parts[1].chars();
        let chars: Vec<char> = chars.collect();
        let letter = chars[0];

        let parts: Vec<&str> = count.split("-").collect();
        let number1 = parts[0];
        let number2 = parts[1];

        let number1: i32 = number1.parse().unwrap();
        let number2: i32 = number2.parse().unwrap();

        if validate_part1(password, letter, number1, number2) {
            valid_part1 += 1;
        }

        if validate_part2(password, letter, number1, number2) {
            valid_part2 += 1;
        }
    }

    println!("Part 1: Number of valid passwords: {}", valid_part1);
    println!("Part 2: Number of valid passwords: {}", valid_part2);

    Ok(())
}

fn validate_part1(password: &str, letter: char, min: i32, max: i32) -> bool {
    let mut is_valid = false;

    let mut count = 0;
    for c in password.chars() {
        if c == letter {
            count += 1;
        }
    }

    if min <= count && count <= max {
        is_valid = true
    }

    is_valid
}

fn validate_part2(password: &str, letter: char, position1: i32, position2: i32) -> bool {
    let position1 = position1 as usize;
    let position2 = position2 as usize;

    let bytes = password.as_bytes();
    let first: char = bytes[position1 - 1] as char;
    let second: char = bytes[position2 - 1] as char;

    (first == letter) ^ (second == letter)
}
