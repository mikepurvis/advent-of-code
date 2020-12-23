extern crate itertools;

use itertools::zip;
use std::collections::VecDeque;
use std::fs;
use std::vec::Vec;

fn parse_decks(input: &str) -> Vec<VecDeque<u8>> {
    let mut lines = input.trim().lines();
    let mut decks = Vec::new();
    'next_deck: loop {
        lines.next().unwrap();
        let mut deck = VecDeque::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                decks.push(deck);
                continue 'next_deck;
            }
            deck.push_back(line.trim().parse().unwrap())
        }
        decks.push(deck);
        break;
    }
    return decks;
}

// Returns Some(winner) if the game is over.
fn take_turn<'a>(decks: &'a mut Vec<VecDeque<u8>>) -> Option<&'a VecDeque<u8>> {
    let mut drawn_cards = decks.iter_mut().map(|d|
        d.pop_front().unwrap()
    ).collect::<Vec<_>>();
    let (winner, loser) = if drawn_cards[0] > drawn_cards[1] {
        (0, 1)
    } else {
        drawn_cards.reverse();
        (1, 0)
    };
    decks[winner].extend(drawn_cards.iter());
    if decks[loser].is_empty() {
        Some(&decks[winner])
    } else {
        None
    }
}

fn compute_score(deck: &VecDeque<u8>) -> u32 {
    zip(deck.iter(), (1..=deck.len()).rev()).map(|t|
        *t.0 as u32 * t.1 as u32
    ).sum()
}

fn play_game(mut decks: &mut Vec<VecDeque<u8>>) -> u32 {
    loop {
        if let Some(winning_deck) = take_turn(&mut decks) {
            return compute_score(&winning_deck);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut decks = parse_decks(&input);
    println!("Winning score: {}", play_game(&mut decks));
}

#[test]
fn test_sample() {
    const INPUT: &str = r#"
    Player 1:
    9
    2
    6
    3
    1

    Player 2:
    5
    8
    4
    7
    10"#;

    let mut decks = parse_decks(INPUT);
    assert_eq!(play_game(&mut decks), 306);
}
