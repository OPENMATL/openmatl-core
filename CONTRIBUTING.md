# Contributing to `openmat-core`

Welcome! If you are here, you are a Core Systems Engineer, Compiler Architect, or Rust Developer looking to optimize the absolute bleeding-edge of Machine Learning compilation.

*(If you just want to add a math formula or deep learning layer using `.om` scripts, please submit those Pull Requests to the [openmat-lang/openmat](https://github.com/openmat-lang/openmat) repository instead!)*

## 🚀 Future Scope (What we expect next!)
We are looking for Rust developers to tackle the following Core Engine upgrades:
1. **GPU Acceleration (CUDA/Metal)**: We need contributors to bind our `NdArray` Apache Arrow buffers to `wgpu` or `CUDA` kernels so we can offload Matrix Multiplications from the CPU to the GPU.
2. **Just-In-Time Compilation (JIT)**: Currently, we execute the AST natively through an interpreter loop (`engine.execute_statement`). We want to introduce a Cranelift or LLVM backend to JIT compile `.om` scripts.
3. **WebAssembly Optimizations**: Improve the `wasm-pack` compilation pipeline inside the `/web` folder to allow multi-threaded WASM execution in the browser.

## ⚠️ Restrictions & Guidelines
1. **Zero Python Constraints**: The core engine must remain purely written in Rust. Do not introduce C++ or Python into `/compiler`. 
2. **Memory Safety First**: Do not use `unsafe` blocks unless absolutely necessary for FFI bindings (like PyO3).
3. **Testing**: Before submitting a PR modifying the AST or execution loop, you must ensure `cargo test` passes successfully.

## 🛠️ How to Contribute
1. Fork this repository.
2. Modify the `engine.rs` or `openmat.pest` files inside `/compiler/src`.
3. Test your changes using `cargo run -- run test.om`.
4. Submit a Pull Request!
