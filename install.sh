#!/bin/sh

set -e

BINARY_NAME="chip8-emu"
ROM_PATH="test_roms/Chip8 Emulator Logo"
INSTALL_PATH="/usr/local/bin"

# Detect OS
OS="$(uname)"
echo "Detected OS: $OS"

if [ "$OS" = "Darwin" ] || [ "$OS" = "Linux" ]; then
    # Ask user about installing to /usr/local/bin
    echo "Would you like to install the emulator binary to $INSTALL_PATH? (y/n)"
    if [ -t 0 ]; then
        read -r INSTALL_CHOICE
    else
        # If not running in a terminal, read from /dev/tty
        read -r INSTALL_CHOICE < /dev/tty
    fi

    # Build the project in release mode
    echo "Building the project in release mode..."
    cargo build --release

    if [ "$INSTALL_CHOICE" = "y" ] || [ "$INSTALL_CHOICE" = "Y" ]; then
        # Copy the binary to /usr/local/bin
        echo "Installing $BINARY_NAME to $INSTALL_PATH (may require sudo)..."
        sudo cp "target/release/$BINARY_NAME" "$INSTALL_PATH/"
        echo "Installed $BINARY_NAME to $INSTALL_PATH"
        echo "Running test ROM: $ROM_PATH"
        "$INSTALL_PATH/$BINARY_NAME" "$ROM_PATH"
    else
        echo "Skipping installation to $INSTALL_PATH."
        echo "Running test ROM: $ROM_PATH"
        "./target/release/$BINARY_NAME" "$ROM_PATH"
    fi
else
    echo "Non-macOS/Linux OS detected. Skipping installation step."
    echo "Building the project in release mode..."
    cargo build --release
    echo "Running test ROM: $ROM_PATH"
    "./target/release/$BINARY_NAME" "$ROM_PATH"
fi
