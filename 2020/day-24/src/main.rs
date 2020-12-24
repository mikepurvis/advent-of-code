extern crate euclid;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use euclid::Vector2D;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq)]
enum Dir {
    NE, E, SE, NW, W, SW
}

struct TileSpace;
type TileVector = Vector2D<isize, TileSpace>;

fn parse_line(line: &str) -> Vec<Dir> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(ne|e|se|nw|w|sw)").unwrap();
    }
    RE.find_iter(line).map(|m|
        match m.as_str() {
            "ne" => Dir::NE,
            "e" => Dir::E,
            "se" => Dir::SE,
            "nw" => Dir::NW,
            "w" => Dir::W,
            "sw" => Dir::SW,
            _ => unreachable!()
        }
    ).collect()
}

fn parse_lines(contents: &str) -> Vec<Vec<Dir>> {
    contents.trim().lines().map(|line|
        parse_line(line.trim())
    ).collect()
}

fn tile_from_steps(steps: &[Dir]) -> TileVector {
    steps.iter().map(|dir|
        match dir {
            Dir::NE => TileVector::new(0, 1),
            Dir::E => TileVector::new(1, 0),
            Dir::SE => TileVector::new(1, -1),
            Dir::NW => TileVector::new(-1, 1),
            Dir::W => TileVector::new(-1, 0),
            Dir::SW => TileVector::new(0, -1),
        }
    ).fold(TileVector::new(0, 0), |acc, x| acc + x)
}

fn flip_tile(tileset: &mut HashSet<TileVector>, position: TileVector) {
    if !tileset.remove(&position) {
        tileset.insert(position);
    }
}

fn get_flipped_tiles(step_vecs: &[Vec<Dir>]) -> HashSet<TileVector> {
    let mut tileset = HashSet::new();
    for steps in step_vecs {
        flip_tile(&mut tileset, tile_from_steps(&steps))
    }
    tileset
}

fn main() {
    let flipped_tiles = get_flipped_tiles(&parse_lines(
            &fs::read_to_string("input.txt").unwrap()));
    println!("Flipped tiles: {}", flipped_tiles.len());
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("wenww"),
               vec![Dir::W, Dir::E, Dir::NW, Dir::W]); 
}

#[test]
fn test_sample() {
    const INPUT: &str = r#"
    sesenwnenenewseeswwswswwnenewsewsw
    neeenesenwnwwswnenewnwwsewnenwseswesw
    seswneswswsenwwnwse
    nwnwneseeswswnenewneswwnewseswneseene
    swweswneswnenwsewnwneneseenw
    eesenwseswswnenwswnwnwsewwnwsene
    sewnenenenesenwsewnenwwwse
    wenwwweseeeweswwwnwwe
    wsweesenenewnwwnwsenewsenwwsesesenwne
    neeswseenwwswnwswswnw
    nenwswwsewswnenenewsenwsenwnesesenew
    enewnwewneswsewnwswenweswnenwsenwsw
    sweneswneswneneenwnewenewwneswswnese
    swwesenesewenwneswnwwneseswwne
    enesenwswwswneneswsenwnewswseenwsese
    wnwnesenesenenwwnenwsewesewsesesew
    nenewswnwewswnenesenwnesewesw
    eneswnwswnwsenenwnwnwwseeswneewsenese
    neswnwewnwnwseenwseesewsenwsweewe
    wseweeenwnesenwwwswnew
    "#;
    assert_eq!(get_flipped_tiles(&parse_lines(INPUT)).len(), 10);
}
