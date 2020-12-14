use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    exec_program()?;

    Ok(())
}

fn exec_program() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input14.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut mem = [0u64; 65536];
    let mut andmask = u64::MAX;
    let mut ormask = 0u64;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let components: Vec<_> = line.split(" = ").collect();

        if components[0] == "mask" {
            andmask = u64::MAX;
            ormask = 0;

            let mut bit = 1 << 35;

            for c in components[1].chars() {
                match c {
                    '0' => andmask &= u64::MAX - bit,
                    '1' => ormask |= bit,
                    'X' => {},
                    _ => panic!("Invalid bitmask {}", components[1])
                }

                bit >>= 1;
            }
        } else {
            let addr = components[0].split("[").skip(1).next().unwrap().split("]").next().unwrap().parse::<u16>().unwrap();
            let val = components[1].parse::<u64>().unwrap();

            let writeval = (val & andmask) | ormask;
            mem[addr as usize] = writeval;
        }
    }

    let sum: u64 = mem.iter().sum();

    println!("Sum is {}", sum);

    Ok(())
}

