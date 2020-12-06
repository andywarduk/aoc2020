use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = load_answers()?;

    println!("Total: {}", count);

    Ok(())
}

fn load_answers() -> Result<u32, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input06.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut answer: u32 = 0;
    let mut group_answer: u32 = u32::MAX;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        if line.is_empty() {
            answer += group_answer.count_ones();
            group_answer = u32::MAX;

        } else {
            let bitmask = process_line(line);
            group_answer &= bitmask;

        }
    }

    answer += group_answer.count_ones();

    Ok(answer)
}

fn process_line(line: String) -> u32 {
    let mut bitmask: u32 = 0;

    for c in line.chars() {
        let charcode = c as u32;
        assert!(charcode >= 'a' as u32);
        assert!(charcode <= 'z' as u32);
        let charno = charcode - ('a' as u32);
        let bit = 1 << charno;
        bitmask |= bit;
    }

    bitmask
}
