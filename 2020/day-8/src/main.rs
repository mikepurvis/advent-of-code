extern crate bit_vec;

use bit_vec::BitVec;
use std::vec::Vec;
use std::fs;

#[derive(Debug, PartialEq)]
enum Exit { Repeat, OutOfBounds }

#[derive(Debug, Clone)]
enum OpCode { Nop, Acc, Jmp }

#[derive(Debug, Clone)]
struct Op {
    code: OpCode,
    arg: isize
}

type Program = Vec<Op>;

fn program_from_contents(contents: &str) -> Program {
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

struct ProgramMutations<'a> {
    program: &'a Program,
    index: usize
}

impl<'a> ProgramMutations<'_> {
    fn from_program(program: &'a Program) -> ProgramMutations {
        ProgramMutations { program: program, index: 0 }
    }
}

impl Iterator for ProgramMutations<'_> {
    type Item = Program;

    fn next(&mut self) -> Option<Program> {
        while self.index < self.program.len() {
            let new_code = match self.program[self.index].code {
                OpCode::Acc => { self.index += 1; continue },
                OpCode::Nop => OpCode::Jmp,
                OpCode::Jmp => OpCode::Nop
            };
            let mut program = self.program.clone();
            program[self.index].code = new_code;
            self.index += 1;
            return Some(program);
        }
        None
    }
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
            self.pc += 1;
            match op.code {
                OpCode::Nop => (),
                OpCode::Acc => self.acc += op.arg,
                OpCode::Jmp => {
                    self.pc -= 1;
                    if op.arg < 0 {
                        self.pc -= -op.arg as usize
                    } else {
                        self.pc += op.arg as usize
                    }
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

    for mutated_program in ProgramMutations::from_program(&program) {
        let mut machine = Machine::new();
        if machine.run(&mutated_program) == Exit::OutOfBounds {
            println!("Accumulator: {}", machine.acc);
            break;
        }
    }
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

    let mut found = false;
    for mutated_program in ProgramMutations::from_program(&program) {
        let mut machine = Machine::new();
        if machine.run(&mutated_program) == Exit::OutOfBounds {
            assert_eq!(machine.acc, 8);
            found = true;
        }
    }
    assert!(found, "Should have found an OutOfBounds mutation.");
}
