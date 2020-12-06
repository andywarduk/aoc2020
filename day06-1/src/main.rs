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
    let mut bitmask: u32 = 0;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        if line.is_empty() {
            answer += bitmask.count_ones();
            bitmask = 0;

        } else {
            process_line(line, &mut bitmask);

        }
    }

    answer += bitmask.count_ones();

    Ok(answer)
}

fn process_line(line: String, bitmask: &mut u32) {
    for c in line.chars() {
        let charcode = c as u32;
        assert!(charcode >= 'a' as u32);
        assert!(charcode <= 'z' as u32);
        let charno = charcode - ('a' as u32);
        let bit = 1 << charno;
        *bitmask |= bit;
    }
}

#[test]
fn process_line_test() {
    let mut bitmask: u32;
    
    bitmask = 0;
    process_line("a".to_string(), &mut bitmask);
    assert!(bitmask == 1, "bitmask for \"a\" incorrect ({})", bitmask);

    bitmask = 0;
    process_line("b".to_string(), &mut bitmask);
    assert!(bitmask == 2, "bitmask for \"b\" incorrect ({})", bitmask);

    bitmask = 0;
    process_line("ab".to_string(), &mut bitmask);
    assert!(bitmask == 3, "bitmask for \"ab\" incorrect ({})", bitmask);
}
