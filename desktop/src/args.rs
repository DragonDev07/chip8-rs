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

    /// Number of CPU steps per frame (overrides CPU frequency if set)
    #[arg(short = 's', long, value_name = "STEPS")]
    pub steps_per_frame: Option<usize>,

    /// CPU frequency in Hz
    #[arg(short = 'f', long, value_name = "HZ", default_value_t = 600)]
    pub cpu_frequency: u32,
}
