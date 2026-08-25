#!/bin/bash
set -e

echo "======================================"
echo "    Installing OpenMat Engine...      "
echo "======================================"

if ! command -v cargo &> /dev/null
then
    echo "[!] Rust toolchain not found."
    echo "[*] Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "[*] Compiling Engine..."
cd compiler
cargo install --path . --force

echo "======================================"
echo "[+] OpenMat installed successfully!   "
echo "[+] Run 'om' in your terminal to start"
echo "======================================"
