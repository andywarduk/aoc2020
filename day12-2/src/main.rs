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

    println!("Manhattan distance: {}", i32::abs(xy.0) + i32::abs(xy.1));

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

fn take_actions(actions: Vec<Action>) -> (i32, i32) {
    let mut waypoint: (i16, i16) = (10, 1);
    let mut xy: (i32, i32) = (0, 0);

    for action in actions {
        match action {
            Action::North(dist) => waypoint.1 += dist as i16,
            Action::South(dist) => waypoint.1 -= dist as i16,
            Action::East(dist) => waypoint.0 += dist as i16,
            Action::West(dist) => waypoint.0 -= dist as i16,
            Action::Left(deg) => {
                let save_waypoint = waypoint;

                match deg {
                    90 => {waypoint.0 = -save_waypoint.1; waypoint.1 = save_waypoint.0},
                    180 => {waypoint.0 = -save_waypoint.0; waypoint.1 = -save_waypoint.1},
                    270 => {waypoint.0 = save_waypoint.1; waypoint.1 = -save_waypoint.0},
                    _ => panic!("Unhandled Left {}", deg)
                }
            }
            Action::Right(deg) => {
                let save_waypoint = waypoint;

                match deg {
                    270 => {waypoint.0 = -save_waypoint.1; waypoint.1 = save_waypoint.0},
                    180 => {waypoint.0 = -save_waypoint.0; waypoint.1 = -save_waypoint.1},
                    90 => {waypoint.0 = save_waypoint.1; waypoint.1 = -save_waypoint.0},
                    _ => panic!("Unhandled Left {}", deg)
                }
            },
            Action::Forward(dist) => {
                xy.0 += dist as i32 * waypoint.0 as i32;
                xy.1 += dist as i32 * waypoint.1 as i32;
            }
        };
    }

    xy
}
