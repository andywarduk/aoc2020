use std::fs;
use std::io::{self, BufRead};

const SUM: u32 = 2020;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = load_numbers()?;

    for a in 0..numbers.len() {
        for b in (a + 1)..numbers.len() {
            for c in (b + 1)..numbers.len() {
                if numbers[a] + numbers[b] + numbers[c] == SUM {
                    println!("{} on line {} + {} on line {} + {} on line {} = {}, product is {}",
                        numbers[a], a + 1,
                        numbers[b], b + 1,
                        numbers[c], c + 1,
                        SUM,
                        numbers[a] * numbers[b] * numbers[c]);
                    return Ok(());
                }
            }
        }
    }

    Err(format!("Sum to {} not found", SUM).into())
}

fn load_numbers() -> Result<Vec<u32>, std::io::Error> {
    // Open the file read only
    let input = fs::File::open("../input01.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let vec = inputbuf.lines() // Create line iterator
        .filter_map(|s| s.unwrap().parse().ok()) // Try and convert to u32. Filter out entries that fail
        .collect(); // Collect results

    Ok(vec)
}
