use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let black_set = load_flips()?;

    println!("{} black tiles", black_set.len());

    Ok(())
}

#[derive(Default, Hash, Eq, PartialEq, Debug)]
struct HexCoord {
    q: isize, // Column (odd-r layout)
    r: isize  // Row
}

fn load_flips() -> Result<HashSet<HexCoord>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input24.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut black_set = HashSet::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let coord = coord_from_string(&line);

        if black_set.get(&coord).is_some() {
            black_set.remove(&coord);
        } else {
            black_set.insert(coord);
        }
    }

    Ok(black_set)
}

fn coord_from_string(line: &str) -> HexCoord {
    let mut coord: HexCoord = Default::default();

    let mut line_chars = line.chars();

    loop {
        match line_chars.next() {
            Some(c1) => {
                match c1 {
                    'e' => coord.q += 1,
                    'w' => coord.q -= 1,
                    'n' => {
                        coord.r -= 1;

                        match line_chars.next() {
                            Some(c2) => {
                                match c2 {
                                    'e' => if coord.r & 1 == 1 { coord.q += 1 },
                                    'w' => if coord.r & 1 == 0 { coord.q -= 1 },
                                    _ => panic!("Invalid direction {}{}", c1, c2)
                                }
                            }
                            None => panic!("Invalid direction {}", c1)
                        }
                    }
                    's' => {
                        coord.r += 1;

                        match line_chars.next() {
                            Some(c2) => {
                                match c2 {
                                    'e' => if coord.r & 1 == 1 { coord.q += 1 },
                                    'w' => if coord.r & 1 == 0 { coord.q -= 1 },
                                    _ => panic!("Invalid direction {}{}", c1, c2)
                                }
                            }
                            None => panic!("Invalid direction {}", c1)
                        }
                    }
                    _ => panic!("Invalid direction {}", c1)
                }
            }
            None => {
                break
            }
        }
    }

    println!("r {} q {} (from {})", coord.r, coord.q, line);

    coord
}