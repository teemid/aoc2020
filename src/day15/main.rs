use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::collections::HashMap;
use std::env;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let part = parse_part(&args);

    let mut f = File::open("src/day15/input_day15.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let mut rounds = 2020;
    if part == Part::Part2 {
        rounds = 30000000;
    }

    let numbers = parse(&s);
    let last = play(&numbers, rounds);

    println!("{:?}", last);

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

fn parse(s: &String) -> Vec<usize> {
    let ns: Vec<&str> = s.split(',').collect();
    println!("{:?}", ns);
    let mut input = vec![];
    for n in ns {
        let n: usize = n.trim_end().parse().unwrap();
        input.push(n);
    }

    input
}

fn play(starting_numbers: &Vec<usize>, rounds: usize) -> usize {
    let mut numbers = vec![];
    let mut last_turn: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut last = 0;

    for (i, n) in starting_numbers.iter().enumerate() {
        let mut count = 0;
        last = *n;
        numbers.push(last);
        if last_turn.contains_key(&last) {
            let (c, _) = last_turn.get(n).unwrap();
            count = *c;
        }

        if i != starting_numbers.len() - 1 {
            last_turn.insert(last, (count + 1, i + 1));
        }
    }

    let start = numbers.len();
    for i in start..rounds {
        let prev_turn = i;

        let mut next = 0;
        let mut count = 1;

        if last_turn.contains_key(&last) {
            let (c, t) = last_turn.get(&last).unwrap();
            count = *c;

            next = prev_turn - *t;
        }

        last_turn.insert(last, (count + 1, prev_turn));

        last = next;
        numbers.push(last);
    }

    last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let starting_numbers = vec![0,3,6];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 436);
    }

    #[test]
    fn test_example2() {
        let starting_numbers = vec![1,3,2];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 1);
    }

    #[test]
    fn test_example3() {
        let starting_numbers = vec![2,1,3];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 10);
    }

    #[test]
    fn test_example4() {
        let starting_numbers = vec![1,2,3];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 27);
    }

    #[test]
    fn test_example5() {
        let starting_numbers = vec![2,3,1];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 78);
    }

    #[test]
    fn test_example6() {
        let starting_numbers = vec![3,2,1];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 438);
    }

    #[test]
    fn test_example7() {
        let starting_numbers = vec![3,1,2];
        let last = play(&starting_numbers, 2020);

        assert_eq!(last, 1836);
    }
}
