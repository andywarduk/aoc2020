use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (depart_ts, times) = load_info()?;

    let wait_info: Vec<_> = times.iter().map(|id| {
        (id, id - (depart_ts % id))
    }).collect();

    println!("{:?}", wait_info);

    let min = wait_info.iter().fold((0, u32::MAX), |minacc, elem| {
        if elem.1 < minacc.1 {
            return (*elem.0, elem.1);
        }
        minacc
    });

    println!("Bus {} is next, wait time {}. Answer {}", min.0, min.1, min.0 * min.1);

    Ok(())
}

fn load_info() -> Result<(u32, Vec<u32>), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input13.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut lines = inputbuf.lines();

    let depart_line = lines.next().unwrap().unwrap();
    let depart_ts = depart_line.parse::<u32>()?;

    let mut times: Vec<u32> = Vec::new();
    
    for id in lines.next().unwrap().unwrap().split(",") {
        if id != "x" {
            times.push(id.parse::<u32>()?);
        }
    }

    Ok((depart_ts, times))
}

