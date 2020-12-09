use std::fs;
use std::io::{self, BufRead};
use std::cmp;

const PREAMBLE: usize = 25;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = load_numbers()?;

    let target = find_invalid_number(&numbers);

    let range = find_range(target, &numbers);

    calc_result(&numbers, range);

    Ok(())
}

fn find_invalid_number(numbers: &Vec<u64>) -> u64 {
    let mut result: u64 = 0;

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
            result = numbers[i];
            break
        }
    }

    result
}

fn find_range(target: u64, numbers: &Vec<u64>) -> (usize, usize) {
    let mut result: (usize, usize) = (0,0);
    let mut finished = false;

    for i in 0..numbers.len() {
        let mut total: u64 = numbers[i];

        for j in (i + 1)..numbers.len() {
            total += numbers[j];

            if total >= target {
                if total == target {
                    result = (i, j);
                    finished = true
                }
                break
            }
        }

        if finished {
            break
        }
    }

    result
}

fn calc_result(numbers: &Vec<u64>, range: (usize, usize)) {
    let mut min: u64 = u64::MAX;
    let mut max: u64 = 0;

    for i in range.0..range.1 {
        min = cmp::min(min, numbers[i]);
        max = cmp::max(max, numbers[i]);
    }

    println!("min+max = {}", min + max)
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
