use super::{instruction::Instruction, memory_tape::MemoryTape};

pub struct Handler {
    memory_tape: MemoryTape,
    loop_stack: Vec<usize>,
}

impl Handler {
    pub fn new(memory_tape: MemoryTape) -> Self {
        Self {
            memory_tape,
            loop_stack:Vec::new(),
        }
    }

    pub fn run(&mut self, instruction_set: &[Instruction]) {
        let mut loop_start_pointer = self.memory_tape.pointer;
        for (i, instruction) in instruction_set.iter().enumerate() {
            match instruction {
                Instruction::MoveLeft => self.memory_tape.move_left(),
                Instruction::MoveRight => self.memory_tape.move_right(),
                Instruction::IncrementValue => self.memory_tape.increment(),
                Instruction::DecrementValue => self.memory_tape.decrement(),
                Instruction::Output => self.memory_tape.output(),
                Instruction::Input => self.memory_tape.input(),
                Instruction::OpenLoop => {
                    self.loop_stack.push(i);
                    loop_start_pointer = self.memory_tape.pointer;
                },
                Instruction::CloseLoop => {
                    while self.memory_tape.storage[loop_start_pointer as usize] != 0 {
                        let loop_start: usize = self.loop_stack.last().unwrap() + 1;
                        let loop_end: usize = i;
                        self.run(&instruction_set[loop_start..loop_end]);
                    }
                    self.loop_stack.pop();
                },
                _ => {},
            }
        }
    }
}
