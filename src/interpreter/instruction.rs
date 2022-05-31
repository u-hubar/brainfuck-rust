use super::sign::SignedInt;

#[derive(Debug)]
pub enum Instruction {
    MoveRight(u16),
    MoveLeft(u16),
    IncrementValue(u8),
    DecrementValue(u8),
    Output,
    Input,
    OpenLoop,
    FindLoopGCD(SignedInt<u8>),
    ExecuteLoopBody(Vec<Instruction>),
    CloseLoop,
    ClearValue,
    MultiplyValue(SignedInt<u16>, SignedInt<u8>),
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
