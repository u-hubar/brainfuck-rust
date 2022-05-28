#[derive(Debug)]
pub enum Instruction {
    MoveRight(usize),
    MoveLeft(usize),
    IncrementValue(usize),
    DecrementValue(usize),
    Output,
    Input,
    OpenLoop,
    ExecuteLoopBody(Vec<Instruction>),
    CloseLoop,
    Ignore,
}

impl From<&char> for Instruction {
    fn from(instr_char: &char) -> Self {
        match instr_char {
            '>' => Instruction::MoveRight(1),
            '<' => Instruction::MoveLeft(1),
            '+' => Instruction::IncrementValue(1),
            '-' => Instruction::DecrementValue(1),
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::OpenLoop,
            ']' => Instruction::CloseLoop,
            _ => Instruction::Ignore,
        }
    }
}
