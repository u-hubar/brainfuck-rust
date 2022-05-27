use super::{error::ParseError, instruction::Instruction};

pub struct Parser;

impl Parser {
    pub fn parse_code(code: &str) -> Result<Vec<Instruction>, ParseError> {
        let mut instruction_set = Vec::new();
        let mut loop_start = 0;
        let mut opened_loops = 0;

        for (i, instr_char) in code.chars().enumerate() {
            match Instruction::from(&instr_char) {
                Instruction::OpenLoop => {
                    if opened_loops == 0 {
                        loop_start = i + 1;
                    }

                    opened_loops += 1;
                },
                Instruction::CloseLoop => {
                    if opened_loops == 1 {
                        instruction_set.push(
                            Instruction::ExecuteLoopBody(
                                Parser::parse_code(
                                    &code[loop_start..i]
                                ).unwrap()
                            )
                        );
                    } else if opened_loops == 0 {
                        return Err(ParseError::InvalidCloseBracket)
                    };

                    opened_loops -= 1;
                },
                Instruction::Ignore => continue,
                instruction => {
                    if opened_loops == 0 {
                        instruction_set.push(instruction);
                    }
                },
            }
        }

        if opened_loops == 0 {
            Ok(instruction_set)
        } else  {
            Err(ParseError::InvalidOpenBracket)
        }
    }
}
