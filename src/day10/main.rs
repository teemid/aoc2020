use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("src/day10/input_day10.txt")?;
    let f = BufReader::new(f);

    let mut adapters: Vec<u32> = vec![];

    for line in f.lines() {
        let line = line.unwrap();

        adapters.push(line.parse().unwrap());
    }

    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);

    let mut count = HashMap::new();
    let mut current = 0;
    for i in 0..adapters.len() {
        let next = adapters[i];
        let difference = next - current;

        let entry = count.entry(difference).or_insert(0);
        *entry += 1;

        current = next;
    }

    let ones = count.get(&1).unwrap();
    let threes = count.get(&3).unwrap();

    println!("{}", ones * threes);

    Ok(())
}
