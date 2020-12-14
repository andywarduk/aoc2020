use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    exec_program()?;

    Ok(())
}

fn exec_program() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input14.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut andmask = u64::MAX;
    let mut ormask = 0;
    let mut floatbits = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let components: Vec<_> = line.split(" = ").collect();

        if components[0] == "mask" {
            andmask = u64::MAX;
            ormask = 0;
            floatbits = Vec::new();

            let mut bit = 1 << 35;

            for c in components[1].chars() {
                match c {
                    '0' => {},
                    '1' => ormask |= bit,
                    'X' => {
                        andmask &= u64::MAX - bit;
                        floatbits.push(bit);
                    }
                    _ => panic!("Invalid bitmask {}", components[1])
                }

                bit >>= 1;
            }
        } else {
            let base_addr = components[0].split("[").skip(1).next().unwrap().split("]").next().unwrap().parse::<u64>().unwrap();
            let val = components[1].parse::<u64>().unwrap();

            let mut addresses = Vec::new();
            addresses.push((base_addr & andmask) | ormask);
            gen_addresses(&floatbits, &mut addresses);

            for addr in addresses {
                mem.insert(addr, val);
            }
        }
    }

    let sum: u64 = mem.iter().map(|(_, e)| *e).sum();

    println!("Sum is {}", sum);

    Ok(())
}

fn gen_addresses(floatbits: &Vec<u64>, addresses: &mut Vec<u64>) {
    for f in floatbits {
        for i in 0..addresses.len() {
            addresses.push(addresses[i] | f)
        }
    }
}
