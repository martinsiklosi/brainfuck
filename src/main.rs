use std::str;

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

type ByteCode = Vec<Instruction>;

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

fn match_bracket_locations(bytecode: &ByteCode) -> ByteCode {
    let mut result = bytecode.clone();
    let mut open_locations = Vec::new();
    for (i, instruction) in bytecode.iter().enumerate() {
        match instruction {
            Instruction::EmptyOpenBracket => {
                open_locations.push(i);
            }
            Instruction::EmptyCloseBracket => {
                let open_location = open_locations.pop().expect("Unbalanced brackets");
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

fn compile(program: &str) -> ByteCode {
    let bytecode: Vec<Instruction> = program
        .chars()
        .filter_map(|character| parse_character(character))
        .collect();
    match_bracket_locations(&bytecode)
}

fn print_byte(byte: &u8) -> () {
    let utf8_array = [byte.to_owned(); 1];
    let s = str::from_utf8(&utf8_array).expect("Unprintable byte");
    print!("{}", s);
}

fn execute(bytecode: &ByteCode) -> () {
    const MEMORY_SIZE: usize = 30_000;
    let mut memory = [0u8; MEMORY_SIZE];
    let mut memory_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;
    while instruction_pointer < bytecode.len() {
        match bytecode[instruction_pointer] {
            Instruction::IncPointer => memory_pointer += 1,
            Instruction::DecPointer => memory_pointer -= 1,
            Instruction::IncByte => memory[memory_pointer] += 1,
            Instruction::DecByte => memory[memory_pointer] -= 1,
            Instruction::Output => print_byte(&memory[memory_pointer]),
            Instruction::Input => (), // TODO
            Instruction::OpenBracket {
                matching_location: jump_location,
            } => {
                if memory[memory_pointer] == 0 {
                    instruction_pointer = jump_location;
                    continue;
                }
            }
            Instruction::CloseBracket {
                matching_location: jump_location,
            } => {
                if memory[memory_pointer] != 0 {
                    instruction_pointer = jump_location;
                    continue;
                }
            }
            _ => (),
        }
        instruction_pointer += 1;
    }
}

fn main() {
    let hello_world = "hello word!++++++++[>++++[>++>+++>+++>+<<<<-]>+>->+>>+[<]<-]>>.>
    >---.+++++++..+++.>.<<-.>.+++.------.--------.>+.>++.";

    let bytecode = compile(hello_world);
    // print_bytecode(&bytecode);
    execute(&bytecode);
}
