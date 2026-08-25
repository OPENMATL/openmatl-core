# Contributing to OpenMat

Thank you for investing your time in contributing to OpenMat!

## 1. Development Environment
You will need the following toolchains installed:
- **Rust Toolchain**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **WASM Pack**: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
- **Python 3.9+**: For testing the `PyO3` machine learning bindings.

## 2. Building from Source
To compile the core engine locally on your OS (Mac/Linux/Windows):
```bash
cargo build --release
```
To install your locally modified binary to your global terminal path:
```bash
cargo install --path . --force
```

## 3. WebAssembly IDE
If you are contributing to the web frontend (`www/index.html`):
```bash
# Build the WASM architecture package
wasm-pack build --target web

# Run the local python web server
python3 -m http.server 8080
```

## 4. Submitting Pull Requests
- Please ensure `cargo clippy` and `cargo test` pass successfully.
- If modifying `openmat.pest` to introduce new grammar tokens, please document the new tokens in `LANGUAGE_SPEC.md`.
