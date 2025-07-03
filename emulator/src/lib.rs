pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096; // 4 KB
const START_ADDR: u16 = 0x200;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const NUM_REGS: usize = 16;
const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pc: u16, // Program Counter
    ram: [u8; RAM_SIZE],
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT], // Screen as 2D array
    stack: [u16; STACK_SIZE],
    dt: u8, // Delay Timer
    st: u8, // Sound Timer
    keys: [bool; NUM_KEYS],
    v_reg: [u8; NUM_REGS], // V Registers (V0 - VF)
    i_reg: u16,            // I Register (Used for indexing into RAM)
    sp: u16,               // Stack Pointer
}

impl Chip8 {
    // Function to initialize a new emulator.
    pub fn new() -> Self {
        let mut emu = Self {
            pc: 0x200,
            ram: [0; RAM_SIZE],
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,
            keys: [false; NUM_KEYS],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
        };

        // Load the fontset into RAM.
        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        return emu;
    }

    // Function to reset the emulator.
    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
        self.stack = [0; STACK_SIZE];
        self.dt = 0;
        self.st = 0;
        self.keys = [false; NUM_KEYS];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;

        // Load fontset into RAM.
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    // Function to tick emulation.
    pub fn tick(&mut self) {
        // Fetch next opcode
        let op = (self.ram[self.pc as usize] as u16) << 8 | self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        // Execute opcode
        self.execute(op);
    }

