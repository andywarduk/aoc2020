#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs;
use std::io::{self, BufRead};

use regex::Regex;

macro_rules! struct_reflect {
    (str_struct $name:ident {
        $($field_name:ident,)*
    }) => {
        #[derive(Default)]
        struct $name {
            $($field_name: String,)*
        }

        impl $name {
            fn set_field(&mut self, name: &str, value: String) {
                match(name) {
                    $(stringify!($field_name) => self.$field_name = value),*,
                    _ => panic!("Field name '{}' not recognised", name)
                }
            }
        }
    }
}

struct_reflect! {
    str_struct Cred {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid,
    }
}

impl Cred {
    pub fn is_valid(&self) -> Result<(), Box<dyn std::error::Error>> {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let byr = self.byr.parse::<u16>()?;
        if byr < 1920 || byr > 2002 { Err(format!("Invalid byr: {}", self.byr))? }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let iyr = self.iyr.parse::<u16>()?;
        if iyr < 2010 || iyr > 2020 { Err(format!("Invalid iyr: {}", self.iyr))? }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let eyr = self.eyr.parse::<u16>()?;
        if eyr < 2020 || eyr > 2030 { Err(format!("Invalid eyr: {}", self.eyr))? }

        // hgt (Height) - a number followed by either cm or in:
        //    If cm, the number must be at least 150 and at most 193.
        //    If in, the number must be at least 59 and at most 76.
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }

        let cap_opt = HGT_RE.captures(&self.hgt);

        match cap_opt {
            None => Err(format!("Invalid hgt: {}", self.hgt))?,
            Some(cap) => {
                let value = cap.get(1).unwrap().as_str().parse::<u16>()?;
                let unit = cap.get(2).unwrap();
                match unit.as_str() {
                    "cm" => {
                        if value < 150 || value > 193 { Err(format!("Invalid cm hgt: {}", value))? }
                    },
                    "in" => {
                        if value < 59 || value > 76 { Err(format!("Invalid in hgt: {}", value))? }
                    }
                    _ => {
                        Err(format!("Invalid hgt unit: {}", unit.as_str()))?
                    }
                }
            }
        };

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
        }

        if !HCL_RE.is_match(&self.hcl) { Err(format!("Invalid hcl: {}", self.hcl))? }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        match &self.ecl[..] {
            "amb" => {},
            "blu" => {},
            "brn" => {},
            "gry" => {},
            "grn" => {},
            "hzl" => {},
            "oth" => {},
            _ => { Err(format!("Invalid ecl: {}", self.ecl))? }
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        lazy_static! {
            static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if !PID_RE.is_match(&self.pid) { Err(format!("Invalid pid: {}", self.pid))? }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = load_creds()?;

    let mut valid: i16 = 0;

    for cred in creds {
        match cred.is_valid() {
            Ok(_) => { valid += 1},
            Err(e) => { println!("{}", e) }
        }
    }

    println!("{} valid credentials", valid);

    Ok(())
}

fn load_creds() -> Result<Vec<Cred>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input04.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut creds = Vec::new();

    let mut cred_opt: Option<Cred> = None;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        if line.is_empty() {
            if cred_opt.is_some() {
                creds.push(cred_opt.unwrap());
                cred_opt = None;
            }

        } else {
            let mut cred: Cred;

            match cred_opt {
                Some(_) => cred = cred_opt.unwrap(),
                None => cred = Default::default()
            }

            let elems = line.split_whitespace();
            
            for elem in elems {
                let terms: Vec<_> = elem.split(":").collect();

                if terms.len() != 2 {
                    Err(format!("Invalid term: {}", elem))?
                }

                cred.set_field(terms[0], terms[1].to_string());
            }

            cred_opt = Some(cred)
        }
    }

    if cred_opt.is_some() {
        creds.push(cred_opt.unwrap())
    }

    Ok(creds)
}