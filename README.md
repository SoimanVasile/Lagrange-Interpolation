# Lagrange Polynomial Interpolation in Rust

This project is a raw Rust implementation of **Lagrange Interpolation**. It takes a set of distinct points $(x, y)$ and calculates the coefficients of the polynomial $P(x)$ of the lowest degree that passes through all provided points.

## 🧮 How It Works

The program calculates the Lagrange Interpolating Polynomial using the formula:

$$P(x) = \sum_{j=0}^{k} y_j L_j(x)$$

Where $L_j(x)$ are the Lagrange basis polynomials defined as:

$$L_j(x) = \prod_{i=0, i \neq j}^{k} \frac{x - x_i}{x_j - x_i}$$

### Code Logic
1.  **`lj` Function**: Calculates the coefficients for a specific basis polynomial $L_j(x)$ given a specific point $(x_j, y_j)$ and the list of all $x$ values. It handles the polynomial expansion (multiplication of binomials) and normalization by the denominator.
2.  **`sum_of_lists` Function**: Performs vector addition to sum the coefficients of the individual basis polynomials.
3.  **`main` Function**: Defines the dataset, iterates through all points to generate the basis polynomials, and accumulates them into the final polynomial.

## 🚀 Usage

### Prerequisites
You must have the Rust toolchain installed. If you don't have it, install it via [rustup.rs](https://rustup.rs/).

### Running the Code

1.  **Save the file**: Ensure your code is saved as `main.rs`.
2.  **Compile and Run**:
    You can use `rustc` directly or run it inside a Cargo project.

    **Option A: Using `rustc` (easiest for single file)**
    ```bash
    rustc main.rs
    ./main   # On Windows, use .\main.exe
    ```

    **Option B: Using Cargo**
    1. Create a project: `cargo new lagrange_interpolation`
    2. Replace `src/main.rs` with this code.
    3. Run:
    ```bash
    cargo run
    ```

## 📊 Input and Output

### Input Data
Currently, the input vectors are hardcoded in the `main` function:
* **X values**: `[1.2, 2.1, 3.2]`
* **Y values**: `[2.4, 3.0, 4.1]`

### Output
The program outputs a vector representing the **coefficients of the polynomial** in descending order of power (highest degree first).

**Example Output:**
```text
[0.60927..., -1.4398..., 3.235...]
