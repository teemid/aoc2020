use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("src/day4/input_day4.txt")?;
    let f = BufReader::new(f);

    let mut entries = vec![];

    let mut entry = String::new();
    for line in f.lines() {
        let line = line.unwrap();

        if line == "" {
            entries.push(entry);
            entry = String::new();
            continue;
        }

        if !entry.is_empty() {
            entry.push_str(" ");
        }

        entry.push_str(&line);
    }

    entries.push(entry);

    let mut passports = vec![];
    for entry in entries {
        let attributes: Vec<&str> = entry.split(" ").collect();
        let mut passport = HashMap::new();

        for attr in attributes {
            let key_value: Vec<&str> = attr.split(":").collect();
            let key = key_value.first().unwrap();
            let key = key.to_string();
            let value = key_value.last().unwrap();
            let value = value.to_string();

            passport.insert(key, value);
        }

        passports.push(passport);
    }

    let mut valid_count = 0;
    for passport in passports {
        if check_passport(&passport) {
            valid_count += 1;
        }
    }

    println!("Number of valid passports: {}", valid_count);

    Ok(())
}

fn check_passport(passport: &HashMap<String, String>) -> bool {
    let expected_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for expected in expected_fields.iter() {
        let s = expected.to_string();
        if !passport.contains_key(&s) {
            return false
        }

        let value = passport.get(&s);
        let is_valid = match value {
            Some(val) => validate_field(&s, val),
            _ => false
        };

        if !is_valid {
            return false
        }
    }

    true
}

fn validate_field(field: &String, value: &String) -> bool {
    match field.as_str() {
        "byr" => validate_byr(value),
        "iyr" => validate_iyr(value),
        "eyr" => validate_eyr(value),
        "hgt" => validate_hgt(value),
        "hcl" => validate_hcl(value),
        "ecl" => validate_ecl(value),
        "pid" => validate_pid(value),
        _ => false
    }
}

fn validate_byr(value: &String) -> bool {
    if value.len() != 4 {
        return false
    }

    match value.parse() {
        Ok(byr) => (1920 <= byr && byr <= 2002),
        _ => false
    }
}

fn validate_iyr(value: &String) -> bool {
    if value.len() != 4 {
        return false
    }

    match value.parse() {
        Ok(byr) => (2010 <= byr && byr <= 2020),
        _ => false
    }
}

fn validate_eyr(value: &String) -> bool {
    if value.len() != 4 {
        return false
    }

    match value.parse() {
        Ok(byr) => (2020 <= byr && byr <= 2030),
        _ => false
    }
}

fn validate_hgt(value: &String) -> bool {
    if value.ends_with("in") {
        let len = value.len();
        let value = &value[..len-2];
        let result = match value.parse() {
            Ok(height) => (59 <= height && height <= 76),
            _ => false
        };

        return result;
    }

    if value.ends_with("cm") {
        let len = value.len();
        let value = &value[..len-2];
        let result = match value.parse() {
            Ok(height) => (150 <= height && height <= 193),
            _ => false
        };

        return result;
    }

    false
}

fn validate_hcl(value: &String) -> bool {
    if value.len() != 7 {
        return false;
    }

    for (i, c) in value.chars().enumerate() {
        if i == 0 {
            if c != '#' {
                return false;
            }

            continue;
        }

        if ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') {
            continue;
        }

        return false;
    }

    true
}

fn validate_ecl(value: &String) -> bool {
    match value.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn validate_pid(value: &String) -> bool {
    if value.len() != 9 {
        return false
    }

    for c in value.chars() {
        if '0' > c || c > '9' {
            return false;
        }
    }

    true
}
