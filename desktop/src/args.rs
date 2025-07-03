use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Teo Welton",
    version = "0.0.1",
    about = "A simple CHIP-8 emulator written in Rust."
)]
pub struct Args {
    /// Path to the ROM file to be loaded.
    pub rom_path: String,
}
