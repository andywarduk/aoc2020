use std::fs;
use std::io::{self, BufRead};
use std::collections::BTreeMap;

const COLOUR: &str = "shiny gold";

struct Rule {
    contains: BTreeMap<String, u16>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rules = load_rules()?;

    let matches = walk_rules(&rules);

    println!("{} rules match", matches);

    Ok(())
}

fn walk_rules(rules: &BTreeMap<String, Rule>) -> u16 {
    let mut matches: u16 = 0;

    for (colour, rule) in rules {
        if colour != COLOUR {
            if walk_rule(&rules, &rule) {
                matches += 1;
            }
        }
    }

    matches
}

fn walk_rule(rules: &BTreeMap<String, Rule>, rule: &Rule) -> bool {
    let mut found: bool = false;

    for (colour, _qty) in &rule.contains {
        if colour == COLOUR {
            found = true;
            break;
        }

        if walk_rule(rules, rules.get(colour).unwrap()) {
            found = true;
            break;
        }
    }

    found
}

fn load_rules() -> Result<BTreeMap<String, Rule>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input07.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut rules = BTreeMap::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let rule_split: Vec<_> = line.split(" bags contain ").collect();
        assert!(rule_split.len() == 2, "Should be 2 in split");

        let colour = rule_split[0].to_string();
        let contents = rule_split[1];

        let mut rule = Rule {
            contains: BTreeMap::new()
        };

        if contents != "no other bags." {
            let content_split = contents.split(",").map(|s| {
                let content = s.split(" bag").next().unwrap().trim();
                let term: Vec<_> = content.split(" ").collect();
                let qty = term[0].parse::<u16>().unwrap();
                (qty, term[1..].join(" "))
            });

            for content in content_split {
                rule.contains.insert(content.1, content.0);
            }
        }

        rules.insert(colour, rule);
    }

    Ok(rules)
}
