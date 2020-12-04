use std::fs;
use std::io::{self, BufRead};

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
    pub fn is_valid(&self) -> bool {
        !self.byr.is_empty() &&
        !self.iyr.is_empty() &&
        !self.eyr.is_empty() &&
        !self.hgt.is_empty() &&
        !self.hcl.is_empty() &&
        !self.ecl.is_empty() &&
        !self.pid.is_empty()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = load_creds()?;

    let mut valid: i16 = 0;

    for cred in creds {
        if cred.is_valid() {
            valid += 1
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