use std::io::{self, Read};

pub struct MemoryTape {
    storage: Vec<u8>,
    pointer: u16,
}

impl MemoryTape {
    pub fn new(pointer: u16) -> Self {
        let storage: Vec<u8> = vec![0; u16::MAX as usize];

        Self {
            storage,
            pointer,
        }
    }

    pub fn move_left(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.pointer < u16::MAX {
            self.pointer += 1;
        }
    }

    pub fn increment(&mut self) {
        if self.storage[self.pointer as usize] < u8::MAX {
            self.storage[self.pointer as usize] += 1;
        } else {
            self.storage[self.pointer as usize] = 0;
        }
    }

    pub fn decrement(&mut self) {
        if self.storage[self.pointer as usize] > 0 {
            self.storage[self.pointer as usize] -= 1;
        } else {
            self.storage[self.pointer as usize] = u8::MAX;
        }
    }

    pub fn output(&mut self) {
        print!("{}", self.storage[self.pointer as usize] as char)
    }

    pub fn input(&mut self) {
        let mut input: [u8; 1] = [0];
        io::stdin().read_exact(&mut input).expect("Failed to read input from stdin");
        self.storage[self.pointer as usize] = input[0];
    }
}
