use std::fs;
use std::io::{self, BufRead};

enum Action {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
    Left(u16),
    Right(u16),
    Forward(u16)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let actions = load_actions()?;

    let xy = take_actions(actions);

    println!("Manhattan distance: {}", i16::abs(xy.0) + i16::abs(xy.1));

    Ok(())
}

fn load_actions() -> Result<Vec<Action>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input12.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut actions = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let action_char = line.chars().next().unwrap();
        let value = &line[1..].parse::<u16>().unwrap();

        let action: Action = match action_char {
            'N' => Action::North(*value),
            'S' => Action::South(*value),
            'E' => Action::East(*value),
            'W' => Action::West(*value),
            'L' => Action::Left(*value),
            'R' => Action::Right(*value),
            'F' => Action::Forward(*value),
            _ => Err(format!("Action {} not recognised", action_char))?
        };

        actions.push(action);
    }

    Ok(actions)
}

fn take_actions(actions: Vec<Action>) -> (i16, i16) {
    let mut bearing: i16 = 90;

    let mut xy: (i16, i16) = (0, 0);

    for action in actions {
        match action {
            Action::North(dist) => xy.1 += dist as i16,
            Action::South(dist) => xy.1 -= dist as i16,
            Action::East(dist) => xy.0 += dist as i16,
            Action::West(dist) => xy.0 -= dist as i16,
            Action::Left(deg) => {
                bearing = bearing - deg as i16;
                while bearing < 0 {
                    bearing += 360
                }
            }
            Action::Right(deg) => bearing = (bearing + deg as i16) % 360,
            Action::Forward(dist) => {
                match bearing {
                    0 => xy.1 += dist as i16,
                    90 => xy.0 += dist as i16,
                    180 => xy.1 -= dist as i16,
                    270 => xy.0 -= dist as i16,
                    _ => panic!("Unhandled bearing {}", bearing)
                }
            }
        };
    }

    xy
}
