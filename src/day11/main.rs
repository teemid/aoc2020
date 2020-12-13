use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::env;

type Seats = Vec<Vec<char>>;

enum Part {
    Part1,
    Part2
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut part = Part::Part1;
    if args.len() > 1 {
        let arg = &args[1];
        match arg.as_str() {
            "1" => part = Part::Part1,
            "2" => part = Part::Part2,
            _ => ()
        }
    }

    let mut f = File::open("src/day11/input_day11.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let seats = parse_seating(&s);

    match part {
        Part::Part1 => {
            print_seating(&seats);

            let height = seats.len();
            let width = seats.first().unwrap().len();
            let mut prev = seats;
            loop {
                let next = apply_rules_part1(&prev, width, height);

                if compare(&prev, &next, (width, height)) {
                    break;
                }

                prev = next;
            }

            let occupied_seats = count_occupied_seats(&prev);
            println!("occupied seats: {}", occupied_seats);
        }
        Part::Part2 => {
            print_seating(&seats);

            let height = seats.len();
            let width = seats.first().unwrap().len();
            let mut prev = seats;
            loop {
                let next = apply_rules_part2(&prev, width, height);

                print_seating(&next);

                if compare(&prev, &next, (width, height)) {
                    break;
                }

                prev = next;
            }

            let occupied_seats = count_occupied_seats(&prev);
            println!("occupied seats: {}", occupied_seats);
        }
    }

    Ok(())
}

fn parse_seating(s: &String) -> Seats {
    let mut seats: Seats = vec![];

    for line in s.lines() {
        let mut l = vec![];
        let line: String = line.split_whitespace().collect();

        for c in line.chars() {
            l.push(c);
        }

        seats.push(l);
    }

    seats
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

fn apply_rules_part1(seats: &Seats, width: usize, height: usize) -> Seats {
    let mut next: Seats = vec![];
    next.reserve(height);

    for i in 0..height {
        let mut line = vec![];
        line.resize(width, '.');
        next.push(line);

        for j in 0..width {
            let c = seats[i][j];
            match c {
                '.' => next[i][j] = '.',
                'L' => {
                    let occupied_seats = adjacent_occupied_seats(&seats, (width, height), (i, j));
                    if occupied_seats == 0 {
                        next[i][j] = '#';
                    } else {
                        next[i][j] = 'L';
                    }
                }
                '#' => {
                    let occupied_seats = adjacent_occupied_seats(&seats, (width, height), (i, j));
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

fn apply_rules_part2(seats: &Seats, width: usize, height: usize) -> Seats {
    let mut next: Seats = vec![];
    next.reserve(height);

    for y in 0..height {
        let mut line = vec![];
        line.resize(width, '.');
        next.push(line);

        for x in 0..width {
            let c = seats[y][x];
            match c {
                '.' => next[y][x] = '.',
                'L' => {
                    let occupied_seats = check_visible_seats(&seats, (width, height), (x, y));
                    if occupied_seats == 0 {
                        next[y][x] = '#';
                    } else {
                        next[y][x] = 'L';
                    }
                }
                '#' => {
                    let occupied_seats = check_visible_seats(&seats, (width, height), (x, y));
                    if occupied_seats >= 5 {
                        next[y][x] = 'L';
                    } else {
                        next[y][x] = '#';
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

            if seats[x_coord][y_coord] == '#' {
                occupied_seats += 1;
            }
        }
    }

    occupied_seats
}

fn check_visible_seats(seats: &Seats, size: (usize, usize), coord: (usize, usize)) -> usize {
    let mut occupied_seats = 0;

    let directions: [(isize, isize); 8] = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

    for direction in directions.iter() {
        let mut x: isize = coord.0 as isize;
        let mut y: isize = coord.1 as isize;

        let width = size.0 as isize;
        let height = size.1 as isize;
        let (x_dir, y_dir) = direction;

        loop {
            x += x_dir;
            y += y_dir;

            if x < 0 || x >= width {
                break;
            }

            if y < 0 || y >= height {
                break;
            }

            let c = seats[y as usize][x as usize];
            match c {
                'L' => break,
                '#' => {
                    occupied_seats += 1;
                    break;
                }
                _ => (),
            }
        }
    }

    occupied_seats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occupied_seat_function() {
        let s = String::from(
        r#".......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        ...#....."#);

        let seats = parse_seating(&s);
        let coord = (3, 4);
        let size = (9, 9);
        let count = check_visible_seats(&seats, size, coord);

        assert_eq!(count, 8);
    }

    #[test]
    fn test2() {
        let s = String::from(
        r#".............
        .L.L.#.#.#.#.
        ............."#);

        let seats = parse_seating(&s);
        let coord = (1, 1);
        let size = (13, 3);
        let count = check_visible_seats(&seats, size, coord);

        assert_eq!(count, 0);
    }

    #[test]
    fn test3() {
        let s = String::from(
        r#".##.##.
        #.#.#.#
        ##...##
        ...L...
        ##...##
        #.#.#.#
        .##.##."#);

        let seats = parse_seating(&s);
        let coord = (3, 3);
        let size = (7, 7);
        let count = check_visible_seats(&seats, size, coord);

        assert_eq!(count, 0);
    }
}