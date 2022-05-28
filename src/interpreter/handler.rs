use super::{instruction::Instruction, memory_tape::MemoryTape};

pub struct Handler {
    memory_tape: MemoryTape,
}

impl Handler {
    pub fn new(memory_tape: MemoryTape) -> Self {
        Self { memory_tape }
    }

    pub fn run(&mut self, instruction_set: &[Instruction]) {
        for instruction in instruction_set.iter() {
            match instruction {
                Instruction::MoveLeft(val) => self.memory_tape.move_left(*val as u16),
                Instruction::MoveRight(val) => self.memory_tape.move_right(*val as u16),
                Instruction::IncrementValue(val) => self.memory_tape.increment(*val as u8),
                Instruction::DecrementValue(val) => self.memory_tape.decrement(*val as u8),
                Instruction::Output => self.memory_tape.output(),
                Instruction::Input => self.memory_tape.input(),
                Instruction::ExecuteLoopBody(loop_instructions) => {
                    if loop_instructions.len() == 1 {
                        match loop_instructions[0] {
                            Instruction::IncrementValue(_) |
                            Instruction::DecrementValue(_) => {
                                self.memory_tape.clear();
                            },
                            _ => {},
                        };
                    }

                    while self.memory_tape.storage[self.memory_tape.pointer as usize] != 0 {
                        self.run(loop_instructions);
                    }
                },
                _ => {},
            }
        }
    }
}
