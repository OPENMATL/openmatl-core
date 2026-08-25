# Contributing to `openmat-core`

Welcome! If you are here, you are a Core Systems Engineer, Compiler Architect, or Rust Developer looking to optimize the absolute bleeding-edge of Machine Learning compilation.

*(If you just want to add a math formula or deep learning layer using `.om` scripts, please submit those Pull Requests to the [openmat-lang/openmat](https://github.com/openmat-lang/openmat) repository instead!)*

## 🏗️ Current Engine Architecture & Capabilities
Before contributing, it is critical to understand what the AST and Native Engine currently support:
- **Data Types:** The engine strictly operates on `Float64` N-Dimensional arrays (`NdArray<f64>`).
- **Operators:** Supports basic arithmetic (`+`, `-`, `*`, `/`) and optimized matrix dot products (`@`).
- **Control Flow:** Basic `if/else` and `return` statements are supported in the AST.
- **Concurrency:** Matrix operations execute in parallel using the `rayon` crate.

## 🚀 Future Scope (What we need Rust Engineers to build!)
To allow the open-source community to build advanced math (like Fourier Transforms) in the `openmat` language repo, we need Rust engineers to extend the core engine with the following primitives:

1. **New Core Data Types:**
   - **Complex Numbers (`Complex64`)**: Necessary for FFT and signal processing algorithms.
   - **Booleans (`bool`)**: To support advanced masking (e.g., `A[A > 5] = 0`).
   - **Strings (`String`)**: To allow robust logging and dynamic `read_csv` path parsing.

2. **GPU Acceleration (CUDA/Metal)**: We need contributors to bind our `NdArray` Apache Arrow buffers to `wgpu` or `CUDA` kernels so we can offload Matrix Multiplications from the CPU to the GPU.

3. **Just-In-Time Compilation (JIT)**: Currently, we execute the AST natively through an interpreter loop (`engine.execute_statement`). We want to introduce a Cranelift or LLVM backend to JIT compile `.om` scripts.

## ⚠️ Restrictions & Guidelines
1. **Zero Python Constraints**: The core engine must remain purely written in Rust. Do not introduce C++ or Python into `/compiler`. 
2. **Memory Safety First**: Do not use `unsafe` blocks unless absolutely necessary for FFI bindings (like PyO3).
3. **Testing**: Before submitting a PR modifying the AST or execution loop, you must ensure `cargo test` passes successfully.

## 🛠️ How to Contribute
1. Fork this repository.
2. Modify the `engine.rs` or `openmat.pest` files inside `/compiler/src`.
3. Test your changes using `cargo run -- run test.om`.
4. Submit a Pull Request!
