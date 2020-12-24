use std::fs::File;
use std::io::prelude::*;
use std::io::{self};
use std::env;

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2
}

#[derive(Debug, PartialEq)]
enum Op {
    Plus,
    Mul,
    LeftParen,
    RightParen,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let part = parse_part(&args);

    let mut f = File::open("src/day18/input_day18.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    match part {
        Part::Part1 => {
            let mut sum = 0;
            for line in s.lines() {
                sum += evaluate(line, &precedence_part1);
            }

            println!("Sum of lines: {}", sum);
        },
        Part::Part2 => {
            let mut sum = 0;
            for line in s.lines() {
                sum += evaluate(line, &precedence_part2);
            }

            println!("Sum of lines: {}", sum);
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

fn evaluate(s: &str, precedence: &dyn Fn(&Op) -> usize) -> u64 {
    let mut output = vec![];
    let mut ops = vec![];

    for c in s.chars() {
        match c {
            '0'..='9' => {
                let n = c.to_digit(10).unwrap() as u64;
                output.push(n);
            },
            '+' => {
                let op = Op::Plus;
                if check_ops(&op, &ops, precedence) {
                    evaluate_op(&mut ops, &mut output);
                }
                ops.push(op);
            },
            '*' => {
                let op = Op::Mul;
                if check_ops(&op, &ops, precedence) {
                    evaluate_op(&mut ops, &mut output);
                }
                ops.push(op);
            },
            '(' => {
                ops.push(Op::LeftParen);
            },
            ')' => {
                let last = &ops[ops.len() - 1];
                let mut is_left_paren = *last == Op::LeftParen;

                while !is_left_paren {
                    evaluate_op(&mut ops, &mut output);

                    let last = &ops[ops.len() - 1];
                    is_left_paren = *last == Op::LeftParen;
                }

                ops.pop();
            }
            _ => (),
        }
    }

    while ops.len() > 0 {
        evaluate_op(&mut ops, &mut output);
    }

    output[0]
}

fn check_ops(op: &Op, ops: &Vec<Op>, precedence: &dyn Fn(&Op) -> usize) -> bool {
    if ops.len() == 0 {
        return false;
    }

    let top = &ops[ops.len() - 1];
    let p1 = precedence(op);
    let p2 = precedence(top);

    p1 >= p2
}

fn evaluate_op(ops: &mut Vec<Op>, output: &mut Vec<u64>) {
    if ops.len() == 0 {
        return;
    }

    if ops[ops.len() - 1] == Op::LeftParen {
        return;
    }

    let op = ops.pop().unwrap();
    let a = output.pop().unwrap();
    let b = output.pop().unwrap();

    match op {
        Op::Plus => output.push(a + b),
        Op::Mul => output.push(a * b),
        _ => (),
    }
}

fn precedence_part1(op: &Op) -> usize {
    match op {
        Op::LeftParen => 0,
        Op::Plus => 1,
        Op::Mul => 1,
        _ => 3
    }
}

fn precedence_part2(op: &Op) -> usize {
    match op {
        Op::LeftParen => 0,
        Op::Plus => 1,
        Op::Mul => 2,
        _ => 3
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let e = String::from("1 + 2 * 3 + 4 * 5 + 6");

        let result = evaluate(&e, &precedence_part1);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_example2() {
        let e = String::from("1 + (2 * 3) + (4 * (5 + 6))");

        let result = evaluate(&e, &precedence_part1);
        assert_eq!(result, 51);
    }

    #[test]
    fn test_example3() {
        let e = String::from("2 * 3 + (4 * 5)");

        let result = evaluate(&e, &precedence_part1);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_example4() {
        let e = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");

        let result = evaluate(&e, &precedence_part1);
        assert_eq!(result, 437);
    }

    #[test]
    fn test_example5() {
        let e = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");

        let result = evaluate(&e, &precedence_part1);
        assert_eq!(result, 12240);
    }

    #[test]
    fn test_example6() {
        let e = String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        let result = evaluate(&e, &precedence_part1);
        assert_eq!(result, 13632);
    }

    #[test]
    fn test_part2_example1() {
        let e = String::from("1 + 2 * 3 + 4 * 5 + 6");

        let result = evaluate(&e, &precedence_part2);
        assert_eq!(result, 231);
    }

    #[test]
    fn test_part2_example2() {
        let e = String::from("1 + (2 * 3) + (4 * (5 + 6))");

        let result = evaluate(&e, &precedence_part2);
        assert_eq!(result, 51);
    }

    #[test]
    fn test_part2_example3() {
        let e = String::from("2 * 3 + (4 * 5)");

        let result = evaluate(&e, &precedence_part2);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_part2_example4() {
        let e = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");

        let result = evaluate(&e, &precedence_part2);
        assert_eq!(result, 1445);
    }

    #[test]
    fn test_part2_example5() {
        let e = String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");

        let result = evaluate(&e, &precedence_part2);
        assert_eq!(result, 669060);
    }

    #[test]
    fn test_part2_example6() {
        let e = String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");

        let result = evaluate(&e, &precedence_part2);
        assert_eq!(result, 23340);
    }
}
