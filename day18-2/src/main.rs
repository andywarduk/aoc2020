use std::fs;
use std::io::{self, BufRead};
use std::fmt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expressions = load_expressions()?;

    let mut total: u64 = 0;

    for e in expressions {
        let te = transform_expression(&e);

        let result = evaluate_expression(&te);

        println!("{} => {} = {}", &e, &te, result);

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
                // Start of bracketed expression
                expression.terms.push(Term::Expression(parse_bracket(line, pos)?));
            }
            ')' => {
                // End of expression
                break
            }
            _ => {
                if c.is_digit(10) {
                    // Start of an integer
                    expression.terms.push(Term::Integer(parse_integer(line, pos)?));
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

fn parse_integer(line: &Vec<char>, pos: &mut usize) -> Result<u32, Box<dyn std::error::Error>> {
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

fn transform_expression(e: &Expression) -> Expression {
    let mut e2: Expression = Default::default();
    let mut lhs: Term = Term::Integer(0);
    let mut last_op: char = ' ';

    let mut push_op = |t1: Term, op: char, t2: Term| -> Term {
        match op {
            ' ' => t2,
            '+' => {
                // Transform additions in to a standalone expression
                let mut add_exp: Expression = Default::default();

                add_exp.terms.push(t1);
                add_exp.terms.push(Term::Operator(op));
                add_exp.terms.push(t2);

                Term::Expression(add_exp)
            }
            _ => {
                // Push terms with no transformation
                e2.terms.push(t1);
                e2.terms.push(Term::Operator(op));
                t2
            }
        }
    };

    for t in &e.terms {
        match t {
            Term::Integer(n) => {
                let t = Term::Integer(*n);
                lhs = push_op(lhs, last_op, t);
            }
            Term::Operator(c) => {
                last_op = *c;
            }
            Term::Expression(e) => {
                let te = Term::Expression(transform_expression(&e));
                lhs = push_op(lhs, last_op, te);
            }
        }
    }

    e2.terms.push(lhs);

    e2
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
