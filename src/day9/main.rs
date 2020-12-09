use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("src/day9/input_day9.txt")?;
    let f = BufReader::new(f);

    let preamble_size = 25;
    let mut message: Vec<i64> = vec![];

    for line in f.lines() {
        let line = line.unwrap();

        message.push(line.parse().unwrap());
    }

    let mut first_invalid: i64 = 0;
    for i in preamble_size..message.len() {
        let n = message[i];

        let start = i - preamble_size;
        let end = i;
        if !validate(&message[start..end], n) {
            first_invalid = n;
            break;
        }
    }

    println!("First invalid entry: {}", first_invalid);

    let range = find_weakness(&mut message, first_invalid);
    let range = &mut message[range.0..range.1];
    range.sort();

    let smallest = range.first().unwrap();
    let largest = range.last().unwrap();
    println!(
        "Smallest: {} Largest: {}, Sum: {}",
        smallest,
        largest,
        smallest + largest
    );

    Ok(())
}

fn validate(numbers: &[i64], e: i64) -> bool {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let one = numbers[i];
            let two = numbers[j];

            let sum = one + two;

            if sum == e {
                return true;
            }
        }
    }

    false
}

fn find_weakness(numbers: &Vec<i64>, e: i64) -> (usize, usize) {
    let mut range = (0, 0);
    let mut sum = 0;

    while sum != e {
        sum = 0;

        for i in range.0..numbers.len() {
            let n = numbers[i];

            sum += n;

            if sum == e {
                range.1 = i + 1;
                return range;
            } else if sum > e {
                break;
            }
        }

        range.0 += 1;
        sum = 0;

        if range.0 == numbers.len() {
            return (0, 0);
        }
    }

    range
}