    // Function to tick timers.
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            // BEEP
            self.st -= 1;
        }
    }

    // Function to get the display state.
    pub fn get_display(&self) -> [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        return self.screen;
    }

    pub fn get_sound_timer(&self) -> u8 {
        return self.st;
    }

    // Function to be able to update keypresses.
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx as usize] = pressed;
    }

    // Function to load ROMs
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    // Function to execute opcode.
    fn execute(&mut self, op: u16) {
        let n1 = (op & 0xF000) >> 12;
        let n2 = (op & 0x0F00) >> 8;
        let n3 = (op & 0x00F0) >> 4;
        let n4 = op & 0x000F;
        let nn = op & 0x00FF;
        let nnn = op & 0x0FFF;

        match (n1, n2, n3, n4) {
            (0, 0, 0, 0) => return,                                                 // NOP
            (0, 0, 0xE, 0) => self.screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT], // 00E0 -> Clear screen.
            (0, 0, 0xE, 0xE) => self.pc = self.pop(), // 00EE -> Return from a subroutine (returns to PC on stack).
            (1, _, _, _) => self.pc = nnn,            // 1NNN -> Jump to address NNN.

            // 2NNN -> Execute subroutine starting at address NNN.
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = nnn;
            }

            // 3XNN -> Skip next instruction if VX = NN.
            (3, _, _, _) => {
                if self.v_reg[n2 as usize] == nn as u8 {
                    self.pc += 2
                }
            }

            // 4XNN -> Skip next instruction if VX != NN.
            (4, _, _, _) => {
                if self.v_reg[n2 as usize] != nn as u8 {
                    self.pc += 2
                }
            }

            // 5XY0 -> Skip next instruction if value in VX == VY.
            (5, _, _, 0) => {
                if self.v_reg[n2 as usize] == self.v_reg[n3 as usize] {
                    self.pc += 2
                }
            }

            (6, _, _, _) => self.v_reg[n2 as usize] = nn as u8, // 6XNN -> Store number NN in VX.

            // 7XNN -> Add value NN to VX.
            (7, _, _, _) => {
                self.v_reg[n2 as usize] = self.v_reg[n2 as usize].wrapping_add(nn as u8)
            }

            (8, _, _, 0) => self.v_reg[n2 as usize] = self.v_reg[n3 as usize], // 8XY0 -> Set VX to the value of VY.
            (8, _, _, 1) => self.v_reg[n2 as usize] |= self.v_reg[n3 as usize], // 8XY1 -> Set VX to VX bitwise OR VY.
            (8, _, _, 2) => self.v_reg[n2 as usize] &= self.v_reg[n3 as usize], // 8XY2 -> Set VX to VX bitwise AND VY.
            (8, _, _, 3) => self.v_reg[n2 as usize] ^= self.v_reg[n3 as usize], // 8XY3 -> Set VX to VX bitwise XOR VY.

            // 8XY4 -> Set VX to VX + VY, set VF to carry.
            (8, _, _, 4) => {
                let x = n2 as usize;
                let y = n3 as usize;
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = carry as u8;
            }

            // 8XY5 -> Set VX to VX - VY, set VF to borrow.
            (8, _, _, 5) => {
                let x = n2 as usize;
                let y = n3 as usize;
                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = !borrow as u8;
            }

            // 8XY6 -> Store VY bitwise shifted right one into VX, set VF to least significant bit prior to shift.
            // This is the original CHIP-8 implementation. CHIP-48 & SUPER-CHIP both expect just a simple shift on VX.
            (8, _, _, 6) => {
                let x = n2 as usize;
                let y = n3 as usize;
                let lsb = self.v_reg[y] & 1;
                self.v_reg[x] = self.v_reg[y] >> 1;
                self.v_reg[0xF] = lsb;
            }

            // 8XY7 -> Set VX to VY - VX, set VF to borrow. (Same as 8XY5 but with opposite operands)
            (8, _, _, 7) => {
                let x = n2 as usize;
                let y = n3 as usize;
                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = !borrow as u8;
            }

            // 8XYE -> Store VY bitwise shifted left one into VX, set VF to most significant bit prior to shift.
            // See above note for opcode 8XY6.
            (8, _, _, 0xE) => {
                let x = n2 as usize;
                let y = n3 as usize;
                let msb = (self.v_reg[y] >> 7) & 1;
                self.v_reg[x] = self.v_reg[y] << 1;
                self.v_reg[0xF] = msb;
            }

            // 9XY0 -> Skip next instruction if value in VX != VY.
            (9, _, _, 0) => {
                if self.v_reg[n2 as usize] != self.v_reg[n3 as usize] {
                    self.pc += 2
                }
            }

            (0xA, _, _, _) => self.i_reg = nnn, // ANNN -> Store address NNN in I.
            (0xB, _, _, _) => self.pc = nnn + self.v_reg[0] as u16, // BNNN -> Jump to address NNN + V0.

            // CXNN -> Set VX to a random number bitwise AND NN.
            (0xC, _, _, _) => {
                let random = rand::random::<u8>();
                self.v_reg[n2 as usize] = random & nn as u8;
            }

            // DXYN -> Draw sprite at position (VX, VY) with N bytes of sprite data starting at address stored in I.
            //         Set VF to 01 if pixels are changed to "off" otherwise, set VF to 00.
            //         Ensure that the sprite itself doesn't wrap but the position of the sprite does.
            (0xD, _, _, _) => {
                // Get (x, y) coords for sprite from registers.
                // We want the starting position of the sprite to wrap, so we apply modulo/AND here.
                let x_coord = (self.v_reg[n2 as usize] as usize) % SCREEN_WIDTH;
                let y_coord = (self.v_reg[n3 as usize] as usize) % SCREEN_HEIGHT;

                // Get rows from the 4th nibble.
                let rows = n4 as usize;

                // Variable to track if any pixels were flipped (turned "off"), indicating a collision.
                let mut flipped = false;

                // Iterate over each row of the sprite.
                for y_offset in 0..rows {
                    // Calculate the actual Y coordinate on the screen for the current row.
                    let screen_y = y_coord + y_offset;

                    // If the current sprite row would be drawn off the bottom edge of the screen,
                    // stop drawing the rest of the sprite.
                    if screen_y >= SCREEN_HEIGHT {
                        break;
                    }

                    // Determine memory address for current row of sprite data.
                    let addr = self.i_reg + y_offset as u16;

                    // Get the row's pixel data from memory.
                    let pixels = self.ram[addr as usize];

                    // Iterate over each column (bit) within the current sprite row (8 pixels wide).
                    for x_offset in 0..8 {
                        // Calculate the actual X coordinate on the screen for the current pixel.
                        let screen_x = x_coord + x_offset;

                        // If the current sprite pixel would be drawn off the right edge of the screen,
                        // stop drawing the rest of this row.
                        if screen_x >= SCREEN_WIDTH {
                            break;
                        }

                        // Check if the current pixel in the sprite is "on", and flip if it is.
                        if (pixels & (0b1000_0000 >> x_offset)) != 0 {
                            // Calculate the actual screen coordinates of the pixel, applying wrapping.
                            let screen_x = (x_coord + x_offset) % SCREEN_WIDTH;
                            let screen_y = (y_coord + y_offset) % SCREEN_HEIGHT;

                            // Check if the target pixel is currently "on" (true).
                            if self.screen[screen_y][screen_x] {
                                flipped = true; // A pixel was turned "off" (flipped from true to false).
                            }

                            // XOR the pixel: true if it was false, false if it was true.
                            self.screen[screen_y][screen_x] ^= true;
                        }
                    }
                }

                // Populate VF register based on whether any pixels were flipped from "on" to "off".
                self.v_reg[0xF] = flipped as u8;
            }

            // EX9E -> Skip next instruction if key specified in VX is pressed.
            (0xE, _, 9, 0xE) => {
                if self.keys[self.v_reg[n2 as usize] as usize] {
                    self.pc += 2;
                }
            }

            // EXA1 -> Skip next instruction if key specified in VX is NOT pressed.
            (0xE, _, 0xA, 1) => {
                if !self.keys[self.v_reg[n2 as usize] as usize] {
                    self.pc += 2;
                }
            }

            // FX07 -> Store current value of delay timer in VX.
            (0xF, _, 0, 7) => self.v_reg[n2 as usize] = self.dt,

            // FX0A -> Wait until any key is pressed, then store the Key into VX.
            (0xF, _, 0, 0xA) => {
                let mut pressed = false;
                for i in 0..NUM_KEYS {
                    if self.keys[i] {
                        self.v_reg[n2 as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }

            (0xF, _, 1, 5) => self.dt = self.v_reg[n2 as usize], // FX15 -> Set the delay timer to the value in VX.
            (0xF, _, 1, 8) => self.st = self.v_reg[n2 as usize], // FX18 -> Set the sound timer to the value in VX.

            // FX1E -> Add value from VX to I register.
            (0xF, _, 1, 0xE) => {
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[n2 as usize] as u16)
            }

            // FX29 -> Set I to the location of the sprite for the hexadecimal digit stored in VX.
            (0xF, _, 2, 9) => {
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[n2 as usize] as u16 * 5);
            }

            // FX33 -> Store the BCD (Binary Coded Decimal) equivalent of value in VX to
            //         address specified by I, I + 1, & I + 2.
            (0xF, _, 3, 3) => {
                // Get value in VX.
                let vx = self.v_reg[n2 as usize] as f32;

                // Fetch the hundreds digit by dividing by 100 and tossing the decimal.
                let hundreds = (vx / 100.0).floor() as u8;

                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal.
                let tens = ((vx / 10.0) % 10.0).floor() as u8;

                // Fetch the ones digit by tossing the hundreds and the tens.
                let ones = (vx % 10.0) as u8;

                // Store the BCD digits in memory.
                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            }

            // FX55 -> Store values from V0 - VX in memory starting at address specified by I.
            (0xF, _, 5, 5) => {
                let x = n2 as usize;

                // Loop from V0 up to and including VX
                for i in 0..=x {
                    // Store the value of Vi into RAM at address I
                    self.ram[self.i_reg as usize] = self.v_reg[i];

                    // Increment I register for the next memory address.
                    self.i_reg += 1;
                }
            }

            // FX65 -> Load V0 - VX from memory starting at address specified by I.
            (0xF, _, 6, 5) => {
                let x = n2 as usize;

                // Loop from V0 up to and including VX
                for i in 0..=x {
                    // Load the value from RAM at address I into register Vi
                    self.v_reg[i] = self.ram[self.i_reg as usize];

                    // Increment I register for the next memory address.
                    self.i_reg += 1;
                }
            }

            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#X}", op),
        }
    }

    // Helper function to push a value onto the stack.
    fn push(&mut self, value: u16) {
        if (self.sp as usize) < STACK_SIZE {
            self.stack[self.sp as usize] = value;
            self.sp += 1;
        } else {
            panic!("Stack overflow"); // TODO: Probably bad practice but who cares!
        }
    }

    // Helper function to pop a value off of the stack
    fn pop(&mut self) -> u16 {
        if self.sp > 0 {
            self.sp -= 1;
            self.stack[self.sp as usize]
        } else {
            panic!("Stack underflow"); // TODO: Probably bad practice but who cares!
        }
    }
}
