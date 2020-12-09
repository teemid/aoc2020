use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
struct VM {
    did_break_on_inf_loop: bool,
    completed_instructions: HashSet<usize>,
    instruction_pointer: usize,
    accumulator: i32,
    ops: Vec<Op>,
}

impl VM {
    fn new(ops: Vec<Op>) -> VM {
        VM {
            did_break_on_inf_loop: false,
            completed_instructions: HashSet::new(),
            instruction_pointer: 0,
            accumulator: 0,
            ops: ops,
        }
    }

    fn get_accumulator(&self) -> i32 {
        self.accumulator
    }

    fn run(&mut self) {
        while self.instruction_pointer != self.ops.len() - 1 {
            let op = &self.ops[self.instruction_pointer];

            if self.completed_instructions.contains(&self.instruction_pointer) {
                self.did_break_on_inf_loop = true;
                return;
            }

            self.completed_instructions.insert(self.instruction_pointer);

            match op {
                Op::Nop(_) => self.nop(),
                Op::Acc(arg) => self.acc(*arg),
                Op::Jmp(arg) => self.jmp(*arg),
            }
        }
    }

    fn try_repair_code(&mut self) {
        let instructions = self.find_jumps_and_nops();

        for instruction in instructions {
            {
                let op = self.ops.get_mut(instruction).unwrap();
                // NOTE (Emil): Convert instruction.
                match op {
                    Op::Nop(arg) => *op = Op::Jmp(*arg),
                    Op::Jmp(arg) => *op = Op::Nop(*arg),
                    _ => (),
                }
            }

            self.run();

            if !self.did_break_on_inf_loop {
                return;
            }

            self.did_break_on_inf_loop = false;
            self.accumulator = 0;
            self.instruction_pointer = 0;
            self.completed_instructions = HashSet::new();

            {
                let op = self.ops.get_mut(instruction).unwrap();
                // NOTE (Emil): Convert instruction back.
                match op {
                    Op::Nop(arg) => *op = Op::Jmp(*arg),
                    Op::Jmp(arg) => *op = Op::Nop(*arg),
                    _ => (),
                }
            }
        }
    }

    fn find_jumps_and_nops(&self) -> Vec<usize> {
        let mut instructions = vec![];

        for (i, op) in self.ops.iter().enumerate() {
            match op {
                Op::Nop(_) => instructions.push(i),
                Op::Jmp(_) => instructions.push(i),
                _ => ()
            }
        }

        instructions
    }

    fn nop(&mut self) {
        self.instruction_pointer += 1;
    }

    fn acc(&mut self, arg: i32) {
        self.accumulator += arg;
        self.instruction_pointer += 1;
    }

    fn jmp(&mut self, arg: i32) {
        let uarg = arg as usize;
        if arg.is_negative() {
            self.instruction_pointer -= arg.wrapping_abs() as u32 as usize;
        } else {
            self.instruction_pointer += uarg;
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open("src/day8/input_day8.txt")?;
    let f = BufReader::new(f);

    let mut ops1 = vec![];

    for line in f.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" ").collect();

        let op_name = parts[0];
        let arg: i32 = parts[1].parse().unwrap();

        let op = match op_name {
            "acc" => Op::Acc(arg),
            "nop" => Op::Nop(arg),
            "jmp" => Op::Jmp(arg),
            _ => panic!("Unsupported op!"),
        };

        ops1.push(op);
    }

    let ops2 = ops1.clone();

    let accumulator1 = part1_run(ops1);
    let accumulator2 = part2_run(ops2);

    println!("Part 1: {}", accumulator1);
    println!("Part 2: {}", accumulator2);

    Ok(())
}

fn part1_run(ops: Vec<Op>) -> i32 {
    let mut vm = VM::new(ops);
    vm.run();

    vm.get_accumulator()
}

fn part2_run(ops: Vec<Op>) -> i32 {
    let mut vm = VM::new(ops);
    vm.try_repair_code();

    vm.get_accumulator()
}
