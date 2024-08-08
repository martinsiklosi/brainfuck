# A Brainfuck Interpreter Written In Rust

## Setup

If you don't have Rust installed, [install Rust](https://www.rust-lang.org/tools/install).

Clone the repository and compile with
```
cargo build --release
```

The executable will be located at `target/release/brainfuck.exe`.

Add the executable location to PATH in order to use the interpreter from anywhere.

## Usage

Run a Brainfuck file with
```
brainfuck <PATH>
```

## Implementation Decisions
The memory is dynamically allocated, giving the Brainfuck program practically infinite memory.
In reality, the memory is limited by the limitations of your machine, what the os will let you do, and the maximum size of the data pointer in the implementation
(18,446,744,073,709,551,615 on a 64-bit system,
4,294,967,295 on a 32-bit system).

## Misc
The `samples` directory contains some example Brainfuck programs, some cool programs taken from the internet, and a couple of test cases.
