use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut numbers: Vec<u32> = vec![];
    let f = File::open("src/day1/input_day1.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        let e: u32 = line.parse().unwrap();
        numbers.push(e);
    }

    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));

    Ok(())
}

fn part1(numbers: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;

    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            let sum = number1 + number2;
            if sum == 2020 {
                result = number1 * number2;
            }
        }
    }

    result
}

fn part2(numbers: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;

    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            for number3 in numbers.iter() {
                let sum = number1 + number2 + number3;
                if sum == 2020 {
                    result = number1 * number2 * number3;
                }
            }
        }
    }

    result
}