use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("src/day3/input_day3.txt")?;
    let f = BufReader::new(f);

    let mut map: Vec<Vec<char>> = vec![];

    for line in f.lines() {
        let line = line.unwrap();
        let mut l: Vec<char> = vec![];
        for c in line.chars() {
            l.push(c);
        }

        map.push(l);
    }

    let count1 = do_part1(&map);
    let count2 = do_part2(&map);
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);

    Ok(())
}

fn do_part1(map: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    // let column_count = map.len();
    let row_count = map.first().unwrap().len();

    let mut x = 0;
    let mut y = 0;

    while y < map.len() - 1 {
        x += 3;
        y += 1;

        let cell = map[y][x % row_count];
        let r = match cell {
            '#' => 1,
            '.' => 0,
            _ => 0,
        };

        count += r;
    }

    count
}

fn do_part2(map: &Vec<Vec<char>>) -> i64 {
    let result1 = calculate_slope(map, 1, 1) as i64;
    let result2 = calculate_slope(map, 3, 1) as i64;
    let result3 = calculate_slope(map, 5, 1) as i64;
    let result4 = calculate_slope(map, 7, 1) as i64;
    let result5 = calculate_slope(map, 1, 2) as i64;

    return result1 * result2 * result3 * result4 * result5;
}

fn calculate_slope(map: &Vec<Vec<char>>, x_slope: usize, y_slope: usize) -> i32 {
    let mut count = 0;

    // let column_count = map.len();
    let row_count = map.first().unwrap().len();

    let mut x = 0;
    let mut y = 0;

    while y < map.len() - 1 {
        x += x_slope;
        y += y_slope;

        let cell = map[y][x % row_count];
        let r = match cell {
            '#' => 1,
            '.' => 0,
            _ => 0,
        };

        count += r;
    }

    count
}
