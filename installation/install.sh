#!/usr/bin/env bash
set -e

REPO_URL="https://github.com/DomioKing653/FLare.git"
REPO_DIR="flarerepo"
BIN_NAME="flarec"
INSTALL_DIR="$HOME/.local/bin"

echo "Cloning repo..."
if [ ! -d "$REPO_DIR" ]; then
    git clone "$REPO_URL"
fi

cd "$REPO_DIR"

echo "Building..."
cargo build --bin "$BIN_NAME" --release

echo "Installing to $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
cp "target/release/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "Done!"
echo "Ensure that you have in PATH: $INSTALL_DIR"
