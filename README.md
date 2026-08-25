<p align="center">
  <img src="assets/logo.png" width="200" height="200" alt="OpenMatL Core Logo">
</p>
<h1 align="center">OpenMatL-Core</h1>
<h3 align="center">The Rust Compilation Engine for OpenMatL</h3>

Welcome to the Engine Room! `openmatl-core` is the native, highly-optimized Rust backend that powers the OpenMatL programming language. 

*(Note: If you are a Data Scientist looking to write `.om` scripts or view the Standard Library, please visit our sister repository: [openmatl-lang/openmatl](https://github.com/openmatl-lang/openmatl)).*

## ⚙️ Architecture

The OpenMatL Engine is designed around extreme parallelism and memory safety:
1. **AST Parser (`openmatl.pest`)**: We use Pest to define a Turing-complete grammar.
2. **NdArray (`engine.rs`)**: Matrices are stored as flattened `Arc<Float64Array>` Apache Arrow buffers.
3. **Rayon Threading**: All matrix dot products and `.backward()` Autograd gradients are parallelized automatically across all available CPU threads using `rayon::par_iter`.

## 🛠️ Building the Compiler

If you are a Core Systems Engineer wanting to build the CLI from source:
```bash
git clone https://github.com/openmatl-lang/openmatl-core.git
cd openmatl-core/compiler

# Build the release binary
cargo build --release

# The compiled engine will be available at:
./target/release/om
```
