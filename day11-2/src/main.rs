use std::fs;
use std::io::{self, BufRead};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq)]
enum LayoutState {
    Floor,
    Empty,
    Occupied
}

type LayoutRow = Vec<LayoutState>;
type Layout = Vec<LayoutRow>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut layout = load_layout()?;

    let mut iteration = 0;
    let mut last_hash = hash_layout(&layout);

    loop {
        layout = mutate_layout(layout);
        iteration += 1;

        let hash = hash_layout(&layout);

        if hash == last_hash {
            break
        }

        last_hash = hash;
    }

    let occupied = layout.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|c| {
            match c {
                LayoutState::Occupied => true,
                _ => false
            }
        }).count()
    });

    println!("Stable after {} iterations, {} seats occupied", iteration, occupied);

    Ok(())
}

fn load_layout() -> Result<Layout, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input11.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut rows = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let cols: LayoutRow = line.chars().map(|c| {
            match c {
                '.' => LayoutState::Floor,
                'L' => LayoutState::Empty,
                _ => panic!("Unexpected state character '{}'", c)
            }
        }).collect();

        rows.push(cols);
    }

    Ok(rows)
}

fn hash_layout(layout: &Layout) -> u64 {
    let mut hash = DefaultHasher::new();

    layout.hash(&mut hash);

    hash.finish()
}

fn mutate_layout(layout: Layout) -> Layout{
    let rows = layout.len();
    let cols = layout[0].len();

    let mut new_layout: Layout = Vec::with_capacity(rows);

    let scan_seat = |y: usize, x: usize, yadd: isize, xadd: isize| {
        let mut result: Option<(usize, usize)> = None;

        let mut iy = y as isize;
        let mut ix = x as isize;

        loop {
            iy += yadd;
            ix += xadd;

            if ix < 0 || ix >= cols as isize || iy < 0 || iy >= rows as isize {
                break
            }

            if layout[iy as usize][ix as usize] != LayoutState::Floor {
                result = Some((iy as usize, ix as usize));
                break
            }
        }

        result
    };

    let adjacent = |y, x| -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        result.push(scan_seat(y, x, -1, -1));
        result.push(scan_seat(y, x, -1, 0));
        result.push(scan_seat(y, x, -1, 1));
        result.push(scan_seat(y, x, 0, -1));
        result.push(scan_seat(y, x, 0, 1));
        result.push(scan_seat(y, x, 1, -1));
        result.push(scan_seat(y, x, 1, 0));
        result.push(scan_seat(y, x, 1, 1));

        result.iter().filter_map(|s| *s).collect()
    };

    for row_no in 0..rows {
        let row = &layout[row_no];

        let mut new_row: LayoutRow = Vec::with_capacity(cols);

        for col_no in 0..cols {
            let new_state = match row[col_no] {
                LayoutState::Floor => LayoutState::Floor,
                LayoutState::Empty => {
                    match adjacent(row_no, col_no).iter().find(|s| {
                        match layout[s.0][s.1] {
                            LayoutState::Occupied => true,
                            _ => false
                        }
                    }) {
                        Some(_) => LayoutState::Empty,
                        None => LayoutState::Occupied
                    }
                },
                LayoutState::Occupied => {
                    let occupied = adjacent(row_no, col_no).iter().filter(|s| {
                        match layout[s.0][s.1] {
                            LayoutState::Occupied => true,
                            _ => false
                        }
                    }).count();

                    if occupied >= 5 {
                        LayoutState::Empty
                    } else {
                        LayoutState::Occupied
                    }
                }
            };

            new_row.push(new_state);
        }

        new_layout.push(new_row);
    }

    new_layout
}
