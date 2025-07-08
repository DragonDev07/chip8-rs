#!/bin/sh

REPO_URL="https://github.com/DragonDev07/chip8-rs.git"
REPO_DIR="chip8-rs"

# Clone the repository if it doesn't exist
if [ ! -d "$REPO_DIR" ]; then
    git clone "$REPO_URL"
fi

cd "$REPO_DIR" || exit 1

# Ensure install.sh is executable
chmod +x ./install.sh

# Run install.sh
./install.sh
