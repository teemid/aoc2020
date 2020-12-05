use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("src/day5/input_day5.txt")?;
    let f = BufReader::new(f);

    let mut rows: Vec<Vec<char>> = vec![];
    for _ in 0..127 {
        let mut columns: Vec<char> = vec![];
        for _ in 0..8 {
            columns.push(' ');
        }
        rows.push(columns);
    }

    let mut highest_seat_id = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let (row, column) = parse_boarding_pass(&line);
        let seat_id = row * 8 + column;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }

        rows[row as usize][column as usize] = 'O';
    }

    println!("Highest seat id: {}", highest_seat_id);

    let mut seat = (0, 0);
    for i in 0..rows.len() {
        let row = &rows[i];
        let mut count = 0;
        let mut free_seat = 0;
        for j in 0..row.len() {
            let cell = row[j];
            if cell == 'O' {
                count += 1;
            }

            if cell == ' ' {
                free_seat = j;
            }
        }

        if count == 7 {
            seat = (i, free_seat)
        }
    }

    println!("{:?}", seat.0 * 8 + seat.1);

    Ok(())
}

fn nop() {

}

fn parse_boarding_pass(boarding_pass: &str) -> (i32, i32) {
    let mut row = (0, 127);
    let mut col = (0, 7);

    for c in boarding_pass.chars() {
        match c {
            'F' => take_front(&mut row),
            'B' => take_back(&mut row),
            'R' => take_right(&mut col),
            'L' => take_left(&mut col),
            _ => nop(),
        }
    }

    (row.0, col.0)
}

fn take_front(row: &mut (i32, i32)) {
    let diff = (row.1 - row.0) / 2;
    row.1 = row.0 + diff;
}

fn take_back(row: &mut (i32, i32)) {
    let diff = (row.1 - row.0) / 2;
    row.0 = row.0 + diff + 1;
}

fn take_left(col: &mut (i32, i32)) {
    let diff = (col.1 - col.0) / 2;
    col.1 = col.0 + diff;
}

fn take_right(col: &mut (i32, i32)) {
    let diff = (col.1 - col.0) / 2;
    col.0 = col.0 + diff + 1;
}