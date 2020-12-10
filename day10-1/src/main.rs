use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut numbers = load_numbers()?;

    numbers.sort();

    let mut last: u8 = 0;
    let mut ones: u8 = 0;
    let mut threes: u8 = 1; // = 1 for jump from last jolt to tthe device

    for n in numbers {
        match n - last {
            1 => { ones += 1},
            3 => { threes += 1 },
            _ => panic!("Unexpected diff ({})", n - last)
        }

        last = n;
    }

    println!("{} * {} = {}", ones, threes, ones as u16 * threes as u16);

    Ok(())
}

fn load_numbers() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input10.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut numbers = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let number = line.parse::<u8>().unwrap();

        numbers.push(number);
    }

    Ok(numbers)
}
