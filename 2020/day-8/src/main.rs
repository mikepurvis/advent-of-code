extern crate bit_vec;

use bit_vec::BitVec;
use std::vec::Vec;
use std::fs;

#[derive(Debug, PartialEq)]
enum Exit { Repeat, OutOfBounds }

enum OpCode { Nop, Acc, Jmp }

struct Op {
    code: OpCode,
    arg: isize
}

type Program = Vec<Op>;

fn program_from_contents(contents: &str) -> Program
{
    let mut program = Vec::new();
    for line in contents.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let code = match tokens[0] {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => unreachable!()
        };
        let arg = tokens[1].parse::<isize>().unwrap();
        program.push(Op { code: code, arg: arg })
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
    fn run(&mut self, program: &Program) -> Exit {
        let mut log = BitVec::from_elem(program.len(), false);

        loop {
            if self.pc >= program.len() {
                return Exit::OutOfBounds
            } else if log[self.pc] {
                return Exit::Repeat
            }

            log.set(self.pc, true);
            let op = &program[self.pc];
            match op.code {
                OpCode::Nop => self.pc += 1,
                OpCode::Acc => { self.acc += op.arg; self.pc += 1 },
                OpCode::Jmp => if op.arg < 0 {
                    self.pc -= -op.arg as usize
                } else {
                    self.pc += op.arg as usize
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let program = program_from_contents(&contents);
    let mut machine = Machine::new();
    machine.run(&program);
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
    let exit_code = machine.run(&program);
    assert_eq!(exit_code, Exit::Repeat);
    assert_eq!(machine.acc, 5);
}
