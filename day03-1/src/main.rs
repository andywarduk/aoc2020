use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = load_map()?;

    let mut x = 0;
    let mut trees = 0;

    for y in 0..map.len() {
        let real_x = x % map[y].len();

        if map[y][real_x] == '#' {
            trees +=1;
        }

        x += 3;
    }

    println!("Hit {} trees on descent", trees);

    Ok(())
}

fn load_map() -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input03.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut yaxis = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        // Split line in to characters
        let xrow = line.chars().collect();

        // Add to collection
        yaxis.push(xrow)
    }

    Ok(yaxis)
}
