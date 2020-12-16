use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
struct MinMax {
    min: u32,
    max: u32
}

#[derive(Debug)]
struct Ranges {
    field: String,
    minmax: Box<[MinMax]>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ranges, _my_ticket, other_tickets) = load_info()?;

    let mut error: u32 = 0;

    for other in &other_tickets {
        for field in &*other {
            let mut ok: bool = false;

            for range in &ranges {
                for minmax in 0..2 {
                    if *field >= range.minmax[minmax].min && *field <= range.minmax[minmax].max {
                        ok = true;
                        break
                    }
                }

                if ok {
                    break
                }
            }

            if !ok {
                error += field;
            }
        }
    }

    println!("Error count: {}", error);

    Ok(())
}

enum LoadStage {
    Ranges,
    MyTicket,
    OtherTickets
}

fn load_info() -> Result<(Vec<Ranges>, Vec<u32>, Vec<Vec<u32>>), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input16.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut ranges: Vec<Ranges> = Vec::new();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut other_tickets: Vec<Vec<u32>> = Vec::new();

    let mut load_stage: LoadStage = LoadStage::Ranges;
    
    for line_result in inputbuf.lines() {
        let line = line_result?;

        match load_stage {
            LoadStage::Ranges => {
                if line.is_empty() {
                    load_stage = LoadStage::MyTicket
                } else {
                    let mut field_split = line.split(":");
                    let field = field_split.next().unwrap();

                    let mut range = Ranges {
                        field: field.to_string(),
                        minmax: Box::new([
                            MinMax {
                                min: 0,
                                max: 0
                            },
                            MinMax {
                                min: 0,
                                max: 0
                            },
                        ])
                    };

                    let or_split = field_split.next().unwrap().trim().split(" or ");

                    for (or_elem, or_term) in or_split.enumerate() {
                        let mut ors = or_term.split("-");
                        range.minmax[or_elem].min = ors.next().unwrap().parse().unwrap();
                        range.minmax[or_elem].max = ors.next().unwrap().parse().unwrap();
                    }

                    ranges.push(range)
                }
            },
            LoadStage::MyTicket => {
                if line.is_empty() {
                    load_stage = LoadStage::OtherTickets
                } else if line != "your ticket:" {
                    for elem in line.split(",") {
                        my_ticket.push(elem.parse().unwrap())
                    }
                }
            },
            LoadStage::OtherTickets => {
                if line != "nearby tickets:" {
                    other_tickets.push(line.split(",").map(|e| e.parse::<u32>().unwrap()).collect());
                }
            }
        }
    }

    Ok((ranges, my_ticket, other_tickets))
}

