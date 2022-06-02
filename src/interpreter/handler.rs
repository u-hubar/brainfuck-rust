use super::{diophantine::Diophantine, instruction::Instruction, memory_tape::MemoryTape};

pub struct Handler {
    memory_tape: MemoryTape,
}

impl Handler {
    pub fn new(memory_tape: MemoryTape) -> Self {
        Self { memory_tape }
    }

    pub fn run(&mut self, instruction_set: &Vec<Instruction>) {
        let mut last_loop_iters = 0;

        for instruction in instruction_set {
            match instruction {
                Instruction::MoveLeft(step) => self.memory_tape.move_left(*step),
                Instruction::MoveRight(step) => self.memory_tape.move_right(*step),
                Instruction::IncrementValue(addend) => self.memory_tape.increment(*addend),
                Instruction::DecrementValue(subtrahend) => self.memory_tape.decrement(*subtrahend),
                Instruction::SolveLoopDiophantine(zero_depth_multiplicand) => {
                    last_loop_iters = Diophantine::solve_extended_gcd(
                        *zero_depth_multiplicand,
                        -((u8::MAX as isize) + 1),
                        self.memory_tape.actual_cell() as isize,
                    );
                },
                Instruction::MultiplyValue(offset, multiplicand) => {
                    self.memory_tape.multiply(last_loop_iters, *offset, *multiplicand)
                },
                Instruction::ClearValue => self.memory_tape.clear(),
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
