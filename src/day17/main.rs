use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::env;

mod dimension3;
mod dimension4;

use dimension3::PocketDimension3;
use dimension4::PocketDimension4;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let part = parse_part(&args);

    let mut f = File::open("src/day17/input_day17.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    match part {
        Part::Part1 => {
            let mut dimension = PocketDimension3::from_string(&s);
            dimension.draw_dimension();

            for gen in 1..7 {
                println!("gen: {}", gen);
                dimension.cycle();
                dimension.draw_dimension();
            }

            println!("Number of active cubes: {}", dimension.active_cubes());
        },
        Part::Part2 => {
            let mut dimension = PocketDimension4::from_string(&s);
            dimension.draw_dimension();

            for gen in 1..7 {
                println!("gen: {}", gen);
                dimension.cycle();
                dimension.draw_dimension();
            }

            println!("Number of active cubes: {}", dimension.active_cubes());
        }
    }

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
