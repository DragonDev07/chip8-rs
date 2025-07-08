use log::{debug, info};

use crate::{
    constants::{FONTSET, FONTSET_SIZE, MEMORY_SIZE, STACK_SIZE},
    error::MemoryError,
};

// Holds RAM and stack for the CHIP-8 emulator.
pub struct Memory {
    ram: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
}

impl Memory {
    // Initialize memory and stack, and load the fontset.
    pub fn new() -> Self {
        let mut mem = Self {
            ram: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
        };
        mem.load_fontset();
        mem
    }

    // Reset memory and stack to initial state, reload fontset.
    pub fn reset(&mut self) {
        self.ram = [0; MEMORY_SIZE];
        self.stack = [0; STACK_SIZE];
        self.load_fontset();
        info!("Memory reset and fontset reloaded.")
    }

    // Write a single byte to RAM at the given address.
    pub fn write_byte(&mut self, addr: u16, value: u8) -> Result<(), MemoryError> {
        if (addr as usize) < MEMORY_SIZE {
            self.ram[addr as usize] = value;
            Ok(())
        } else {
            Err(MemoryError::OutOfBoundsWrite { addr })
        }
    }

    // Write a slice of bytes to RAM starting at the given address.
    pub fn write_bytes(&mut self, start: u16, data: &[u8]) -> Result<(), MemoryError> {
        if start as usize + data.len() <= MEMORY_SIZE {
            self.ram[start as usize..start as usize + data.len()].copy_from_slice(data);
            Ok(())
        } else {
            Err(MemoryError::OutOfBoundsWriteRange {
                start: start,
                len: data.len(),
            })
        }
    }

    // Read a single byte from RAM at the given address.
    pub fn read_byte(&mut self, addr: u16) -> Result<u8, MemoryError> {
        if (addr as usize) < MEMORY_SIZE {
            Ok(self.ram[addr as usize])
        } else {
            Err(MemoryError::OutOfBoundsRead { addr })
        }
    }

    // Read a range of bytes from RAM (start inclusive, end exclusive).
    pub fn read_bytes(&self, start: u16, end: u16) -> Result<Vec<u8>, MemoryError> {
        let start = start as usize;
        let end = end as usize;
        if start <= end && end <= MEMORY_SIZE {
            Ok(self.ram[start..end].to_vec())
        } else {
            Err(MemoryError::OutOfBoundsReadRange {
                start: start as u16,
                end: end as u16,
            })
        }
    }

    // Push a value onto the stack at the given stack pointer index.
    // Caller should increment sp **after** calling.
    pub fn push_stack(&mut self, sp: usize, value: u16) -> Result<(), MemoryError> {
        if sp < self.stack.len() {
            self.stack[sp] = value;
            Ok(())
        } else {
            Err(MemoryError::StackOverflow { sp })
        }
    }

    // Pop a value from the stack at the given stack pointer index.
    // Caller should decrement sp before calling.
    pub fn pop_stack(&mut self, sp: usize) -> Result<u16, MemoryError> {
        if sp < self.stack.len() {
            Ok(self.stack[sp])
        } else {
            Err(MemoryError::StackUnderflow { sp })
        }
    }

    // Helper function to load the CHIP-8 fontset into the beginning of RAM.
    fn load_fontset(&mut self) {
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        debug!("Fontset loaded into memory.");
    }
}
