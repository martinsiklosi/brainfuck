use clap::Parser;
use std::{fs, str};
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

fn match_brackets(bytecode: &Bytecode) -> Bytecode {
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
    result
}

fn compile(source_code: String) -> Bytecode {
    let bytecode: Vec<Instruction> = source_code.chars().filter_map(parse_character).collect();
    match_brackets(&bytecode)
}

fn execute(bytecode: &Bytecode) {
    const MEMORY_SIZE: usize = 30_000;
    let mut memory = [0u8; MEMORY_SIZE];
    let mut data_pointer = 0;
    let mut instruction_pointer = 0;
    while instruction_pointer < bytecode.len() && data_pointer < MEMORY_SIZE {
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
    }
}

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let source_code = fs::read_to_string(args.path).expect("Path should be valid");
    let bytecode = compile(source_code);
    execute(&bytecode);
}
