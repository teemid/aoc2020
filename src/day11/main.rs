use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

type Seats = Vec<Vec<char>>;

fn main() -> io::Result<()> {
    let f = File::open("src/day11/input_day11.txt")?;
    let f = BufReader::new(f);

    let mut seats = vec![];

    for line in f.lines() {
        let line = line.unwrap();
        let mut l = vec![];

        for c in line.chars() {
            l.push(c);
        }

        seats.push(l);
    }

    // print_seating(&seats);

    let height = seats.len();
    let width = seats.first().unwrap().len();
    let mut prev = seats;
    loop {
        let next = apply_rules(&prev, width, height);

        if compare(&prev, &next, (width, height)) {
            break;
        }

        // print_seating(&next);

        prev = next;
    }

    let occupied_seats = count_occupied_seats(&prev);
    println!("occupied seats: {}", occupied_seats);

    Ok(())
}

fn count_occupied_seats(seats: &Seats) -> u32 {
    let mut occupied_seats = 0;

    for line in seats {
        for c in line {
            if *c == '#' {
                occupied_seats += 1;
            }
        }
    }

    occupied_seats
}

fn print_seating(seats: &Seats) {
    for line in seats {
        for c in line {
            print!("{}", c);
        }

        print!("\n");
    }

    print!("\n");
}

fn compare(prev: &Seats, current: &Seats, size: (usize, usize)) -> bool {
    let (width, height) = size;

    for i in 0..height {
        for j in 0..width {
            if prev[i][j] != current[i][j] {
                return false;
            }
        }
    }

    true
}

fn apply_rules(seats: &Seats, width: usize, height: usize) -> Seats {
    let mut next: Seats = vec![];
    next.reserve(height);

    for i in 0..height {
        let mut line = vec![];
        line.resize(width, '.');
        next.push(line);

        for j in 0..width {
            // print_seating(&seats);
            let c = seats[i][j];
            match c {
                '.' => next[i][j] = '.',
                'L' => {
                    let occupied_seats = adjacent_occupied_seats(&seats, (width, height), (i, j));
                    // println!("({}, {}): {}", i, j, occupied_seats);
                    if occupied_seats == 0 {
                        next[i][j] = '#';
                    } else {
                        next[i][j] = 'L';
                    }
                }
                '#' => {
                    let occupied_seats = adjacent_occupied_seats(&seats, (width, height), (i, j));
                    // println!("({}, {}): {}", i, j, occupied_seats);
                    if occupied_seats >= 4 {
                        next[i][j] = 'L';
                    } else {
                        next[i][j] = '#';
                    }
                }
                _ => (),
            }
        }
    }

    next
}

fn adjacent_occupied_seats(seats: &Seats, size: (usize, usize), coord: (usize, usize)) -> usize {
    let (width, height) = size;
    let (x, y) = coord;

    let width = width as isize;
    let height = height as isize;
    let x = x as isize;
    let y = y as isize;

    let mut occupied_seats = 0;
    let x_start = x - 1;
    let x_end = x + 2;
    let y_start = y - 1;
    let y_end = y + 2;

    for i in x_start..x_end {
        for j in y_start..y_end {
            if i < 0 || j < 0 {
                continue;
            }

            if i >= height || j >= width {
                continue;
            }

            let x_coord = i as usize;
            let y_coord = j as usize;

            if coord == (x_coord, y_coord) {
                continue;
            }

            // print!("({}, {})", i, j);
            if seats[x_coord][y_coord] == '#' {
                occupied_seats += 1;
            }
        }
    }

    // println!("{}", occupied_seats);

    occupied_seats
}
