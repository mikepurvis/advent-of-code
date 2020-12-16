extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

#[derive(Debug)]
struct Mask {
    mask: u64,
    value: u64
}

#[derive(Debug)]
enum Instr {
    Mask(Mask),
    Mem {
        addr: u64,
        value: u64
    }
}

fn read_program(contents: &str) -> Vec<Instr> {
    let mask_re = Regex::new(r"^mask = ([01X]+)").unwrap();
    let mem_re = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)").unwrap();
    contents.trim().lines().map(|line| {
        let trimmed = line.trim();
        if let Some(mask_result) = mask_re.captures(trimmed) {
            let mask_str = mask_result.get(1).unwrap().as_str();
            let mask = u64::from_str_radix(
                &mask_str.replace("1", "0").replace("X", "1"), 2).unwrap();
            let value = u64::from_str_radix(&mask_str.replace("X", "0"), 2).unwrap();
            Instr::Mask(Mask { mask, value })
        } else if let Some(mem_result) = mem_re.captures(trimmed) {
            let addr = mem_result.get(1).unwrap().as_str().parse().unwrap();
            let value = mem_result.get(2).unwrap().as_str().parse().unwrap();
            Instr::Mem { addr, value }
        } else {
            // All input lines should match one of the above cases.
            unreachable!();
        }
    }).collect()
}

fn run_program(program: &[Instr]) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut current_mask: Option<&Mask> = None;
    for instr in program.iter() {
        match instr {
            Instr::Mask(m) => current_mask = Some(&m),
            Instr::Mem { addr, value } => {
                let result = value & current_mask.unwrap().mask
                                   | current_mask.unwrap().value;
                memory.insert(*addr, result);
            }
        }
    }
    memory
}

fn run_program_v2(program: &[Instr]) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut current_mask: Option<&Mask> = None;
    let mut mask_offsets = Vec::new();
    for instr in program.iter() {
        match instr {
            Instr::Mask(m) => {
                current_mask = Some(&m);
                // Break down the mask into a vector for each bit that is
                // participating, then get a vector of all the sums of
                // combinations of those bits as memory offsets.
                let mask_bits = (0..36u64).filter_map(|bit| {
                    let val = m.mask & (1 << bit);
                    if val == 0 { None } else { Some(val) }
                }).collect::<Vec<_>>();
                mask_offsets = (0..=mask_bits.len()).map(|len| {
                    mask_bits.iter().copied().combinations(len)
                }).flatten().map(|group| {
                    group.iter().sum::<u64>()
                }).collect();
            }
            Instr::Mem { addr, value } => {
                // First set the ones and zero out the masked "floating" bits,
                // then apply all possible memory offsets as computed before.
                let addr_base = addr & !current_mask.unwrap().mask
                                     | current_mask.unwrap().value;
                for offset in &mask_offsets {
                    memory.insert(addr_base + offset, *value);
                }
            }
        }
    }
    memory
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let program = read_program(&contents);
    let memory = run_program(&program);
    println!("Memory sum v1: {}", memory.values().sum::<u64>());

    let memory2 = run_program_v2(&program);
    println!("Memory sum v2: {}", memory2.values().sum::<u64>());
}

#[test]
fn test_sample_v1() {
    const SAMPLE: &str = r#"
    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0
    "#;

    let program = read_program(SAMPLE);
    let memory = run_program(&program);
    assert_eq!(memory.values().sum::<u64>(), 165);
}

#[test]
fn test_sample_v2() {
    const SAMPLE: &str = r#"
    mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1
    "#;

    let program = read_program(SAMPLE);
    let memory = run_program_v2(&program);
    assert_eq!(memory.values().sum::<u64>(), 208);
}
