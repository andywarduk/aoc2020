use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut numbers = load_numbers()?;

    numbers.sort();

    let mut answer: u64 = 1;
    let mut last: u8 = 0;
    let mut start: usize = 0;

    for (elem, n) in numbers.iter().enumerate() {
        match *n - last {
            1 => {},
            3 => { 
                let mut range = elem - start;

                if start == 0 {
                    range += 1
                }

                process_range(&mut answer, range);

                start = elem;
            },
            _ => panic!("Unexpected diff ({})", n - last)
        }

        last = *n;
    }

    process_range(&mut answer, numbers.len() - start);

    println!("{} combinations", answer);

    Ok(())
}

fn process_range(answer: &mut u64, range: usize) {
    println!("Range of {}", range);

    match range {
        1 | 2 => {},      // 1
        3 => *answer *= 2, // 1->2->3, 1->3
        4 => *answer *= 4, // 1->2->3->4, 1->2->4, 1->3->4, 1->4
        5 => *answer *= 7, // 1->2->3->4->5, 1->2->3->5, 1->2->4->5, 1->2->5, 1->3->4->5, 1->3->5, 1->4->5
        _ => panic!("Unhandled range {}", range)
    }
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
