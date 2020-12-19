use std::fs;
use std::io::{self, BufRead};
use std::fmt;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rules, messages) = load_input()?;

    let mut matched = 0;

    for m in &messages {
        if match_message(&m, &rules) {
            matched += 1
        }
    }

    println!("{} messages matched (out of {})", matched, &messages.len());

    Ok(())
}

type RuleNum = u8;
type Rules = HashMap<RuleNum, Rule>;

enum Rule {
    Empty,
    Rule(RuleNum),
    Chain(RuleNum, Box<Rule>),
    Or(Box<Rule>, Box<Rule>),
    Char(char)
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rule::Empty => write!(f, "(empty)"),
            Rule::Rule(r) => write!(f, "{}", r),
            Rule::Chain(rn, br) => write!(f, "{} {}", rn, *br),
            Rule::Or(br1, br2) => write!(f, "{} | {}", *br1, *br2),
            Rule::Char(c) => write!(f, "\"{}\"", c),
        }
    }
}

enum LoadStage {
    Rules,
    Messages
}

fn load_input() -> Result<(Rules, Vec<Vec<char>>), Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input19-2.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let mut load_stage = LoadStage::Rules;

    for line_result in inputbuf.lines() {
        let line = line_result?;

        match load_stage {
            LoadStage::Rules => {
                if line.is_empty() {
                    load_stage = LoadStage::Messages;
                } else {
                    load_rule(&mut rules, line);
                }
            },
            LoadStage::Messages => {
                messages.push(line.chars().collect());
            }
        }
    }

    Ok((rules, messages))
}

fn load_rule(rules: &mut Rules, rulestr: String) {
    let mut split = rulestr.split_whitespace();

    let ruleno = split.next().unwrap().split(":").next().unwrap().parse::<RuleNum>().unwrap();

    let mut rule = Rule::Empty;

    for term in split {
        if term == "|" {
            rule = match rule {
                Rule::Rule(rn) => {
                    Rule::Or(Box::new(Rule::Rule(rn)), Box::new(Rule::Empty))
                },
                Rule::Chain(rn, br) => {
                    Rule::Or(Box::new(Rule::Chain(rn, br)), Box::new(Rule::Empty))
                },
                Rule::Char(c) => {
                    Rule::Or(Box::new(Rule::Char(c)), Box::new(Rule::Empty))
                },
                _ => panic!("Unable to parse")
            };

        } else if let Ok(r) = term.parse::<RuleNum>() {
            rule = match rule {
                Rule::Empty => {
                    Rule::Rule(r)
                },
                Rule::Rule(rn) => {
                    Rule::Chain(rn, Box::new(Rule::Rule(r)))
                },
                Rule::Or(br1, br2) => {
                    match *br2 {
                        Rule::Empty => {
                            Rule::Or(br1, Box::new(Rule::Rule(r)))
                        },
                        Rule::Rule(rn) => {
                            Rule::Or(br1, Box::new(Rule::Chain(rn, Box::new(Rule::Rule(r)))))
                        },
                        Rule::Chain(rn, br3) => {
                            match *br3 {
                                Rule::Rule(r3) => {
                                    Rule::Or(br1, Box::new(Rule::Chain(rn, Box::new(Rule::Chain(r3, Box::new(Rule::Rule(r)))))))
                                },
                                _ => panic!("Unable to parse")
                            }
                        },
                        _ => panic!("Unable to parse")
                    }
                },
                _ => panic!("Unable to parse")
            };

        } else if term.chars().nth(0).unwrap() == '\"' {
            rule = match rule {
                Rule::Empty => {
                    Rule::Char(term.chars().nth(1).unwrap())
                },
                _ => panic!("Unable to parse")
            };

        }
        else {
            panic!("Unable to parse")

        }
    }

    rules.insert(ruleno, rule);
}

fn match_message(message: &Vec<char>, rules: &Rules) -> bool {
    let mut ok = false;

    let rule = rules.get(&0).unwrap();
    
    let pos = match_rule(message, rules, &vec![0], &rule);

    let valid = pos.iter().filter(|&&p| {p == message.len()}).count();

    if valid > 0 {
        ok = true
    }

    ok
}

fn match_rule(message: &Vec<char>, rules: &Rules, posvec: &Vec<usize>, rule: &Rule) -> Vec<usize> {
    let results = match rule {
        Rule::Rule(r) => {
            let rule = rules.get(&r).unwrap();
            match_rule(message, rules, posvec, rule)
        },
        Rule::Chain(r, br) => {
            let rule = rules.get(&r).unwrap();

            let mut next_pos = match_rule(message, rules, posvec, rule);

            if next_pos.len() > 0 {
                next_pos = match_rule(message, rules, &next_pos, &*br)
            }

            next_pos
        },
        Rule::Or(br1, br2) => {
            let mut or = match_rule(message, rules, posvec, &*br1);
            or.append(&mut match_rule(message, rules, posvec, &*br2));
            or
        },
        Rule::Char(c) => {
            posvec.iter().filter_map(|&pos| {
                if pos < message.len() {
                    if message[pos] == *c {
                        Some(pos + 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).collect()
        },
        _ => panic!("Unable to match rule")
    };

    results
}
