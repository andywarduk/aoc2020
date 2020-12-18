use std::fs;
use std::io::{self, BufRead};
use std::fmt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expressions = load_expressions()?;

    let mut total: u64 = 0;

    for e in expressions {
        let result = evaluate_expression(&e);

        println!("{} = {}", &e, result);

        total += result;
    }

    println!("Total: {}", total);

    Ok(())
}

fn load_expressions() -> Result<Vec<Expression>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input18.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut expressions = Vec::new();

    for line_result in inputbuf.lines() {
        let line: Vec<_> = line_result?.chars().collect();

        let expression = parse_expression(&line, &mut 0)?;

        expressions.push(expression);
    }

    Ok(expressions)
}

fn parse_expression(line: &Vec<char>, pos: &mut usize) -> Result<Expression, Box<dyn std::error::Error>> {
    let mut expression: Expression = Default::default();

    while *pos < line.len() {
        let c = line[*pos];

        match c {
            ' ' => {}, // Skip whitespace
            '*' | '+' => {
                // Operator
                expression.terms.push(Term::Operator(c))
            },
            '(' => {
                expression.terms.push(Term::Expression(parse_bracket(line, pos)?));
            }
            ')' => {
                break
            }
            _ => {
                if c.is_digit(10) {
                    expression.terms.push(Term::Integer(parse_number(line, pos)?));
                } else {
                    Err(format!("Unexpected character {}", c))?;
                }
            }
        }

        *pos += 1;
    }

    Ok(expression)
}

fn parse_bracket(line: &Vec<char>, pos: &mut usize) -> Result<Expression, Box<dyn std::error::Error>> {
    assert!(line[*pos] == '(', "Expecting bracket");
    *pos += 1;

    let expression = parse_expression(line, pos)?;

    Ok(expression)
}

fn parse_number(line: &Vec<char>, pos: &mut usize) -> Result<u32, Box<dyn std::error::Error>> {
    let mut scan_pos = *pos;

    loop {
        scan_pos += 1;

        if scan_pos >= line.len() {
            break
        }

        if !line[scan_pos].is_digit(10) {
            break
        }
    }

    let num_str: String = line[*pos..scan_pos].iter().collect();

    scan_pos -= 1;
    *pos = scan_pos;

    let number = num_str.parse()?;

    Ok(number)
}

fn evaluate_expression(e: &Expression) -> u64 {
    let mut result: u64 = 0;
    let mut last_op: char = ' ';

    let mut apply_value = |value: u64, op: &char| {
        result = match *op {
            ' ' => value,
            '+' => result + value,
            '*' => result * value,
            _ => {
                println!("Unrecognised operator {}", op);
                result
            }
        };
    };

    for t in &e.terms {
        match t {
            Term::Integer(n) => {
                apply_value(*n as u64, &last_op);
            }
            Term::Operator(c) => {
                last_op = *c;
            }
            Term::Expression(e) => {
                apply_value(evaluate_expression(e), &last_op);
            }
        }
    }

    result
}

#[derive(Default, Debug)]
struct Expression {
    terms: Vec<Term>
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in &self.terms {
            write!(f, "{}", t)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Term {
    Integer(u32),
    Operator(char),
    Expression(Expression),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Integer(n) => { write!(f, "{}", n)? }
            Term::Operator(c) => { write!(f, "{}", c)? }
            Term::Expression(e) => { write!(f, "({})", e)? }
        }

        Ok(())
    }
}
