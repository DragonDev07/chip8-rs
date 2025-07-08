use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum MemoryError {
    #[error("Memory write out of bounds at address {addr:#06X}")]
    OutOfBoundsWrite { addr: u16 },

    #[error("Memory write out of bounds: start={start:#06X}, len={len:#06X}")]
    OutOfBoundsWriteRange { start: u16, len: usize },

    #[error("Memory read out of bounds at address {addr:#06X}")]
    OutOfBoundsRead { addr: u16 },

    #[error("Memory read out of bounds: start={start:#06X}, end={end:#06X}")]
    OutOfBoundsReadRange { start: u16, end: u16 },

    #[error("Stack overflow at stack pointer {sp}")]
    #[diagnostic(code(emulator::memory::stack_overflow))]
    StackOverflow { sp: usize },

    #[error("Stack underflow at stack pointer {sp}")]
    #[diagnostic(code(emulator::memory::stack_underflow))]
    StackUnderflow { sp: usize },
}

#[derive(Debug, Error, Diagnostic)]
pub enum KeypadError {
    #[error("Keypad index out of bounds: {idx}")]
    OutOfBoundsKeyIndex { idx: usize },
}

#[derive(Debug, Error, Diagnostic)]
pub enum CpuError {
    #[error("Memory error")]
    #[diagnostic(transparent)]
    Memory {
        #[from]
        #[source]
        source: MemoryError,
    },

    #[error("Keypad error")]
    #[diagnostic(transparent)]
    Keypad {
        #[from]
        #[source]
        source: KeypadError,
    },

    #[error("Unimplemented opcode: {opcode:#06X}")]
    #[diagnostic(code(emulator::cpu::unimplemented_opcode))]
    UnimplementedOpcode { opcode: u16 },
}

#[derive(Debug, Error, Diagnostic)]
pub enum EmulatorError {
    #[error("Memory error")]
    #[diagnostic(transparent)]
    Memory {
        #[from]
        #[source]
        source: MemoryError,
    },

    #[error("Keypad error")]
    #[diagnostic(transparent)]
    Keypad {
        #[from]
        #[source]
        source: KeypadError,
    },

    #[error("CPU execution error")]
    #[diagnostic(transparent)]
    Cpu {
        #[from]
        #[source]
        source: CpuError,
    },
}
