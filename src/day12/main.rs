use std::fs::File;
use std::io::prelude::*;
use std::io::{self};

#[derive(Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug)]
struct Ship {
    direction: i32,
    east: i32,
    north: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: 90,
            east: 0,
            north: 0,
        }
    }
}

#[derive(Debug)]
struct Waypoint {
    rotation: i32,
    east: i32,
    north: i32,
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint {
            rotation: 0,
            east: 10,
            north: 1,
        }
    }
}

fn main() -> io::Result<()> {
    let mut f = File::open("src/day12/input_day12.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let instructions = parse(&s);

    let mut ship = Ship::new();
    let distance = apply_instructions(&mut ship, &instructions);
    println!("Part 1 distance: {}", distance);

    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    let distance = apply_instructions_to_waypoint(&mut ship, &mut waypoint, &instructions);
    println!("Part 2 distance: {}", distance);

    Ok(())
}

fn parse(s: &String) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in s.lines() {
        let c = line.chars().next().unwrap();
        let rest = line.get(1..line.len()).unwrap();
        let number: i32 = rest.parse().unwrap();
        match c {
            'N' => instructions.push(Instruction::North(number)),
            'S' => instructions.push(Instruction::South(number)),
            'E' => instructions.push(Instruction::East(number)),
            'W' => instructions.push(Instruction::West(number)),
            'L' => instructions.push(Instruction::Left(number)),
            'R' => instructions.push(Instruction::Right(number)),
            'F' => instructions.push(Instruction::Forward(number)),
            _ => (),
        }
    }

    instructions
}

fn apply_instructions(ship: &mut Ship, instructions: &Vec<Instruction>) -> u32 {
    for instruction in instructions {
        match instruction {
            Instruction::North(count) => ship.north += count,
            Instruction::South(count) => ship.north -= count,
            Instruction::East(count) => ship.east += count,
            Instruction::West(count) => ship.east -= count,
            Instruction::Left(rotation) => ship.direction -= rotation,
            Instruction::Right(rotation) => ship.direction += rotation,
            Instruction::Forward(count) => {
                match ship.direction % 360 {
                    0 => ship.north += count,
                    90 => ship.east += count,
                    180 => ship.north -= count,
                    270 => ship.east -= count,
                    _ => panic!("Unknown direction {}", ship.direction),
                }
            },
        }
    }

    manhattan_distance(ship.east, ship.north)
}

fn apply_instructions_to_waypoint(ship: &mut Ship, waypoint: &mut Waypoint, instructions: &Vec<Instruction>) -> u32 {
    for instruction in instructions {
        match instruction {
            Instruction::North(count) => waypoint.north += count,
            Instruction::South(count) => waypoint.north -= count,
            Instruction::East(count) => waypoint.east += count,
            Instruction::West(count) => waypoint.east -= count,
            Instruction::Left(rotation) => {
                let (east, north) = rotate_waypoint_left(waypoint.east, waypoint.north, rotation);
                waypoint.east = east;
                waypoint.north = north;
            },
            Instruction::Right(rotation) => {
                let (east, north) = rotate_waypoint_right(waypoint.east, waypoint.north, rotation);
                waypoint.east = east;
                waypoint.north = north;
            },
            Instruction::Forward(count) => {
                ship.north += count * waypoint.north;
                ship.east += count * waypoint.east;
            },
        }
    }

    manhattan_distance(ship.east, ship.north)
}

fn rotate_waypoint_right(east: i32, north: i32, rotation: &i32) -> (i32, i32) {
    let mut result = (0, 0);

    match rotation {
        0 => result = (east, north),
        90 => result = (north, -east),
        180 => result = (-east, -north),
        270 => result = (-north, east),
        _ => (),
    }

    result
}

fn rotate_waypoint_left(east: i32, north: i32, rotation: &i32) -> (i32, i32) {
    let mut result = (0, 0);

    match rotation {
        0 => result = (east, north),
        90 => result = (-north, east),
        180 => result = (-east, -north),
        270 => result = (north, -east),
        _ => (),
    }

    result
}

fn manhattan_distance(x: i32, y: i32) -> u32 {
    let d = x.abs() + y.abs();

    d as u32
}