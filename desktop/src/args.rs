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

    /// CPU frequency in Hz (default: 500)
    #[arg(short = 'f', long, value_name = "HZ", default_value_t = 500)]
    pub cpu_frequency: u32,

    /// Display refresh frequency in Hz (default: 60)
    #[arg(short = 'd', long, value_name = "HZ", default_value_t = 60)]
    pub display_frequency: u32,

    /// Timer frequency in Hz (default: 60)
    #[arg(short = 't', long, value_name = "HZ", default_value_t = 60)]
    pub timer_frequency: u32,

    /// Number of CPU steps per frame (overrides calculated value using CPU and display frequency)
    #[arg(short = 's', long, value_name = "STEPS")]
    pub steps_per_frame: Option<usize>,
}
