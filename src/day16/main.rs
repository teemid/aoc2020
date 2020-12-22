use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::collections::{HashMap, HashSet};
use std::env;
use std::ops::Range;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2
}

type Limits = HashMap<String, (Range<usize>, Range<usize>)>;
type PossibleFields<'a> = Vec<&'a String>;
type PossibleTicketFields<'a> = Vec<PossibleFields<'a>>;
type Ticket = Vec<usize>;

#[derive(Debug)]
struct Data {
    limits: Limits,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let part = parse_part(&args);

    let mut f = File::open("src/day16/input_day16.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let data = parse(&s);

    match part {
        Part::Part1 => {
            let error_rate = calulate_ticket_error_rate(&data);
            println!("error rate: {}", error_rate);
        },
        Part::Part2 => {
            let fields = determine_fields(&data);
            let result = calculate_product(&data, &fields);
            println!("Product: {}", result);
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

fn parse(s: &String) -> Data {
    let parts: Vec<&str> = s.split("\r\n\r\n").collect();

    let limits = parts[0];
    let my_ticket = parts[1];
    let nearby_tickets = parts[2];

    let limits = parse_limits(&limits);
    let my_ticket = parse_my_ticket(&my_ticket);
    let nearby_tickets = parse_tickets(&nearby_tickets);

    Data { limits: limits, my_ticket: my_ticket, nearby_tickets: nearby_tickets }
}

fn parse_limits(limits: &str) -> HashMap<String, (Range<usize>, Range<usize>)> {
    let mut h = HashMap::new();

    for limit in limits.lines() {
        let parts: Vec<&str> = limit.split(": ").collect();

        let limit_name = parts[0];
        let limit_value = parts[1];

        let parts: Vec<&str> = limit_value.split(" or ").collect();

        let ps: Vec<&str> = parts[0].split("-").collect();
        let start: usize = ps[0].parse().unwrap();
        let end: usize = ps[1].parse().unwrap();

        let r1 = start..end + 1;

        let ps: Vec<&str> = parts[1].split("-").collect();
        let start: usize = ps[0].parse().unwrap();
        let end: usize = ps[1].parse().unwrap();

        let r2 = start..end + 1;

        h.insert(String::from(limit_name), (r1, r2));
    }

    h
}

fn parse_my_ticket(ticket: &str) -> Ticket {
    let mut my_ticket = vec![];

    for ticket in ticket.lines() {
        if ticket.starts_with("your") {
            continue;
        }

        let mut t = vec![];
        let parts: Vec<&str> = ticket.split(",").collect();
        for part in parts {
            let n: usize = part.parse().unwrap();
            t.push(n);
        }

        my_ticket = t;
        break;
    }

    my_ticket
}

fn parse_tickets(tickets: &str) -> Vec<Ticket> {
    let mut v = vec![];

    for ticket in tickets.lines() {
        if ticket.starts_with("nearby") {
            continue;
        }

        let mut t = vec![];
        let parts: Vec<&str> = ticket.split(",").collect();
        for part in parts {
            let n: usize = part.parse().unwrap();
            t.push(n);
        }

        v.push(t);
    }

    v
}

fn calulate_ticket_error_rate(data: &Data) -> usize {
    let mut ticket_error_rate = 0;

    for ticket in &data.nearby_tickets {
        let error = get_ticket_error(ticket, &data.limits);
        ticket_error_rate += error;
    }

    ticket_error_rate
}

fn get_ticket_error(ticket: &Vec<usize>, limits: &Limits) -> usize {
    for field in ticket {
        let mut errors = vec![];
        let mut is_valid_for_some_range = false;
        for (_, ranges) in limits {
            let (r1, r2) = ranges;

            if r1.contains(field) || r2.contains(field) {
                is_valid_for_some_range = true;
                break;
            }

            errors.push(field);
        }

        if !is_valid_for_some_range {
            return *errors[0];
        }
    }

    0
}

fn determine_fields<'a>(data: &'a Data) -> Vec<String> {
    let mut tickets = vec![];

    let mut field_overview = HashSet::new();
    for (field_name, _) in &data.limits {
        field_overview.insert(field_name);
    }

    for ticket in &data.nearby_tickets {
        let error = get_ticket_error(ticket, &data.limits);
        if error > 0 {
            continue;
        }

        let possibilities = get_possible_fields(ticket, &data.limits);
        tickets.push(possibilities);
    }

    let mut field_possibilities = HashMap::new();
    for ticket in &tickets {
        for (i, field) in ticket.iter().enumerate() {
            let collected = field_possibilities.entry(i).or_insert(HashMap::new());
            for possibility in field {
                let count = collected.entry(possibility).or_insert(0);
                *count += 1;
            }
        }
    }

    let number_of_tickets = tickets.len();
    let count = data.limits.len();
    let mut names = vec![];
    for _ in 0..count {
        names.push(String::from("unknown"));
    }

    loop {
        for i in 0..count {
            let possibilities = &field_possibilities[&i];
            let mut vs = vec![];
            if names[i] != "unknown" {
                continue;
            }

            for (field_name, field_count) in possibilities {
                if !field_overview.contains(*field_name) {
                    continue;
                }

                if *field_count == number_of_tickets {
                    vs.push(field_name);
                }
            }

            if vs.len() == 1 {
                let n = String::from(**vs[0]);
                field_overview.remove(&n);
                names[i] = n;
            }
        }

        if are_all_fields_determined(&names) {
            break;
        }
    }

    names
}

fn calculate_product(data: &Data, names: &Vec<String>) -> usize {
    let mut fields = vec![];
    for (i, name) in names.iter().enumerate() {
        if name.starts_with("departure") {
            fields.push(i);
        }
    }

    let mut result = 1;
    for field in fields {
        result *= data.my_ticket[field];
    }

    result
}

fn are_all_fields_determined(names: &Vec<String>) -> bool {
    for name in names {
        if name == "unknown" {
            return false;
        }
    }

    true
}

fn get_possible_fields<'a>(ticket: &Vec<usize>, limits: &'a Limits) -> PossibleTicketFields<'a> {
    let mut ticket_possibilities = vec![];

    for field in ticket {
        let mut possibilities = vec![];
        for (name, ranges) in limits {
            let (r1, r2) = ranges;

            if r1.contains(field) || r2.contains(field) {
                possibilities.push(name);
            }
        }

        ticket_possibilities.push(possibilities);
    }

    ticket_possibilities
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let mut f = File::open("src/day16/example_day16.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let data = parse(&s);

        let error_rate = calulate_ticket_error_rate(&data);

        assert_eq!(error_rate, 71);
    }
}
