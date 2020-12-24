fn main() -> Result<(), Box<dyn std::error::Error>> {
    play("315679824", 1_000_000, 10_000_000);

    Ok(())
}

fn play(start_str: &str, size: usize, moves: usize) {
    let mut nexts: Vec<usize> = Vec::new();

    for _ in 0..=size + 1 {
        nexts.push(0)
    }

    let start_chars: Vec<char> = start_str.chars().collect();

    let mut start = start_chars[0].to_digit(10).unwrap() as usize;

    for i in 1..start_chars.len() {
        let prev = start_chars[i - 1].to_digit(10).unwrap() as usize;
        let next = start_chars[i].to_digit(10).unwrap() as usize;
        nexts[prev] = next;
    }

    let mut prev = start_chars[start_chars.len() - 1].to_digit(10).unwrap() as usize;
    for i in start_chars.len() + 1..=size {
        nexts[prev] = i;
        prev = i;
    }

    nexts[prev] = start;

    for _ in 0..moves {
        let int1 = nexts[start];
        let int2 = nexts[int1];
        let int3 = nexts[int2];

        // Get next start number
        let next_start = nexts[int3];

        // Chop out the 3 numbers
        nexts[start] = next_start;

        // Work out destination
        let mut dest = start - 1;
        loop {
            if dest == 0 {
                dest = size;
            }
            if dest != int1 && dest != int2 && dest != int3 {
                break
            }
            dest -= 1;
        }

        // Insert chopped numbers at destintion
        nexts[int3] = nexts[dest];
        nexts[dest] = int1;

        // Move to next start
        start = next_start;
    }

    println!("{} * {} = {}", nexts[1], nexts[nexts[1]], nexts[1] * nexts[nexts[1]]);
}
