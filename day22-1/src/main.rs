use std::fs;
use std::io::{self, BufRead};
use std::collections::VecDeque;

type Hand = VecDeque<u16>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut hands = load_hands()?;

    println!("Starting cards: {:?} ({}, {})", hands, hands[0].len(), hands[1].len());

    play(&mut hands);

    println!("Ending cards: {:?} ({}, {})", hands, hands[0].len(), hands[1].len());

    println!("Score: {}", score_hands(&hands));

    Ok(())
}

fn load_hands() -> Result<[Hand; 2], Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input22.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut hands: [Hand; 2] = [Hand::new(), Hand::new()];
    let mut cur_player: Option<usize> = None;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        match cur_player {
            None => {
                if line.starts_with("Player ") {
                    let player_str = *&line[7..].split(":").next().unwrap();
                    let player_no = player_str.parse::<usize>()?;
                    cur_player = Some(player_no - 1);
                }
            }
            Some(player) => {
                if line.is_empty() {
                    cur_player = None
                } else {
                    hands[player].push_back(line.parse().unwrap())
                }
            }
        }
    }

    Ok(hands)
}

fn play(hands: &mut [Hand]) {
    loop {
        if let Some(c1) = hands[0].pop_front() {
            if let Some(c2) = hands[1].pop_front() {
                if c1 > c2 {
                    // Player 1 win
                    hands[0].push_back(c1);
                    hands[0].push_back(c2);
                } else {
                    // Player 1 win
                    hands[1].push_back(c2);
                    hands[1].push_back(c1);
                }

                println!("Hands: {:?} ({}, {})", hands, hands[0].len(), hands[1].len());
            } else {
                hands[0].push_front(c1);
                break
            }
        } else {
            break
        }
    }
}

fn score_hands(hands: &[Hand]) -> u64 {
    let mut score: u64 = 0;

    for h in hands {
        let hand_len = h.len();

        for i in 0..hand_len {
            score += h[i] as u64 * (hand_len - i) as u64;
        }
    }

    score
}
