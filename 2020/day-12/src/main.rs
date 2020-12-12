extern crate euclid;

use euclid::Vector2D;
use std::fs;
use std::vec::Vec;

#[derive(Debug)]
struct Instr {
    action: char,
    value: i32
}

type Point = Vector2D<i32, i32>;

fn program_from_contents(contents: &str) -> Vec<Instr> {
    contents.lines().map(|line| {
        let mut ch = line.trim().chars();
        Instr {
            action: ch.next().unwrap(),
            value: ch.as_str().parse::<i32>().unwrap()
        }
    }).collect()
}

fn run_program1(program: &[Instr]) -> Point {
    let mut ship_pose = Point::new(0, 0);
    let mut facing: i32 = 360 * 10;

    for instr in program.iter() {
        match instr.action {
            'L' => facing += instr.value,
            'R' => facing -= instr.value,
            _ => {
                let dir = if instr.action == 'F' {
                    match facing % 360 {
                        0 => 'E', 90 => 'N', 180 => 'W', 270 => 'S', _ => unreachable!()
                    }
                } else { instr.action };
                ship_pose += match dir {
                    'E' => Point::new(1, 0),
                    'W' => Point::new(-1, 0),
                    'N' => Point::new(0, -1),
                    'S' => Point::new(0, 1),
                    _ => unreachable!()
                } * instr.value;
            }
        }
    }
    ship_pose
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let program = program_from_contents(&contents);
    let result1 = run_program1(&program);
    println!("{}", result1.x + result1.y)
}


#[test]
fn test_sample() {
    const SAMPLE: &str = "\
    F10
    N3
    F7
    R90
    F11";
    let program = program_from_contents(SAMPLE);
    let result1 = run_program1(&program);
    assert_eq!(result1.x + result1.y, 25)
}
