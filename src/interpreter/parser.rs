use std::collections::HashMap;

use super::{error::ParseError, instruction::Instruction, sign::SignedInt};

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
                        let mut loop_body = Parser::parse_code(
                            &code[loop_start..i]
                        ).unwrap();

                        let mut depth: isize = 0;
                        let mut loop_muls: HashMap<isize, isize> = HashMap::new();

                        for inner_instr in loop_body.iter() {
                            match *inner_instr {
                                Instruction::MoveRight(step) => depth += step as isize,
                                Instruction::MoveLeft(step) => depth -= step as isize,
                                Instruction::IncrementValue(val) => {
                                    *loop_muls.entry(depth).or_insert(0) += val as isize;
                                }
                                Instruction::DecrementValue(val) => {
                                    *loop_muls.entry(depth).or_insert(0) -= val as isize;
                                }
                                _ => {
                                    loop_muls.clear();
                                    break;
                                },
                            }
                        }

                        if loop_muls.contains_key(&0) && depth == 0 {
                            let zero_depth_multiplicand = loop_muls.remove(&0).unwrap();
                            loop_body.clear();

                            instruction_set.push(
                                Instruction::SolveLoopDiophantine(
                                    zero_depth_multiplicand
                                )
                            );
                            
                            for (depth, multiplicand) in loop_muls {
                                instruction_set.push(
                                    Instruction::MultiplyValue(
                                        SignedInt::try_from(depth).unwrap(),
                                        SignedInt::try_from(multiplicand).unwrap(),
                                    )
                                );
                            }

                            instruction_set.push(Instruction::ClearValue);
                        } else {
                            instruction_set.push(Instruction::ExecuteLoopBody(loop_body));
                        }
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
                            Instruction::MoveLeft(val) => {
                                *val += 1;
                                continue;
                            },
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
