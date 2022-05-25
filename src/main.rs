use std::{ffi::OsStr, fs::File, io::{self, prelude::*}, path::Path};

use rustfuck::interpreter::{handler::Handler, memory_tape::MemoryTape, parser::Parser};

fn main() {
    println!("Please, choose option for code loading:");
    println!("[1] Manual");
    println!("[2] File");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    option = option.trim_end().to_string();

    let mut code = String::new();

    match option.as_str() {
        "1" => {
            read_input(&mut code, "\nEnter the Brainfuck code:");
        },
        "2" => {
            let mut filename = String::new();
            read_input(&mut filename, "\nEnter filename:");

            if !valid_extension(filename.as_str(), vec!("bf", "rf")) {
                panic!("Extension for {} is not supported.", filename)
            }

            let mut file = File::open(filename)
                .expect("File not found.");

            file.read_to_string(&mut code)
                .expect("Failed to read the file");
        },
        _ => { panic!("'{}' is not supported!", option); },
    };

    let instruction_set = Parser::parse_code(code).unwrap();

    let memory_tape = MemoryTape::new(0);
    let mut handler = Handler::new(memory_tape);

    print!("\nOutput: ");
    handler.run(&instruction_set);

    println!("\n\nEnd.")
}

fn read_input(string: &mut String, text: &str) -> String {
    println!("{}", text);

    io::stdin()
        .read_line(string)
        .expect("Failed to read line");

    let truncate_len = string.trim_end_matches(&['\r', '\n'][..]).len();
    string.truncate(truncate_len);

    string.to_string()
}

fn valid_extension(filename: &str, extensions: Vec<&str>) -> bool {
    let file_ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    extensions.contains(&file_ext)
}
