<p align="center">
  <img src="assets/logo.png" width="200" height="200" alt="OpenMat Core Logo">
</p>
<h1 align="center">OpenMat-Core</h1>
<h3 align="center">The Rust Compilation Engine for OpenMat</h3>

Welcome to the Engine Room! `openmat-core` is the native, highly-optimized Rust backend that powers the OpenMat programming language. 

*(Note: If you are a Data Scientist looking to write `.om` scripts or view the Standard Library, please visit our sister repository: [openmat-lang/openmat](https://github.com/openmat-lang/openmat)).*

## ⚙️ Architecture

The OpenMat Engine is designed around extreme parallelism and memory safety:
1. **AST Parser (`openmat.pest`)**: We use Pest to define a Turing-complete grammar.
2. **NdArray (`engine.rs`)**: Matrices are stored as flattened `Arc<Float64Array>` Apache Arrow buffers.
3. **Rayon Threading**: All matrix dot products and `.backward()` Autograd gradients are parallelized automatically across all available CPU threads using `rayon::par_iter`.

## 🛠️ Building the Compiler

If you are a Core Systems Engineer wanting to build the CLI from source:
```bash
git clone https://github.com/openmat-lang/openmat-core.git
cd openmat-core/compiler

# Build the release binary
cargo build --release

# The compiled engine will be available at:
./target/release/om
```
