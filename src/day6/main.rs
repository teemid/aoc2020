use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let f = File::open("src/day6/input_day6.txt")?;
    let f = BufReader::new(f);

    let mut groups: Vec<String> = vec![];
    let mut group_size: Vec<usize> = vec![];
    group_size.push(0);

    let mut group = String::new();
    for line in f.lines() {
        let line = line.unwrap();
        if line == "" {
            groups.push(group);
            group = String::new();
            group_size.push(0);
        } else {
            let length = group_size.len();
            let count = group_size[length - 1];
            group_size[length - 1] = count + 1;

            group.push_str(&line);
        }
    }

    groups.push(group);

    let part1_answer = count_answers_part1(&groups);
    let part2_answer = count_answers_part2(&groups, &group_size);

    println!("Part1: Number of yes answers: {}", part1_answer);
    println!("Part2: Number of yes answers: {}", part2_answer);

    Ok(())
}

fn count_answers_part1(groups: &Vec<String>) -> usize {
    let mut yes_answer_count = 0;
    for group in groups {
        let mut chars: Vec<char> = group.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        chars.dedup();
        yes_answer_count += chars.len();
    }

    yes_answer_count
}

fn count_answers_part2(groups: &Vec<String>, group_size: &Vec<usize>) -> usize {
    let mut yes_answer_count = 0;
    for i in 0..groups.len() {
        let group = &groups[i];
        let group_count = group_size[i];

        let chars: Vec<char> = group.chars().collect();
        let mut answer_count: HashMap<char, usize> = HashMap::new();
        for c in chars {
            answer_count.entry(c).or_insert(0);
            if answer_count.contains_key(&c) {
                let count = answer_count.get_mut(&c).unwrap();
                *count = *count + 1;
            }
        }

        for (_, count) in answer_count {
            if count == group_count {
                yes_answer_count += 1;
            }
        }
    }

    yes_answer_count
}
