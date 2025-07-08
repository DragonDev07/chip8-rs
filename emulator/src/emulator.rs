use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, PROGRAM_START},
    cpu::Cpu,
    display::Display,
    error::EmulatorError,
    input::Keypad,
    memory::Memory,
};

// Holds the main components of the CHIP-8 emulator (CPU, memory, display, keypad).
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keypad: Keypad,
}

impl Emulator {
    // Create a new emulator with all components initialized.
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    // Reset all emulator components to their initial state.
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.keypad.reset();
        self.display.clear();
    }

    // Perform a single CPU cycle (fetch, decode, execute).
    pub fn cycle(&mut self) -> Result<(), EmulatorError> {
        self.cpu
            .cycle(&mut self.memory, &mut self.display, &mut self.keypad)
            .map_err(|result| EmulatorError::Cpu { source: result })
    }

    // Load a ROM into memory starting at the program start address.
    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), EmulatorError> {
        self.memory
            .write_bytes(PROGRAM_START, data)
            .map_err(|result| EmulatorError::Memory { source: result })
    }

    // Tick (decrement) the CPU timers.
    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers();
    }

    // Get the current value of the sound timer.
    pub fn get_st(&mut self) -> u8 {
        self.cpu.get_st()
    }

    // Get a reference to the display buffer.
    pub fn get_display_buffer(&mut self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        self.display.get_buffer()
    }

    // Mark the key at the given index as pressed (true).
    pub fn press_key(&mut self, idx: usize) -> Result<(), EmulatorError> {
        self.keypad
            .press_key(idx)
            .map_err(|result| EmulatorError::Keypad { source: result })
    }

    // Mark the key at the given index as released (false).
    pub fn release_key(&mut self, idx: usize) -> Result<(), EmulatorError> {
        self.keypad
            .release_key(idx)
            .map_err(|result| EmulatorError::Keypad { source: result })
    }
}
