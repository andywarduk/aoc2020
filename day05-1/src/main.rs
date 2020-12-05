use std::fs;
use std::io::{self, BufRead};

const ROWS: u8 = 128;
const COLS: u8 = 8;
const SEATS: u16 = seat_from_position((ROWS - 1, COLS - 1));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input05.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut seats = [false; SEATS as usize];

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let pos = position_from_chop(&line[..]);
        let seat = seat_from_position(pos);

        seats[seat as usize] = true;
    }

    let mut highest: u16 = 0;

    for (elem, seat) in seats.iter().enumerate() {
        if *seat {
            highest = elem as u16;
        }
    }

    println!("Highest seat is {}", highest);

    Ok(())
}

fn position_from_chop(chop: &str) -> (u8, u8) {
    let mut ymin: u8 = 0;
    let mut ymax: u8 = ROWS - 1;
    let mut xmin: u8 = 0;
    let mut xmax: u8 = COLS - 1;

    for dir in chop.chars() {
        match dir {
            'F' => { let cnt = ((ymax - ymin) + 1) / 2; ymax -= cnt; },
            'B' => { let cnt = ((ymax - ymin) + 1) / 2; ymin += cnt; },
            'L' => { let cnt = ((xmax - xmin) + 1) / 2; xmax -= cnt; },
            'R' => { let cnt = ((xmax - xmin) + 1) / 2; xmin += cnt; },
            _ => panic!("Direction '{}' invalid", dir)
        }
    }

    return (ymin, xmin)
}

#[test]
fn position_from_chop_test() {
    assert_eq!((44, 5), position_from_chop("FBFBBFFRLR"));
    assert_eq!((70, 7), position_from_chop("BFFFBBFRRR"));
    assert_eq!((14, 7), position_from_chop("FFFBBBFRRR"));
    assert_eq!((102, 4), position_from_chop("BBFFBBFRLL"));
}

const fn seat_from_position(pos: (u8, u8)) -> u16 {
    ((pos.0 as u16) * 8) + pos.1 as u16
}

#[test]
fn seat_from_position_test() {
    assert_eq!(357, seat_from_position((44, 5)));
    assert_eq!(567, seat_from_position((70, 7)));
    assert_eq!(119, seat_from_position((14, 7)));
    assert_eq!(820, seat_from_position((102, 4)));
}
