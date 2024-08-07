use clap::Parser;
use std::{fmt, fs, process, str};
use text_io::read;

#[derive(Clone)]
enum Instruction {
    IncPointer,
    DecPointer,
    IncByte,
    DecByte,
    Output,
    Input,
    EmptyOpenBracket,
    EmptyCloseBracket,
    OpenBracket { matching_location: usize },
    CloseBracket { matching_location: usize },
}

type Bytecode = Vec<Instruction>;

#[derive(Debug)]
struct CompileError {
    message: String,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompileError: {}", self.message)
    }
}

#[derive(Debug)]
struct RuntimeError {
    message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.message)
    }
}

fn parse_character(character: char) -> Option<Instruction> {
    match character {
        '>' => Some(Instruction::IncPointer),
        '<' => Some(Instruction::DecPointer),
        '+' => Some(Instruction::IncByte),
        '-' => Some(Instruction::DecByte),
        '.' => Some(Instruction::Output),
        ',' => Some(Instruction::Input),
        '[' => Some(Instruction::EmptyOpenBracket),
        ']' => Some(Instruction::EmptyCloseBracket),
        _ => None,
    }
}

fn match_brackets(bytecode: &Bytecode) -> Result<Bytecode, CompileError> {
    let open_count = bytecode
        .iter()
        .filter(|instruction| matches!(instruction, Instruction::EmptyOpenBracket))
        .count();
    let close_count = bytecode
        .iter()
        .filter(|instruction| matches!(instruction, Instruction::EmptyCloseBracket))
        .count();
    if open_count != close_count {
        return Err(CompileError {
            message: "Unbalanced brackets".to_string(),
        });
    }

    let mut result = bytecode.clone();
    let mut open_locations = Vec::new();
    for (i, instruction) in bytecode.iter().enumerate() {
        match instruction {
            Instruction::EmptyOpenBracket => {
                open_locations.push(i);
            }
            Instruction::EmptyCloseBracket => {
                let open_location = open_locations.pop().expect("Brackets should be balanced");
                result[i] = Instruction::CloseBracket {
                    matching_location: open_location,
                };
                result[open_location] = Instruction::OpenBracket {
                    matching_location: i,
                };
            }
            _ => (),
        }
    }
    Ok(result)
}

fn compile(source_code: String) -> Result<Bytecode, CompileError> {
    let bytecode: Vec<Instruction> = source_code.chars().filter_map(parse_character).collect();
    match_brackets(&bytecode)
}

fn execute(bytecode: &Bytecode) -> Result<(), RuntimeError> {
    const MEMORY_SIZE: usize = 30_000;
    let mut memory = [0u8; MEMORY_SIZE];
    let mut data_pointer = 0;
    let mut instruction_pointer = 0;
    while instruction_pointer < bytecode.len() {
        match bytecode[instruction_pointer] {
            Instruction::IncPointer => data_pointer += 1,
            Instruction::DecPointer => data_pointer -= 1,
            Instruction::IncByte => memory[data_pointer] += 1,
            Instruction::DecByte => memory[data_pointer] -= 1,
            Instruction::Output => print!("{}", memory[data_pointer] as char),
            Instruction::Input => {
                let input: String = read!("{}\n");
                memory[data_pointer] = input.bytes().next().unwrap();
            }
            Instruction::OpenBracket {
                matching_location: jump_location,
            } => {
                if memory[data_pointer] == 0 {
                    instruction_pointer = jump_location;
                    continue;
                }
            }
            Instruction::CloseBracket {
                matching_location: jump_location,
            } => {
                if memory[data_pointer] != 0 {
                    instruction_pointer = jump_location;
                    continue;
                }
            }
            _ => (),
        }
        instruction_pointer += 1;
        if data_pointer >= MEMORY_SIZE {
            return Err(RuntimeError {
                message: "Exceeded memory bounds".to_string(),
            });
        }
    }
    Ok(())
}

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let source_code = match fs::read_to_string(args.path) {
        Ok(source_code) => source_code,
        Err(error) => {
            println!("{}", error);
            process::exit(1)
        }
    };
    let bytecode = match compile(source_code) {
        Ok(bytecode) => bytecode,
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };
    match execute(&bytecode) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };
}
