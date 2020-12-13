extern crate euclid;

use euclid::Vector2D;
use std::fs;
use std::vec::Vec;

#[derive(Debug)]
struct Instr {
    action: char,
    value: i32
}

type Vec2 = Vector2D<i32, i32>;

fn program_from_contents(contents: &str) -> Vec<Instr> {
    contents.lines().map(|line| {
        let mut ch = line.trim().chars();
        Instr {
            action: ch.next().unwrap(),
            value: ch.as_str().parse().unwrap()
        }
    }).collect()
}

fn rotate_vector(input: &Vec2, degrees: i32) -> Vec2 {
    let mut rotated = input.clone();
    for _rotations in 0..(degrees / 90) {
        rotated = Vec2::new(rotated.y, -rotated.x);
    }
    rotated
}

fn run_program1(program: &[Instr]) -> Vec2 {
    let mut ship_pose = Vec2::new(0, 0);
    let mut facing = Vec2::new(1, 0);

    for instr in program.iter() {
        match instr.action {
            'L' => facing = rotate_vector(&facing, instr.value),
            'R' => facing = rotate_vector(&facing, 360 - instr.value),
            _ => {
                ship_pose += match instr.action {
                    'F' => facing,
                    'E' => Vec2::new(1, 0),
                    'W' => Vec2::new(-1, 0),
                    'N' => Vec2::new(0, -1),
                    'S' => Vec2::new(0, 1),
                    _ => unreachable!()
                } * instr.value;
            }
        }
    }
    ship_pose
}

fn run_program2(program: &[Instr]) -> Vec2 {
    let mut ship_pose = Vec2::new(0, 0);
    let mut waypoint = Vec2::new(10, -1);

    for instr in program.iter() {
        match instr.action {
            'L' => waypoint = rotate_vector(&waypoint, instr.value),
            'R' => waypoint = rotate_vector(&waypoint, 360 - instr.value),
            'F' => ship_pose += waypoint * instr.value,
            _ => {
                waypoint += match instr.action {
                    'E' => Vec2::new(1, 0),
                    'W' => Vec2::new(-1, 0),
                    'N' => Vec2::new(0, -1),
                    'S' => Vec2::new(0, 1),
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
    println!("Distance by first rules: {}", (result1.x + result1.y).abs());

    let result2 = run_program2(&program);
    println!("Distance by second rules: {}", (result2.x + result2.y).abs());
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
    assert_eq!(result1.x + result1.y, 25);

    let result2 = run_program2(&program);
    assert_eq!(result2.x + result2.y, 286);
}
