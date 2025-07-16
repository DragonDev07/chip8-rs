# Chip8-rs

A simple chip8 emulator written in Rust.

## Information Used

- [Awesome Chip-8](https://chip-8.github.io/links/)
- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

## Keyboard Layout
### CHIP-8 Layout
|     |     |     |     |
| --- | --- | --- | --- |
| `1` | `2` | `3` | `C` |
| `4` | `5` | `6` | `D` |
| `7` | `8` | `9` | `E` |
| `A` | `0` | `B` | `F` |

### Qwerty Layout
|     |     |     |     |
| --- | --- | --- | --- |
| `1` | `2` | `3` | `4` |
| `Q` | `W` | `E` | `R` |
| `A` | `S` | `D` | `F` |
| `Z` | `X` | `C` | `V` |

## ROMs (test_roms Directory)

The test_roms directory includes several CHIP-8 ROMs for testing and demonstration:

- **Chip8 emulator Logo.ch8** - Displays the CHIP-8 logo
- **IBM Logo.ch8** - Shows the classic IBM logo
- **Pong (1 player).ch8** - Single-player version of Pong
- **Bowling.ch8** - Simple bowling game
- **Tetris.ch8** - Tetris!

These ROMs can be run directly with the emulator to test functionality or just to enjoy some classic games.

Note: I have not documented the controls for each game, as they heavily vary, you'll just have to try them out.

## Usage / Demo
```bash
> chip8-rs --help

Usage: chip8-emu [OPTIONS] <ROM_PATH>

Arguments:
  <ROM_PATH>  Path to the ROM file to be loaded

Options:
  -s, --steps-per-frame <STEPS>  Number of CPU steps per frame (overrides CPU frequency if set)
  -f, --cpu-frequency <HZ>       CPU frequency in Hz [default: 600]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Handsfree Installation (Recommended)

Simply run the following in terminal, which will clone the repository (if needed), build the project, ask if you want to install the emulator to `/usr/local/bin`, and then run a test ROM:

```bash
curl -fsSL https://raw.githubusercontent.com/DragonDev07/chip8-rs/main/handsfree_install.sh | sh
```

The script automatically detects your operating system and will adjust accordingly if you're using macOS, Linux, or another system.

## Semi-Automatic Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/DragonDev07/chip8-rs.git
   ```

2. Navigate to the project directory:

   ```bash
   cd chip8-rs
   ```

3. Run the installation script:
   ```bash
   ./install.sh
   ```

This will build the project, ask if you want to install the emulator to `/usr/local/bin`, and then run a test ROM.

## Building from Source

If you prefer to build the project manually:

1. Clone the repository:

   ```bash
   git clone https://github.com/DragonDev07/chip8-rs.git
   ```

2. Navigate to the project directory:

   ```bash
   cd chip8-rs
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

4. Run the emulator with a ROM file:
   ```bash
   ./target/release/chip8-emu path/to/your/rom.ch8
   ```

You can also run one of the included test ROMs:

```bash
./target/release/chip8-emu test_roms/Chip8\ emulator\ Logo.ch8
```
