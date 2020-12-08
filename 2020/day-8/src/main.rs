extern crate bit_vec;

use bit_vec::BitVec;
use itertools::Itertools;
use std::vec::Vec;
use std::fs;


#[derive(Debug)]
enum Op {
    Nop,
    Acc { arg: isize },
    Jmp { arg: isize }
}

type Program = Vec<Op>;

fn program_from_contents(contents: &str) -> Program
{
    let mut program = Vec::new();
    for line in contents.lines() {
        program.push(match line.split_whitespace().next_tuple().unwrap() {
            ("nop", _) => Op::Nop,
            ("acc", arg) => Op::Acc { arg: arg.parse().unwrap() },
            ("jmp", arg) => Op::Jmp { arg: arg.parse().unwrap() },
            _ => unreachable!()
        });
    }
    return program;
}

struct Machine {
    pc: usize,
    acc: isize
}

impl Machine {
    fn new() -> Self {
        Self { pc: 0, acc: 0 }
    }
    fn run_until_repeat(&mut self, program: &Program) {
        let mut log = BitVec::from_elem(program.len(), false);
        while !log[self.pc] {
            log.set(self.pc, true);
            match program[self.pc] {
                Op::Nop => self.pc += 1,
                Op::Acc { arg } => { self.acc += arg; self.pc += 1 },
                Op::Jmp { arg } if arg < 0 => self.pc -= -arg as usize,
                Op::Jmp { arg } => self.pc += arg as usize
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let program = program_from_contents(&contents);
    let mut machine = Machine::new();
    machine.run_until_repeat(&program);
    println!("Accumulator: {}", machine.acc);
}

#[test]
fn test_sample() {
    const SAMPLE_INPUT: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;
    let program = program_from_contents(SAMPLE_INPUT);
    let mut machine = Machine::new();
    machine.run_until_repeat(&program);
    assert_eq!(machine.acc, 5);
}
