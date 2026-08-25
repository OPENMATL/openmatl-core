# OpenMat Engine

OpenMat is an ultra-fast, multithreaded Matrix Compute Engine and custom Programming Language completely written in Rust. It serves as a modern, open-source alternative to MATLAB, featuring zero-copy Apache Arrow buffer matrices, native Machine Learning primitive mapping via Rayon parallelization, and seamless Python & Node.js interop.

## Architecture Highlights
- **Apache Arrow Vectors**: Deeply nested matrices are flattened into pure contiguous zero-copy `Float64Array` buffers.
- **Rayon Multithreading**: Math evaluations automatically shard across available OS threads, processing 1 Million element tensors in ~0.2s.
- **Turing Complete**: The custom `.om` extension parses deep logic gates, nested conditions, loop mapping, and user-defined function scopes.
- **Autograd Ready**: The `NdArray` struct inherently supports gradient tape accumulation via `.backward()` triggers for Neural Network training.

## Installation

**From Source (Rust):**
```bash
git clone https://github.com/yourusername/openmat.git
cd openmat/openmat-core
cargo install --path . --force
```

This binds the `om` engine globally to your terminal!

## Usage (CLI)

Run the interactive REPL:
```bash
om
```

Execute an OpenMat script (`.om`):
```bash
om run my_script.om
```

## Usage (Web IDE)
OpenMat ships with a stunning native WebAssembly (WASM) frontend IDE! 
1. Build the WASM bindings: `wasm-pack build --target web`
2. Spin up a local server: `python3 -m http.server 8080`
3. Navigate to `http://localhost:8080/www/index.html` in your browser.

## Contributing
We welcome all researchers, open-source contributors, and compiler engineers! Please read our [CONTRIBUTING.md](./CONTRIBUTING.md) for how to build the project and run the Rust test-suites, and [LANGUAGE_SPEC.md](./LANGUAGE_SPEC.md) to understand how the OpenMat compiler parses text into Abstract Syntax Trees.
