use std::fs::File;
use std::io::prelude::*;
use std::io::{self};

fn main() -> io::Result<()> {
    let mut f = File::open("src/day13/input_day13.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let (timestamp, bus_times) = parse(&s);

    let mut best_bus = 0;
    let mut wait_time = timestamp;
    for bus in bus_times {
        let o = timestamp / bus;
        let low = o * bus;
        let high = (o + 1) * bus;

        let mut bus_ts = low;
        if low < timestamp {
            bus_ts = high;
        }

        let diff = bus_ts - timestamp;
        if diff < wait_time {
            wait_time = diff;
            best_bus = bus;
        }
    }

    println!("Part 1: {}", wait_time * best_bus);

    Ok(())
}

fn parse(s: &String) -> (u32, Vec<u32>) {
    let mut bus_times = vec![];

    let lines: Vec<&str> = s.lines().collect();
    let timestamp: u32 = lines.first().unwrap().parse().unwrap();

    let busses = lines.last().unwrap();
    for bus in busses.split(',') {
        match bus {
            "x" => (),
            _ => bus_times.push(bus.parse().unwrap()),
        }
    }

    (timestamp, bus_times)
}