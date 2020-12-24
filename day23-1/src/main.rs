fn main() -> Result<(), Box<dyn std::error::Error>> {
    //play("389125467", 10);
    
    play("315679824", 100);

    Ok(())
}

fn play(start_str: &str, moves: usize) {
    let size = start_str.len();
    let mut nexts: Vec<usize> = Vec::new();

    for _ in 0..=size {
        nexts.push(0)
    }

    let start_chars: Vec<char> = start_str.chars().collect();

    let mut start = start_chars[0].to_digit(10).unwrap() as usize;

    for i in 1..start_chars.len() {
        let prev = start_chars[i - 1].to_digit(10).unwrap() as usize;
        let next = start_chars[i].to_digit(10).unwrap() as usize;
        nexts[prev] = next;
    }

    let prev = start_chars[start_chars.len() - 1].to_digit(10).unwrap() as usize;
    nexts[prev] = start;

    print!("Start: ");
    dump_seq(&nexts, start, false);

    for m in 0..moves {
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

        print!("move {}: ", m + 1);
        dump_seq(&nexts, start, false);
    }

    dump_seq(&nexts, 1, true);
}

fn dump_seq(nexts: &Vec<usize>, start: usize, skip: bool) {
    let mut next;
    
    if skip {
        next = nexts[start];
    } else {
        next = start;
    }

    loop {
        print!("{}", next);
        next = nexts[next];

        if next == start {
            break
        }
    }
    println!("");
}
