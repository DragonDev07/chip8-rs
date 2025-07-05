use crate::{
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, PROGRAM_START},
    cpu::Cpu,
    display::Display,
    input::Keypad,
    memory::Memory,
};

// TODO: Doc Comment
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keypad: Keypad,
}

impl Emulator {
    // TODO: Doc Comment
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    // TODO: Doc Comment
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.display.reset();
        self.keypad.reset();
    }

    // TODO: Doc Comment
    pub fn cycle(&mut self) {
        self.cpu
            .cycle(&mut self.memory, &mut self.display, &mut self.keypad);
    }

    // TODO: Doc Comment
    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory.write_bytes(PROGRAM_START, data);
    }

    // TODO: Doc Comment
    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers();
    }

    // TODO: Doc Comment
    pub fn get_st(&mut self) -> u8 {
        self.cpu.get_st()
    }

    // TODO: Doc Comment
    pub fn get_display_buffer(&mut self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        self.display.get_buffer()
    }

    // TODO: Doc Comment
    pub fn press_key(&mut self, idx: usize) {
        self.keypad.press_key(idx);
    }

    // TODO: Doc Comment
    pub fn release_key(&mut self, idx: usize) {
        self.keypad.release_key(idx);
    }
}
