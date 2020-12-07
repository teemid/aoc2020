use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("src/day7/input_day7.txt")?;
    let f = BufReader::new(f);

    let mut nodes: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for line in f.lines() {
        let line = line.unwrap();

        let parts: Vec<&str> = line.split(" contain ").collect();
        let first = parts.first().unwrap();
        let second = parts.last().unwrap();
        let children: Vec<&str> = second.split(", ").collect();

        let mut cs: Vec<(String, usize)> = vec![];
        for child in children {
            if child == "no other bags." {
                continue;
            }

            let parts: Vec<&str> = child.split(" ").collect();
            if parts.len() == 4 {
                let count: usize = parts[0].parse().unwrap();
                let first = parts[1];
                let second = parts[2];

                let mut c = String::new();
                c.push_str(first);
                c.push_str(" ");
                c.push_str(second);

                cs.push((c, count));
            }
        }

        let parts: Vec<&str> = first.split(" ").collect();
        let first = parts[0];
        let second = parts[1];

        let mut key = String::new();
        key.push_str(first);
        key.push_str(" ");
        key.push_str(second);

        nodes.insert(key.to_string(), cs);
    }

    let s = "shiny gold".to_string();
    let mut count = 0;
    for key in nodes.keys() {
        if find(&s, key, &nodes) {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
    println!("Part 2: {}", count_bags(&s, &nodes));

    Ok(())
}

fn find(needle: &String, key: &String, nodes: &HashMap<String, Vec<(String, usize)>>) -> bool {
    let children = nodes.get(key).unwrap();
    if children.is_empty() {
        return false
    }

    for (child, _) in children {
        if child == needle {
            return true;
        } else if find(needle, child, nodes) {
            return true;
        }
    }

    false
}

fn count_bags(needle: &String, nodes: &HashMap<String, Vec<(String, usize)>>) -> usize {
    let mut total_count = 0;
    let children = nodes.get(needle).unwrap();
    if children.is_empty() {
        return total_count;
    }

    for (key, count) in children {
        total_count += count + (count * count_bags(key, nodes));
    }


    total_count
}