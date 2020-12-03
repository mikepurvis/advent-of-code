use std::fs;
use std::vec::Vec;


fn map_from_file(filename: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let mut map: Vec<Vec<char>> = Vec::new();
    let contents = fs::read_to_string(filename).unwrap();
    for line in contents.split_whitespace() {
        map.push(line.chars().collect::<Vec<char>>())
    }
    Ok(map)
}


fn main() -> std::io::Result<()> {
    let map = map_from_file("input.txt").unwrap();
    let mut colnum = 0;
    let mut collisions = 0;
    for row in map.iter() {
        if row[colnum] != '.' {
            collisions += 1;
        }
        colnum += 3;
        if colnum >= row.len() {
            colnum -= row.len()
        }
    }
    println!("Collisions: {}", collisions);
    Ok(())
}
