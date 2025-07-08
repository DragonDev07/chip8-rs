use winit::keyboard::{KeyCode, PhysicalKey};

/// Maps a physical keyboard key to a CHIP-8 keypad index.
// CHIP-8    QWERTY
// 1 2 3 C   1 2 3 4
// 4 5 6 D   Q W E R
// 7 8 9 E   A S D F
// A 0 B F   Z X C V
pub fn map_keyboard(physical_key: PhysicalKey) -> Option<usize> {
    match physical_key {
        PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
        PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
        PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
        PhysicalKey::Code(KeyCode::Digit4) => Some(0xC),
        PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
        PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
        PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
        PhysicalKey::Code(KeyCode::KeyR) => Some(0xD),
        PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
        PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
        PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
        PhysicalKey::Code(KeyCode::KeyF) => Some(0xE),
        PhysicalKey::Code(KeyCode::KeyZ) => Some(0xA),
        PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
        PhysicalKey::Code(KeyCode::KeyC) => Some(0xB),
        PhysicalKey::Code(KeyCode::KeyV) => Some(0xF),
        _ => None,
    }
}
