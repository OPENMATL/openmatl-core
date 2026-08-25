<p align="center">
  <svg width="200" height="200" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
    <defs>
      <linearGradient id="neonCyan" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stop-color="#00f2fe" />
        <stop offset="100%" stop-color="#4facfe" />
      </linearGradient>
      <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
        <feGaussianBlur stdDeviation="6" result="blur" />
        <feComposite in="SourceGraphic" in2="blur" operator="over" />
      </filter>
    </defs>
    <!-- Background -->
    <rect width="200" height="200" rx="45" fill="#0d1117" />
    <!-- Matrix Bracket Left -->
    <path d="M 60 40 L 40 40 L 40 160 L 60 160" fill="none" stroke="url(#neonCyan)" stroke-width="10" filter="url(#glow)" stroke-linecap="round" stroke-linejoin="round"/>
    <!-- Matrix Bracket Right -->
    <path d="M 140 40 L 160 40 L 160 160 L 140 160" fill="none" stroke="url(#neonCyan)" stroke-width="10" filter="url(#glow)" stroke-linecap="round" stroke-linejoin="round"/>
    <!-- Nodes -->
    <circle cx="80" cy="75" r="10" fill="#00f2fe" filter="url(#glow)"/>
    <circle cx="120" cy="75" r="10" fill="#4facfe" filter="url(#glow)"/>
    <circle cx="80" cy="125" r="10" fill="#4facfe" filter="url(#glow)"/>
    <circle cx="120" cy="125" r="10" fill="#00f2fe" filter="url(#glow)"/>
  </svg>
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
