
extern crate ndarray;
use ndarray::{Array2, s, concatenate, Axis, Zip};

extern crate itertools;
use itertools::concat;

use std::fs;


fn array_from_contents(contents: &str) -> Array2<u16> {
    let width = contents.lines().next().unwrap().trim().as_bytes().len();
    let rows = contents.lines()
        .map(|line| line.trim().bytes()
             .map(|byte| match byte {
                  b'.' => 0,
                  b'L' => 1,
                  _ => unreachable!()
             }).collect::<Vec<_>>());
    let data = concat(rows);
    Array2::from_shape_vec((width, data.len() / width), data).unwrap()
}

fn surrounds_array(input: &Array2<u16>) -> Array2<u16> {
    let (xd, yd) = input.dim();

    let shift_down = |input: &Array2<u16>| concatenate!(Axis(0), Array2::zeros((1, yd)), input.slice(s![..-1, ..]));
    let shift_up = |input: &Array2<u16>| concatenate!(Axis(0), input.slice(s![1.., ..]), Array2::zeros((1, yd)));
    let shift_right = |input: &Array2<u16>| concatenate!(Axis(1), Array2::zeros((xd, 1)), input.slice(s![.., ..-1]));
    let shift_left = |input: &Array2<u16>| concatenate!(Axis(1), input.slice(s![.., 1..]), Array2::zeros((xd, 1)));

    let shifted_down = shift_down(input);
    let shifted_up = shift_up(input);
    &shifted_down + &shifted_up + shift_left(input) + shift_right(input) +
        shift_left(&shifted_down) + shift_right(&shifted_down) +
        shift_left(&shifted_up) + shift_right(&shifted_up)
}

fn next_step(seats: &Array2<u16>, occupied: &Array2<u16>) -> Array2<u16> {
    let surrounds = surrounds_array(&occupied);
    let mut new_occupied = occupied.clone();
    Zip::from(&mut new_occupied).and(seats).and(&surrounds)
        .apply(|occupied, &seat, &surrounds| {
            if seat == 1 && *occupied == 1 && surrounds >= 4 {
                *occupied = 0
            } else if seat == 1 && *occupied == 0 && surrounds == 0 {
                *occupied = 1
            }
        });
    new_occupied
}

fn step_until_stable(seats: &Array2<u16>) -> Array2<u16> {
    // Assume all seats begin as full.
    let mut occupied = seats.clone();
    loop {
        let new_occupied = next_step(&seats, &occupied);
        println!("{:?}", new_occupied);
        if occupied == new_occupied {
            return occupied;
        }
        occupied = new_occupied;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let seats = array_from_contents(&contents);
    let occupied = step_until_stable(&seats);
    println!("Occupied seats: {}", occupied.sum());
}


#[test]
fn test_sample() {
    const SAMPLE: &str = "\
    L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";
    let seats = array_from_contents(SAMPLE);
    let occupied = step_until_stable(&seats);
    assert_eq!(occupied.sum(), 37);
}
