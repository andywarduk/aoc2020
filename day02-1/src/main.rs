use std::fs;
use std::io::{self, BufRead};

struct PassEnt {
    min: u8,
    max: u8,
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

    let mut line;
    for line_result in inputbuf.lines() {
        line = String::new();

        match line_result {
            Err(e) => Err(e)?,
            Ok(l) => line = l.clone()
        }

        let elems: Vec<&str> = line.split_whitespace().collect();

        if elems.len() < 3 {
            Err("Not enough entry elements")?
        }

        let max_min: Vec<&str> = elems[0].split("-").collect();

        if max_min.len() < 2 {
            Err("Not enough max min elements")?
        }

        let ent = PassEnt {
            min: max_min[0].parse::<u8>()?,
            max: max_min[1].parse::<u8>()?,
            pchar: elems[1].chars().next().unwrap(),
            password: elems[2].to_string()
        };

        vec.push(ent)
    }

    Ok(vec)
}

fn valid_count(passents: Vec<PassEnt>) -> u16 {
    let mut valid: u16 = 0;

    for ent in passents {
        let pchars = ent.password.chars().filter(|e| *e == ent.pchar).count();

        if pchars >= ent.min.into() && pchars <= ent.max.into() {
            valid += 1;
        }
    }

    valid
}