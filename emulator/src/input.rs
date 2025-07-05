use crate::constants::NUM_KEYS;

// TODO: Doc Comment
pub struct Keypad {
    keys: [bool; NUM_KEYS],
}

impl Keypad {
    // TODO: Doc Comment
    pub fn new() -> Self {
        Self {
            keys: [false; NUM_KEYS],
        }
    }

    // TODO: Doc Comment
    pub fn reset(&mut self) {
        self.keys = [false; NUM_KEYS];
    }

    // TODO: Doc Comment
    pub fn is_pressed(&self, idx: usize) -> bool {
        self.keys[idx]
    }

    // TODO: Doc Comment
    pub fn press_key(&mut self, idx: usize) {
        self.keys[idx] = true;
    }

    // TODO: Doc Comment
    pub fn release_key(&mut self, idx: usize) {
        self.keys[idx] = false;
    }
}
