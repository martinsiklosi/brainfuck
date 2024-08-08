use clap::Parser;
use std::{collections::VecDeque, fmt, fs, process, str};
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
    OpenBracket { jump_location: usize },
    CloseBracket { jump_location: usize },
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

fn brackets_are_balanced(bytecode: &Bytecode) -> bool {
    let open_count = bytecode
        .iter()
        .filter(|instruction| matches!(instruction, Instruction::EmptyOpenBracket))
        .count();
    let close_count = bytecode
        .iter()
        .filter(|instruction| matches!(instruction, Instruction::EmptyCloseBracket))
        .count();
    open_count == close_count
}

fn match_brackets(bytecode: &Bytecode) -> Result<Bytecode, CompileError> {
    if !brackets_are_balanced(bytecode) {
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
                    jump_location: open_location,
                };
                result[open_location] = Instruction::OpenBracket { jump_location: i };
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

struct State {
    memory: VecDeque<u8>,
    data_pointer: usize,
    instruction_pointer: usize,
}

impl State {
    fn new() -> Self {
        Self {
            memory: VecDeque::from(vec![0u8]),
            data_pointer: 0,
            instruction_pointer: 0,
        }
    }

    fn inc_pointer(mut self) -> Result<Self, RuntimeError> {
        if self.data_pointer == usize::MAX {
            return Err(RuntimeError {
                message: "Out of memory".to_string(),
            });
        }
        self.data_pointer += 1;
        if self.data_pointer == self.memory.len() {
            self.memory.push_back(0u8);
        }
        self.instruction_pointer += 1;
        Ok(self)
    }

    fn dec_pointer(mut self) -> Result<Self, RuntimeError> {
        if self.data_pointer == 0 && self.memory.len() == usize::MAX {
            return Err(RuntimeError {
                message: "Out of memory".to_string(),
            });
        }
        if self.data_pointer == 0 {
            self.memory.push_front(0u8);
        } else {
            self.data_pointer -= 1;
        }
        self.instruction_pointer += 1;
        Ok(self)
    }

    fn inc_byte(mut self) -> Result<Self, RuntimeError> {
        self.memory[self.data_pointer] += 1;
        self.instruction_pointer += 1;
        Ok(self)
    }

    fn dec_byte(mut self) -> Result<Self, RuntimeError> {
        self.memory[self.data_pointer] -= 1;
        self.instruction_pointer += 1;
        Ok(self)
    }

    fn output(mut self) -> Result<Self, RuntimeError> {
        print!("{}", self.memory[self.data_pointer] as char);
        self.instruction_pointer += 1;
        Ok(self)
    }

    fn input(mut self) -> Result<Self, RuntimeError> {
        let input: String = read!("{}\n");
        match input.bytes().next() {
            Some(byte) => self.memory[self.data_pointer] = byte,
            None => {
                return Err(RuntimeError {
                    message: "Error taking input".to_string(),
                })
            }
        }
        self.instruction_pointer += 1;
        Ok(self)
    }

    fn open_bracket(mut self, jump_location: usize) -> Result<Self, RuntimeError> {
        if self.memory[self.data_pointer] == 0 {
            self.instruction_pointer = jump_location;
        } else {
            self.instruction_pointer += 1;
        }
        Ok(self)
    }

    fn close_bracket(mut self, jump_location: usize) -> Result<Self, RuntimeError> {
        if self.memory[self.data_pointer] != 0 {
            self.instruction_pointer = jump_location;
        } else {
            self.instruction_pointer += 1;
        }
        Ok(self)
    }
}

fn execute(bytecode: &Bytecode) -> Result<(), RuntimeError> {
    let mut state = State::new();
    while state.instruction_pointer < bytecode.len() {
        match bytecode[state.instruction_pointer] {
            Instruction::IncPointer => {
                state = state.inc_pointer()?;
            }
            Instruction::DecPointer => {
                state = state.dec_pointer()?;
            }
            Instruction::IncByte => {
                state = state.inc_byte()?;
            }
            Instruction::DecByte => {
                state = state.dec_byte()?;
            }
            Instruction::Output => {
                state = state.output()?;
            }
            Instruction::Input => {
                state = state.input()?;
            }
            Instruction::OpenBracket { jump_location } => {
                state = state.open_bracket(jump_location)?;
            }
            Instruction::CloseBracket { jump_location } => {
                state = state.close_bracket(jump_location)?;
            }
            _ => (),
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
