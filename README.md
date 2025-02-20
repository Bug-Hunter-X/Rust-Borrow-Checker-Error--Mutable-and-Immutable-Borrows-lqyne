# Rust Borrow Checker Error: Mutable and Immutable Borrows

This repository demonstrates a common error encountered when working with references in Rust: attempting to use an immutable reference after a mutable reference to the same data has been established.  The Rust compiler's borrow checker prevents this to ensure data integrity and prevent data races.

The `bug.rs` file contains code that will fail to compile because it violates the borrow checker's rules. The `bugSolution.rs` file provides a corrected version.  This demonstrates best practices for handling mutable and immutable references in Rust.

## Running the Code

1. Clone the repository:
   ```bash
git clone https://github.com/yourusername/rust-borrow-error.git
   ```
2. Navigate to the repository directory:
   ```bash
cd rust-borrow-error
   ```
3. Compile and run the `bug.rs` file (this will result in a compile-time error):
   ```bash
rustc bug.rs
./bug
   ```
4. Compile and run the `bugSolution.rs` file (this will run successfully):
   ```bash
rustc bugSolution.rs
./bugSolution
   ```