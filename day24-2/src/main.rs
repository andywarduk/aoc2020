use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut black_set = load_flips()?;

    for i in 1..=100 {
        black_set = flip_tiles(black_set);

        println!("Day {}, {} black tiles", i, black_set.len());
    }

    println!("{} black tiles", black_set.len());

    Ok(())
}

// Hexagon coordinate systems at https://www.redblobgames.com/grids/hexagons/

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone)]
struct HexCoord {
    q: isize, // Column (odd-r layout)
    r: isize  // Row
}

type BlackSet = HashSet<HexCoord>;

fn load_flips() -> Result<BlackSet, Box<dyn std::error::Error>> {
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

                        coord.r -= 1;
                    }
                    's' => {
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

                        coord.r += 1;
                    }
                    _ => panic!("Invalid direction {}", c1)
                }
            }
            None => {
                break
            }
        }
    }

    coord
}

fn flip_tiles(black_set: BlackSet) -> BlackSet {
    let mut new_set = HashSet::new();

    let (min, max) = min_max(&black_set);

    let mut c: HexCoord = Default::default();

    for r in min.r..=max.r {
        c.r = r;

        for q in min.q..=max.q {
            c.q = q;

            let neig = count_neighbours(&black_set, &c);

            if black_set.get(&c).is_some() {
                // Currently black
                match neig {
                    1 | 2 => {
                        // Stays black
                        new_set.insert(c.clone());
                    },
                    _ => {}
                };
            } else {
                // Currently white
                match neig {
                    2 => {
                        // Flips to black
                        new_set.insert(c.clone());
                    },
                    _ => {}
                };
            }
        }
    }

    new_set
}

fn min_max(black_set: &BlackSet) -> (HexCoord, HexCoord) {
    let mut min = HexCoord {
        q: isize::MAX,
        r: isize::MAX
    };

    let mut max = HexCoord {
        q: isize::MIN,
        r: isize::MIN
    };

    for c in black_set {
        if c.r < min.r {
            min.r = c.r
        }
        if c.r > max.r {
            max.r = c.r
        }
        if c.q < min.q {
            min.q = c.q
        }
        if c.q > max.q {
            max.q = c.q
        }
    }

    // Adjust so that all possibe white tiles are included
    min.r -= 1;
    min.q -= 2;
    max.q += 2;
    max.r += 1;

    (min, max)
}

fn count_neighbours(black_set: &BlackSet, coord: &HexCoord) -> u8 {
    let mut neig: u8 = 0;

    let mut inspect = |r, q| {
        let ins_coord = HexCoord {
            q,
            r
        };

        if black_set.get(&ins_coord).is_some() {
            neig += 1;
        }
    };

    inspect(coord.r, coord.q - 1);
    inspect(coord.r, coord.q + 1);

    inspect(coord.r - 1, coord.q);
    inspect(coord.r + 1, coord.q);

    if coord.r & 1 == 1 {
        // Odd row
        inspect(coord.r - 1, coord.q + 1);
        inspect(coord.r + 1, coord.q + 1);
    } else {
        // Even row
        inspect(coord.r - 1, coord.q - 1);
        inspect(coord.r + 1, coord.q - 1);
    }

    neig
}

#[test]
fn test_neig() {
    let mut black_set: BlackSet = HashSet::new();

    // Test around r3 q3
    black_set.insert(HexCoord { q: 3, r: 2});
    black_set.insert(HexCoord { q: 4, r: 2});
    black_set.insert(HexCoord { q: 2, r: 3});
    black_set.insert(HexCoord { q: 4, r: 3});
    black_set.insert(HexCoord { q: 3, r: 4});
    black_set.insert(HexCoord { q: 4, r: 4});

    assert!(count_neighbours(&black_set, &HexCoord { q: 3, r: 3}) == 6, "Neigbours should be 6");
    black_set.clear();

    // Test around r4 q4
    black_set.insert(HexCoord { q: 3, r: 3});
    black_set.insert(HexCoord { q: 4, r: 3});
    black_set.insert(HexCoord { q: 3, r: 4});
    black_set.insert(HexCoord { q: 5, r: 4});
    black_set.insert(HexCoord { q: 3, r: 5});
    black_set.insert(HexCoord { q: 4, r: 5});

    assert!(count_neighbours(&black_set, &HexCoord { q: 4, r: 4}) == 6, "Neigbours should be 6");
}
