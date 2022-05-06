use std::io;

use rustfuck::interpreter::{handler::Handler, memory_tape::MemoryTape, parser::Parser};

fn main() {
    println!("Enter the Brainfuck code:");
    let mut code = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line");

    let instruction_set = Parser::parse_code(code);

    let memory_tape = MemoryTape::new(0);
    let mut handler = Handler::new(memory_tape);

    handler.run(&instruction_set);

    println!("\n\nEnd.")
}
