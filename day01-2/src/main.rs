use std::fs;
use std::io::{self, BufRead};

fn main() {
    let numbers = load_numbers();

    let mut finished = false;
    for a in 0..numbers.len() {
        for b in (a + 1)..numbers.len() {
            for c in (b + 1)..numbers.len() {
                if numbers[a] + numbers[b] + numbers[c] == 2020 {
                    println!("{} on line {} + {} on line {} + {} on line {} = 2020, product is {}",
                        numbers[a], a, numbers[b], b, numbers[c], c, numbers[a] * numbers[b] * numbers[c]
                        );
                    finished = true;
                    break;
                }
            }

            if finished {
                break
            }
        }

        if finished {
            break
        }
    }
}

fn load_numbers() -> Vec<u32> {
    // Open the file read only
    let input = fs::File::open("../input01.txt").expect("Error opening file");

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    inputbuf.lines() // Create line iterator
        .filter(|s| !s.as_ref().unwrap().is_empty()) // Remove empty lines
        .map(|s| s.unwrap().parse::<u32>().unwrap()) // Parse to u32
        .collect() // Collect results
}
