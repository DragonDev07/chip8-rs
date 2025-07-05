use crate::constants::{FONTSET, FONTSET_SIZE, MEMORY_SIZE, STACK_SIZE};

// TODO: Doc Comment
pub struct Memory {
    ram: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
}

impl Memory {
    // TODO: Doc Comment
    pub fn new() -> Self {
        let mut mem = Self {
            ram: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
        };
        mem.load_fontset();
        mem
    }

    // TODO: Doc Comment
    pub fn reset(&mut self) {
        let mut mem = Self {
            ram: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
        };
        mem.load_fontset();
    }

    // TODO: Doc Comment
    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    // TODO: Doc Comment
    pub fn write_bytes(&mut self, start: u16, data: &[u8]) {
        self.ram[start as usize..start as usize + data.len()].copy_from_slice(data);
    }

    // TODO: Doc Comment
    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    // TODO: Doc Comment
    pub fn read_bytes(&self, start: u16, end: u16) -> Vec<u8> {
        self.ram[start as usize..end as usize].to_vec()
    }

    // TODO: Doc Comment
    // Make sure to increment sp after calling!
    pub fn push_stack(&mut self, sp: usize, value: u16) {
        if sp < self.stack.len() {
            self.stack[sp] = value;
        } else {
            panic!("Stack overflow!");
        }
    }

    // TODO: Doc Comment
    // Make sure to decrement sp before calling!
    pub fn pop_stack(&self, sp: usize) -> u16 {
        if sp > 0 && sp <= self.stack.len() {
            self.stack[sp]
        } else {
            panic!("Stack underflow"); // TODO: Actual error handling.
        }
    }

    // Helper function to load fontset into ram.
    fn load_fontset(&mut self) {
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
}
