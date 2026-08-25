# OpenMat Language Specification

OpenMat (`.om`) is a Turing-complete matrix programming language designed to feel familiar to MATLAB and Python users, but executed at lightspeed across Native Rust zero-copy buffers.

## 1. Variables and Data Types
Unlike MATLAB, OpenMat variables do not require explicit type definition. The engine inherently tracks Data Arrays and Scalars natively via the Apache Arrow `Float64Array`.

```rust
A = 5;                  // Scalar
B = [1, 2, 3];          // 1D Vector (Shape: [3])
C = [[1, 2], [3, 4]];   // 2D Matrix (Shape: [2, 2])
```

## 2. Basic Arithmetic
OpenMat inherently broadcasts arithmetic across dimensions identically to MATLAB:
```rust
A = [1, 2] + [3, 4];    // [4, 6]
B = [[1, 2], [3, 4]] * 2; // [[2, 4], [6, 8]]
```

## 3. Matrix Dot Product (`@`)
Unlike MATLAB's standard `*`, OpenMat distinctly separates Element-wise multiplication (`*`) from Matrix Dot Products (`@`).
```rust
A = [[1, 2], [3, 4]];
B = [[2, 0], [0, 2]];

C = A @ B; // Yields [[2, 4], [6, 8]]
```

## 4. Control Flow
OpenMat utilizes standard C-like block enclosures (`{}`) for control flow rather than MATLAB's `end` blocks.
```rust
if A[0] > 5 {
    B = A * 2;
} else {
    B = A / 2;
}
```

## 5. Machine Learning Primitives
All operations inherently execute on a separate Rayon Thread-Pool across matrix chunks for massive speed:
- **`relu(x)`**: Applies Rectified Linear Unit mapping.
- **`sigmoid(x)`**: Applies the Sigmoid curve mapping.
- **`sin(x), cos(x), tan(x)`**: Trigonometric native mappings.
- **`sum(x), mean(x)`**: Arrow vector sum reductions.

## 6. User-Defined Functions
Variables declared inside a function body are completely isolated from the global engine scope.
```rust
fn deep_network(input, weights) {
    Z = input @ weights;
    return relu(Z);
}

output = deep_network(X, W);
```

## 7. Autograd and Memory
Tracking gradients natively allocates a parallel lock `Arc<Mutex<Option<Float64Array>>>` across the tensor memory pointer:
```rust
output.backward(); // Anchors gradient graph accumulation
```
