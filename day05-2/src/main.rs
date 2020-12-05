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

    let mut got_first: bool = false;
    let mut seat_num: u16 = 0;

    for (elem, seat) in seats.iter().enumerate() {
        if *seat {
            got_first = true;
        } else {
            if got_first {
                seat_num = elem as u16;
                break
            }
        }
    }

    println!("Seat is {}", seat_num);

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

const fn seat_from_position(pos: (u8, u8)) -> u16 {
    ((pos.0 as u16) * 8) + pos.1 as u16
}
