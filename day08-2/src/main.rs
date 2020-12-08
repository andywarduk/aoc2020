use std::fs;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program = load_program()?;

    let mut corrupted: Vec<usize> = Vec::new();

    for (i, inst) in program.iter().enumerate() {
        match inst.op {
            OpCode::nop | OpCode::jmp => corrupted.push(i),
            _ => {}
        }
    }

    println!("{} possibly corrupted instructions", corrupted.len());

    for i in corrupted {
        let saveop: OpCode = program[i].op;

        match program[i].op {
            OpCode::nop => program[i].op = OpCode::jmp,
            OpCode::jmp => program[i].op = OpCode::nop,
            _ => panic!("Unexpected opcode")
        }

        match execute_program(&program) {
            Ok(s) => {
                println!("{}", s);
                break
            },
            Err(e) => {
                println!("{}", e);
            },
        }

        program[i].op = saveop;
    }

    Ok(())
}

macro_rules! inst_build {
    ($name:ident {
        $($field_name:ident,)*
    }) => {
        #[derive(Debug, Copy, Clone)]
        #[allow(non_camel_case_types)]
        enum $name {
            $($field_name,)*
        }

        impl $name {
            fn from_string(name: &str) -> $name {
                match name {
                    $(stringify!($field_name) => $name::$field_name),*,
                    _ => panic!("opcode '{}' not recognised", name)
                }
            }
        }
    }
}

inst_build! {
    OpCode {
        nop,
        acc,
        jmp,
    }
}

struct Instruction {
    op: OpCode,
    arg: i32
}

fn load_program() -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    // Open the file read only
    let input = fs::File::open("../input08.txt")?;

    // Create a buffered reader on the file
    let inputbuf = io::BufReader::new(input);

    let mut program = Vec::new();

    for line_result in inputbuf.lines() {
        let line = line_result?;

        let mut split = line.split_whitespace();

        let str_inst = split.next().unwrap();
        let inst: OpCode = OpCode::from_string(str_inst);
        let arg = split.next().unwrap().parse::<i32>().unwrap();

        let inst = Instruction {
            op: inst,
            arg: arg
        };

        program.push(inst);
    }

    Ok(program)
}

struct ExecState {
    pc: i32,
    acc: i32,
}

fn execute_program(program: &Vec<Instruction>) -> Result<String, String> {
    let prog_len = program.len();
    let mut touched = vec![false; prog_len];
    let mut state = ExecState {
        pc: 0,
        acc: 0
    };

    loop {
        if state.pc < 0 || state.pc as usize > prog_len {
            Err(format!("pc out of bounds: {}", state.pc))?
        }

        let pc = state.pc as usize;

        if pc == prog_len {
            break
        }

        if touched[pc] {
            Err(format!("Loop found at pc {}. acc is {}", pc, state.acc))?;
        }

        touched[pc] = true;

        let op = &program[pc].op;
        let arg = program[pc].arg;

        // println!("{}: {:?} {}", pc, op, arg);

        state.pc += 1;

        match op {
            OpCode::nop => {},
            OpCode::acc => {
                state.acc += arg;
            },
            OpCode::jmp => {
                state.pc += arg - 1;
            }
        }
    }

    Ok(format!("Program finished. acc is {}", state.acc))
}
