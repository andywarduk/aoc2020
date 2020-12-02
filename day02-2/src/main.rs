use std::fs;
use std::io::{self, BufRead};

struct PassEnt {
    p1: usize,
    p2: usize,
    pchar: char,
    password: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passents = load_passwd()?;

    println!("Valid entry count: {}", valid_count(passents));

    Ok(())
}

fn load_passwd() -> Result<Vec<PassEnt>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input02.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut vec = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        // Get string elements
        let elems: Vec<_> = line.split_whitespace().collect();

        match elems.len() {
            3 => (),
            _ => {
                Err(format!("Invalid number of entry elements - {}, expecting 3", elems.len()))?
            }
        }

        let pos: Vec<_> = elems[0].split("-").collect();

        // Get position elements
        match pos.len() {
            2 => (),
            _ => {
                Err(format!("Invalid number of position elements - {}, expecting 2", pos.len()))?
            }
        }

        // Build entry
        let ent = PassEnt {
            p1: pos[0].parse()?,
            p2: pos[1].parse()?,
            pchar: elems[1].chars().next().unwrap(),
            password: elems[2].to_string()
        };

        // Add to collection
        vec.push(ent)
    }

    Ok(vec)
}

fn valid_count(passents: Vec<PassEnt>) -> u16 {
    let mut valid: u16 = 0;

    for ent in passents {
        let password: Vec<_> = ent.password.chars().collect();

        if password[ent.p1 - 1] == ent.pchar {
            if password[ent.p2 - 1] != ent.pchar {
                valid +=1;
            }
        }
        else if password[ent.p2 - 1] == ent.pchar {
            valid +=1;
        }
    }

    valid
}
