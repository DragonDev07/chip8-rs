use log::{debug, warn};

use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

// Holds the display buffer for the CHIP-8 emulator as a 2D array.
pub struct Display {
    display_buffer: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // Screen as 2D array.
}

impl Display {
    // Create a new display with all pixels turned off.
    pub fn new() -> Self {
        Self {
            display_buffer: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    // Clear the display (set all pixels to off).
    pub fn clear(&mut self) {
        self.display_buffer = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        debug!("Display reset / cleared.")
    }

    // Get a reference to the display buffer.
    pub fn get_buffer(&self) -> &[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        &self.display_buffer
    }

    // Draw a sprite at (x, y) onto the display buffer.
    // Returns true if any pixels were flipped from set to unset (collision).
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        // Handle sprite possibly being empty.
        if sprite.is_empty() {
            warn!("Attempted to draw empty sprite at ({}, {})", x, y);
            return false;
        }

        // Handle sprite possibly being too large for CHIP-8.
        if sprite.len() > 15 {
            warn!(
                "Sprite length {} exceeds CHIP-8 max at ({}, {})",
                sprite.len(),
                x,
                y
            );
        }

        let mut flipped = false;
        let height = sprite.len();

        // Wrap starting coordinates.
        let x_coord = x % DISPLAY_WIDTH;
        let y_coord = y % DISPLAY_HEIGHT;

        // Iterate over each row of the sprite.
        for y_offset in 0..height {
            // Calculate the actual Y coordinate on the screen for the current row.
            let screen_y = y_coord + y_offset;

            // If the current sprite row would be drawn off the bottom edge of the screen,
            // stop drawing the rest of the sprite.
            if screen_y >= DISPLAY_HEIGHT {
                break;
            }

            // Get the pixels for the current row of the sprite.
            let byte = sprite[y_offset];

            // Iterate over each column (bit) within the current sprite row (8 pixels wide).
            for x_offset in 0..8 {
                // Calculate the actual X coordinate on the screen for the current pixel.
                let screen_x = x_coord + x_offset;

                // If the current sprite pixel would be drawn off the right edge of the screen,
                // stop drawing the rest of this row.
                if screen_x >= DISPLAY_WIDTH {
                    break;
                }

                // Check if the current pixel in the sprite is "on", and flip if it is.
                if (byte & (0b1000_0000 >> x_offset)) != 0 {
                    // Calculate the actual screen coordinates of the pixel, applying wrapping.
                    let screen_y = (y_coord + y_offset) % DISPLAY_HEIGHT;
                    let screen_x = (x_coord + x_offset) % DISPLAY_WIDTH;

                    // Check if the target pixel is currently "on" (true).
                    if self.display_buffer[screen_y][screen_x] {
                        flipped = true; // A pixel was turned "off" (flipped from true to false).
                    }

                    // XOR the pixel: true if it was false, false if it was true.
                    self.display_buffer[screen_y][screen_x] ^= true;
                }
            }
        }
        flipped
    }
}
