#[derive(Debug)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
    Ignore,
}

impl From<&char> for Instruction {
    fn from(instr_char: &char) -> Self {
        match instr_char {
            '>' => Instruction::MoveRight,
            '<' => Instruction::MoveLeft,
            '+' => Instruction::IncrementValue,
            '-' => Instruction::DecrementValue,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::OpenLoop,
            ']' => Instruction::CloseLoop,
            _ => Instruction::Ignore,
        }
    }
}
