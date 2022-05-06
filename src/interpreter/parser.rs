use super::instruction::Instruction;

pub struct Parser;

impl Parser {
    pub fn parse_code(code: String) -> Vec<Instruction> {
        let mut instruction_set = Vec::new();

        for char in code.chars() {
            instruction_set.push(Instruction::from(&char));
        }

        instruction_set
    }
}
