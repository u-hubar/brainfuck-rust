use super::{error::ParseError, instruction::Instruction};

pub struct Parser;

impl Parser {
    pub fn parse_code(code: &str) -> Result<Vec<Instruction>, ParseError> {
        let mut instruction_set = Vec::new();
        let mut loop_start = 0;
        let mut opened_loops = 0;
        let mut last_char = '#';

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
                    if opened_loops != 0 {
                        continue;
                    }

                    if instr_char == last_char {
                        match instruction_set.last_mut().unwrap() {
                            Instruction::MoveRight(val) |
                            Instruction::MoveLeft(val) |
                            Instruction::IncrementValue(val) |
                            Instruction::DecrementValue(val) => {
                                *val += 1;
                                continue;
                            },
                            _ => {},
                        };
                    }

                    instruction_set.push(instruction);
                },
            }

            last_char = instr_char;
        }

        if opened_loops == 0 {
            Ok(instruction_set)
        } else  {
            Err(ParseError::InvalidOpenBracket)
        }
    }
}
