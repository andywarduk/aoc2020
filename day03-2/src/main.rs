use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = load_map()?;

    let mut result = 1;

    result *= traverse_map(&map, 1, 1);
    result *= traverse_map(&map, 3, 1);
    result *= traverse_map(&map, 5, 1);
    result *= traverse_map(&map, 7, 1);
    result *= traverse_map(&map, 1, 2);

    println!("Answer: {}", result);

    Ok(())
}

fn traverse_map(map: &Vec<Vec<char>>, xadd: u8, yadd: u8) -> u16 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees: u16 = 0;

    while y < map.len() {
        let real_x = x % map[y].len();

        if map[y][real_x] == '#' {
            trees +=1;
        }

        y += yadd as usize;
        x += xadd as usize;
    }

    println!("Hit {} trees on descent moving x+{}, y+{}", trees, xadd, yadd);

    trees
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
