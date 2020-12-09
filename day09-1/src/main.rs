use std::fs;
use std::io::{self, BufRead};

const PREAMBLE: usize = 25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = load_numbers()?;

    for i in PREAMBLE..(numbers.len() - 1) {
        let target = numbers[i];
        let mut found = false;

        for j in (((i + 1) - PREAMBLE)..i).rev() {
            for k in (((i + 1) - (PREAMBLE + 1))..j).rev() {
                if numbers[j] + numbers[k] == target {
                    found = true;
                    break
                }
            }
        }

        if !found {
            println!("Element {} ({}) does not have a sum", i + 1, numbers[i]);
            break
        }
    }

    Ok(())
}

fn load_numbers() -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input09.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut numbers = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let number = line.parse::<u64>().unwrap();

        numbers.push(number);
    }

    Ok(numbers)
}
