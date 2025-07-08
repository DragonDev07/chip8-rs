use log::{info, warn};

use crate::{constants::NUM_KEYS, error::KeypadError};

// Holds the state of the CHIP-8 keypad (16 keys).
// CHIP-8    QWERTY
// 1 2 3 C   1 2 3 4
// 4 5 6 D   Q W E R
// 7 8 9 E   A S D F
// A 0 B F   Z X C V
pub struct Keypad {
    keys: [bool; NUM_KEYS],
}

impl Keypad {
    // Create a new keypad with all keys in the released state.
    pub fn new() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }

    // Reset all keys to released state.
    pub fn reset(&mut self) {
        self.keys = [false; NUM_KEYS];
        info!("Keypad reset (all keys released).")
    }

    // Check if the key at the given index is pressed and return its state.
    pub fn is_pressed(&self, idx: usize) -> Result<bool, KeypadError> {
        if idx < NUM_KEYS {
            Ok(self.keys[idx])
        } else {
            warn!("Attempted to check out-of-bounds key: {}", idx);
            Err(KeypadError::OutOfBoundsKeyIndex { idx })
        }
    }

    // Mark the key at the given index as pressed (true).
    pub fn press_key(&mut self, idx: usize) -> Result<(), KeypadError> {
        if idx < NUM_KEYS {
            self.keys[idx] = true;
            Ok(())
        } else {
            warn!("Attempted to set out-of-bounds key as pressed: {}", idx);
            Err(KeypadError::OutOfBoundsKeyIndex { idx })
        }
    }

    // Mark the key at the given index as released (false).
    pub fn release_key(&mut self, idx: usize) -> Result<(), KeypadError> {
        if idx < NUM_KEYS {
            self.keys[idx] = false;
            Ok(())
        } else {
            warn!("Attempted to set out-of-bounds key as released: {}", idx);
            Err(KeypadError::OutOfBoundsKeyIndex { idx })
        }
    }
}
