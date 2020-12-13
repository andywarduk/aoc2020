use std::fs;
use std::io::{self, BufRead};

type IntType = i128;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let times = load_info()?;

    println!("{:?}", times);

    // Algorithm at https://brilliant.org/wiki/chinese-remainder-theorem/
    // Lifted from https://gist.github.com/samueltardieu/ad5d4e49fcb0841cda2e08c1b7f47fad

    // Calculate product of all periods
    let prod = times.iter().map(|&(_, p)| p).product();

    // Calculate sum of product / period multiplied by offset
    let factors = times.iter().map(|&(o, p)| o * prod / p).sum::<IntType>() % prod;

    let factors = prod - factors;

    let sum = times.iter().map(|&(_, p)| prod / p).sum();

    let isum = modinverse(sum, prod).unwrap();

    println!("Answer: {}", (isum * factors) % prod);

    Ok(())
}

fn load_info() -> Result<Vec<(IntType, IntType)>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input13.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let lines = inputbuf.lines();

    let mut times: Vec<_> = Vec::new();
    
    for (elem, id) in lines.skip(1).next().unwrap().unwrap().split(",").enumerate() {
        if id != "x" {
            times.push((elem as IntType, id.parse::<IntType>()?));
        }
    }

    Ok(times)
}

// Lifted from https://docs.rs/modinverse/0.1.0/modinverse/

pub fn egcd(a: IntType, b: IntType) -> (IntType, IntType, IntType) {
    assert!(a < b);

    if a == 0 {
        return (b, 0, 1);
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        return (g, y - (b / a) * x, x);
    }
}

pub fn modinverse(a: IntType, m: IntType) -> Option<IntType> {
    let (g, x, _) = egcd(a, m);

    if g != 1 {
        return None;
    }
    else {
        return Some(x % m);
    }
}
