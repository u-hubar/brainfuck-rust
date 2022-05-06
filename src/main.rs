use std::io;

use rustfuck::interpreter::parser::Parser;

fn main() {
    println!("Enter the Brainfuck code:");
    let mut code = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line");

    let instruction_set = Parser::parse_code(code);

    for instruction in instruction_set.iter() {
        println!("{:?}", instruction)
    }
}
