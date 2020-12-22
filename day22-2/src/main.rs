use std::fs;
use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

fn play(hands: &mut [Hand]) -> u8 {
    let winner: u8;

    let mut seen_hands: HashSet<u64> = HashSet::new();

    loop {
        // Seen these hands before?
        let hash = hash_hands(hands);

        match seen_hands.get(&hash) {
            Some(_) => {
                winner = 1;
                break
            },
            None => {
                seen_hands.insert(hash);
            }
        }

        // Get player 1 card
        if let Some(c1) = hands[0].pop_front() {
            // Get player 2 card
            if let Some(c2) = hands[1].pop_front() {
                let hand_winner;

                // Need to play a recursive game?
                if c1 as usize <= hands[0].len() && c2 as usize <= hands[1].len() {
                    // Yes
                    hand_winner = recursive_play(&hands, c1, c2)
                } else {
                    // No - greatest value card wins
                    if c1 > c2 {
                        hand_winner = 1;
                    } else {
                        hand_winner = 2;
                    }
                }

                match hand_winner {
                    1 => {
                        // Player 1 win
                        hands[0].push_back(c1);
                        hands[0].push_back(c2);
                    }
                    2 => {
                        // Player 2 win
                        hands[1].push_back(c2);
                        hands[1].push_back(c1);
                    }
                    _ => panic!("Invalid winner")
                }

            } else {
                // Player 2 has no cards left
                hands[0].push_front(c1);
                winner = 1;
                break

            }

        } else {
            // Player 1 has no cards left
            winner = 2;
            break

        }
    }

    winner
}

fn recursive_play(hands: &[Hand], c1: u16, c2: u16) -> u8 {
    let mut recurse_hands: [Hand; 2] = [Hand::new(), Hand::new()];

    // Build hands for recursive play
    for i in 0..c1 as usize {
        recurse_hands[0].push_back(hands[0][i]);
    }

    for i in 0..c2 as usize {
        recurse_hands[1].push_back(hands[1][i]);
    }

    play(&mut recurse_hands)
}

fn hash_hands(hands: &[Hand]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hands.hash(&mut hasher);
    hasher.finish()
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
