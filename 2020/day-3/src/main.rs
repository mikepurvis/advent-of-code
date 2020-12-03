use std::fs;
use std::ops;
use std::vec::Vec;


fn map_from_file(filename: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let mut map: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(filename).unwrap();
    for line in contents.split_whitespace() {
        map.push(line.chars().collect::<Vec<char>>())
    }
    Ok(map)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

impl ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() -> std::io::Result<()> {
    let slopes = vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];
    let map = map_from_file("input.txt").unwrap();
    let mut result: u64 = 1;

    for slope in slopes.iter()
    {
        let mut pos = Point { x: 0, y: 0 };
        let mut collisions = 0;
        while pos.y < map.len() {
            if map[pos.y][pos.x] != '.' {
                collisions += 1;
            }
            pos += &slope;
            if pos.x >= map[0].len() {
                pos.x -= map[0].len()
            }
        }
        println!("Collisions for {:?}: {}", slope, collisions);
        result *= collisions;
    }
    println!("Result: {}", result);
    Ok(())
}
