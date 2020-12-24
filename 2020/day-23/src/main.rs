use std::vec::Vec;

fn parse_cups(input: &str) -> Vec<u32> {
    input.chars().map(|c|
        c.to_string().parse().unwrap()
    ).collect()
}

fn cups_str(cups: &Vec<u32>) -> String {
    let one_index = cups.iter().position(|&cup| cup == 1).unwrap();
    let digit_strs: Vec<_> = cups[(one_index + 1)..].iter().chain(cups[0..one_index].iter()).map(|i|
        i.to_string()
    ).collect();
    digit_strs.concat()
}

fn cups_multiple(cups: &Vec<u32>) -> u64 {
    let one_index = cups.iter().position(|&cup| cup == 1).unwrap();
    cups[one_index + 1] as u64 * cups[one_index + 2] as u64
}

// Assume first in list is current cup.
fn do_move(cups: &mut Vec<u32>) {
    let head: Vec<_> = cups.drain(..4).collect();
    let current = head[0];

    let mut dest = current - 1;
    loop {
        if dest == 0 {
            dest = cups.len() as u32 + 4;
        }
        if head[1..].iter().any(|&n| n == dest) {
            dest -= 1;
            continue;
        }
        break;
    }

    let dest_index = (cups.iter().position(|&cup| cup == dest).unwrap() + 1) as usize;
    cups.splice(dest_index..dest_index, head[1..].iter().copied());
    cups.push(current);
}

fn main() {
    let mut cups = parse_cups("925176834");
    for _i in 0..100 { do_move(&mut cups) }
    println!("Cups: {}", cups_str(&cups));
}

#[test]
fn test_rules1() {
    let mut cups = parse_cups("389125467");
    for _i in 0..10 { do_move(&mut cups) }
    assert_eq!(cups_str(&cups), "92658374");
    for _i in 10..100 { do_move(&mut cups) }
    assert_eq!(cups_str(&cups), "67384529");
}

#[test]
fn test_rules2() {
    let mut cups = parse_cups("389125467");
    cups.extend(10..=1_000_000);
    for _i in 0..100 { do_move(&mut cups) }
    assert_eq!(cups_multiple(&cups), 149245887792);
}
