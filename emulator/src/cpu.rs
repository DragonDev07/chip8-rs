use log::info;

use crate::{
    constants::{NUM_KEYS, NUM_REGS, PROGRAM_START},
    display::Display,
    error::CpuError,
    input::Keypad,
    memory::Memory,
};

// Holds the state of the CHIP-8 CPU, including registers, timers, and pointers.
pub struct Cpu {
    pc: u16,               // Program Counter
    sp: u16,               // Stack Pointer
    v_reg: [u8; NUM_REGS], // V Registers (V0 - VF)
    i_reg: u16,            // I Register (Used for indexing into RAM)
    dt: u8,                // Delay Timer
    st: u8,                // Sound Timer
}

impl Cpu {
    // Create a new CPU with registers and pointers initialized.
    pub fn new() -> Self {
        Self {
            pc: PROGRAM_START,
            sp: 0,
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            dt: 0,
            st: 0,
        }
    }

    // Reset CPU state to initial values.
    pub fn reset(&mut self) {
        self.pc = PROGRAM_START;
        self.sp = 0;
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.dt = 0;
        self.st = 0;
        info!(
            "CPU reset (PC set to {:#05X}, registers cleared).",
            PROGRAM_START
        );
    }

    // Perform a single CPU cycle: fetch, decode, and execute one opcode.
    pub fn cycle(
        &mut self,
        memory: &mut Memory,
        display: &mut Display,
        keypad: &mut Keypad,
    ) -> Result<(), CpuError> {
        let op = (memory.read_byte(self.pc)? as u16) << 8 | memory.read_byte(self.pc + 1)? as u16;
        self.pc += 2;

        // Execute opcode
        self.execute(op, memory, display, keypad)
    }

    // Decrement the delay and sound timers if they are not zero.
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1
        }
        if self.st > 0 {
            self.st -= 1
        }
    }

    // Get the current value of the sound timer.
    pub fn get_st(&self) -> u8 {
        self.st
    }

    // Decode and execute a single opcode.
    fn execute(
        &mut self,
        op: u16,
        memory: &mut Memory,
        display: &mut Display,
        keypad: &mut Keypad,
    ) -> Result<(), CpuError> {
        let n1 = (op & 0xF000) >> 12;
        let n2 = (op & 0x0F00) >> 8;
        let n3 = (op & 0x00F0) >> 4;
        let n4 = op & 0x000F;
        let nn = op & 0x00FF;
        let nnn = op & 0x0FFF;

        match (n1, n2, n3, n4) {
            // NOP
            (0, 0, 0, 0) => {}

            // 00E0 -> Clear screen.
            (0, 0, 0xE, 0) => display.clear(),

            // 00EE -> Return from a subroutine (returns to PC on stack).
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = memory.pop_stack(self.sp as usize)?;
            }

            // 1NNN -> Jump to address NNN.
            (1, _, _, _) => self.pc = nnn,

            // 2NNN -> Execute subroutine starting at address NNN.
            (2, _, _, _) => {
                memory.push_stack(self.sp as usize, self.pc)?;
                self.sp += 1;
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

            // 6XNN -> Store number NN in VX.
            (6, _, _, _) => self.v_reg[n2 as usize] = nn as u8,

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
                let x = self.v_reg[n2 as usize] as usize;
                let y = self.v_reg[n3 as usize] as usize;
                let sprite = memory.read_bytes(self.i_reg, self.i_reg + n4);
                let flipped = display.draw_sprite(x, y, &sprite?);

                // Populate VF register based on whether any pixels were flipped from "on" to "off".
                self.v_reg[0xF] = flipped as u8;
            }

            // EX9E -> Skip next instruction if key specified in VX is pressed.
            (0xE, _, 9, 0xE) => {
                if keypad.is_pressed(self.v_reg[n2 as usize] as usize)? {
                    self.pc += 2;
                }
            }

            // EXA1 -> Skip next instruction if key specified in VX is NOT pressed.
            (0xE, _, 0xA, 1) => {
                if !keypad.is_pressed(self.v_reg[n2 as usize] as usize)? {
                    self.pc += 2;
                }
            }

            // FX07 -> Store current value of delay timer in VX.
            (0xF, _, 0, 7) => self.v_reg[n2 as usize] = self.dt,

            // FX0A -> Wait until any key is pressed, then store the Key into VX.
            (0xF, _, 0, 0xA) => {
                let mut pressed = false;

                for i in 0..NUM_KEYS {
                    if keypad.is_pressed(i)? {
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
                self.i_reg = self.v_reg[n2 as usize] as u16 * 5;
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
                memory.write_byte(self.i_reg, hundreds)?;
                memory.write_byte(self.i_reg + 1, tens)?;
                memory.write_byte(self.i_reg + 2, ones)?;
            }

            // FX55 -> Store values from V0 - VX in memory starting at address specified by I.
            (0xF, _, 5, 5) => {
                let x = n2 as usize;

                // Loop from V0 up to and including VX
                for i in 0..=x {
                    // Store the value of Vi into RAM at address I
                    memory.write_byte(self.i_reg, self.v_reg[i])?;

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
                    self.v_reg[i] = memory.read_byte(self.i_reg)?;

                    // Increment I register for the next memory address.
                    self.i_reg += 1;
                }
            }

            _ => return Err(CpuError::UnimplementedOpcode { opcode: op }),
        }
        Ok(())
    }
}
