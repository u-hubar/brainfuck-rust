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
                Instruction::MoveLeft => self.memory_tape.move_left(),
                Instruction::MoveRight => self.memory_tape.move_right(),
                Instruction::IncrementValue => self.memory_tape.increment(),
                Instruction::DecrementValue => self.memory_tape.decrement(),
                Instruction::Output => self.memory_tape.output(),
                Instruction::Input => self.memory_tape.input(),
                Instruction::ExecuteLoopBody(loop_instructions) => {
                    while self.memory_tape.storage[self.memory_tape.pointer as usize] != 0 {
                        self.run(loop_instructions);
                    }
                },
                _ => {},
            }
        }
    }
}
