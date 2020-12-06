
use std::fs;
use std::vec::Vec;


#[derive(Debug, PartialEq)]
struct Seat {
    row: u8,
    column: u8
}

impl Seat {
    fn from_line(line: &str) -> Result<Self, std::io::Error> {
        let bytes: Vec<u8> = line.bytes().collect();
        let mut row = 0;
        let mut column = 0;
        for (i, b) in bytes[..7].iter().rev().enumerate() {
            if *b == b'B' {
                row += 1 << i;
            }
        }
        for (i, b) in bytes[7..10].iter().rev().enumerate() {
            if *b == b'R' {
                column += 1 << i;
            }
        }
        Ok(Self { row: row, column: column })
    }
    fn get_id(&self) -> u16 {
        (self.row as u16 * 8) + self.column as u16
    }
}

fn seats_from_file(filename: &str) -> Result<Vec<Seat>, std::io::Error> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut seats = Vec::new();

    for line in contents.lines() {
        seats.push(Seat::from_line(&line).unwrap());
    }
    Ok(seats)
}

fn main() {
    let seats = seats_from_file("input.txt").unwrap();
    let mut ids: Vec<u16> = seats.iter().map(|s| s.get_id()).collect();
    println!("Highest ID: {}", &ids.iter().max().unwrap());

    ids.sort();
    for s in ids.windows(2) {
        if s[1] - s[0] != 1 {
            println!("Our seat: {}", s[0] + 1);
            break;
        }
    }
}

#[test]
fn test_samples() {
	let cases = vec![
		("FBFBBFFRLR", 44, 5, 357),
		("BFFFBBFRRR", 70, 7, 567),
		("FFFBBBFRRR", 14, 7, 119),
		("BBFFBBFRLL", 102, 4, 820)
	];

    for (s, r, c, id) in cases.into_iter() {
        let seat = Seat::from_line(&s).unwrap();
        assert_eq!(seat.row, r);
        assert_eq!(seat.column, c);
        assert_eq!(seat.get_id(), id);
    }
}
