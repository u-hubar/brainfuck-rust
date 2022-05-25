use super::{error::ParseError, instruction::Instruction};

pub struct Parser;

impl Parser {
    pub fn parse_code(code: String) -> Result<Vec<Instruction>, ParseError> {
        let mut instruction_set = Vec::new();
        let mut unclosed_loops = 0;

        for instr_char in code.chars() {
            let instruction = Instruction::from(&instr_char);

            match instruction {
                Instruction::OpenLoop => {
                    unclosed_loops += 1;
                },
                Instruction::CloseLoop => {
                    unclosed_loops -= 1;
                },
                Instruction::Ignore => { continue; },
                _ => {}
            }

            instruction_set.push(instruction);
        }

        if unclosed_loops == 0 {
            Ok(instruction_set)
        } else if unclosed_loops < 0 {
            Err(ParseError::InvalidCloseBracket)
        } else  {
            Err(ParseError::InvalidOpenBracket)
        }
    }
}
