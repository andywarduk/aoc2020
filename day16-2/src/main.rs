use std::fs;
use std::io::{self, BufRead};
use std::collections::BTreeMap;

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
    let (ranges, my_ticket, other_tickets) = load_info()?;
    let field_count = my_ticket.len();

    // Get vector of valid other tickets
    let valid_other: Vec<_> = valid_other(&other_tickets, &ranges);

    // Create sets for each ticket number
    let mut valid_fields: Vec<BTreeMap<&String, &Ranges>> = Vec::new();

    for _i in 0..field_count {
        let mut set: BTreeMap<&String, &Ranges> = BTreeMap::new();

        for r in &ranges {
            set.insert(&r.field, r);
        }

        valid_fields.push(set);
    }

    // Whittle down the valid fields
    for o in valid_other {
        for (i, bt) in valid_fields.iter_mut().enumerate() {
            let mut remove: Vec<&String> = Vec::new();

            let val = o[i];

            for (&n, f) in bt.iter() {
                if (val < f.minmax[0].min || val > f.minmax[0].max) &&
                   (val < f.minmax[1].min || val > f.minmax[1].max) {
                    remove.push(n);
                }
            }

            if remove.len() > 0 {
                for r in remove {
                    bt.remove(r);
                }
            }
        }
    }

    // Iteratively eliminate fields which must be correct (one have one possible field)
    let mut fields: Vec<Option<String>> = Vec::new();

    for _i in 0..field_count {
        fields.push(None)
    }

    let mut finished = false;

    while !finished {
        finished = true;

        for i in 0..field_count {
            if valid_fields[i].len() == 1 {
                if fields[i] == None {
                    let found = valid_fields[i].iter().next().unwrap().0.to_string();
                    for j in 0..field_count {
                        if j != i {
                            valid_fields[j].remove(&found);
                        }
                    }
                    fields[i] = Some(found);
                    finished = false;
                    break
                }
            } else {
                finished = false
            }
        }
    }

    // Total the fields beginning with 'departure '
    let mut total = 1u64;

    for i in 0..field_count {
        if fields[i].as_ref().unwrap().starts_with("departure ") {
            total *= my_ticket[i] as u64;
        }
    }

    println!("Total is {}", total);

    Ok(())
}

fn valid_other<'a>(other_tickets: &'a Vec<Vec<u32>>, ranges: &Vec<Ranges>) -> Vec<&'a Vec<u32>> {
    let valid = other_tickets.iter().filter(|&t| {
        for field in t {
            let mut field_ok: bool = false;

            for range in ranges {
                for minmax in 0..2 {
                    if *field >= range.minmax[minmax].min && *field <= range.minmax[minmax].max {
                        field_ok = true;
                        break
                    }
                }

                if field_ok {
                    break
                }
            }

            if !field_ok {
                return false
            }
        }

        true
    }).collect::<Vec<&Vec<u32>>>();

    valid.to_vec()
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
