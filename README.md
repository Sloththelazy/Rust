# Rust

## Installation and Getting Started

To initialize a new Rust project locally, follow these steps:

1. **Install Rust** (if not already installed):
   - Run the following command to install Rust via the Rust toolchain installer `rustup`:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - After installation, add `cargo` and `rustc` to your path by restarting your terminal or using:
     ```bash
     source $HOME/.cargo/env
     ```

2. **Create a New Rust Project**:
   - Use the `cargo` command to create a new project:
     ```bash
     cargo init
     cargo new project_name
     ```
   - Replace `project_name` with the desired name of your project.

3. **Navigate to the Project Directory**:
   ```bash
   cd project_name
   ```

4. **Build the Project**:
   - Run the following command to compile the project:
     ```bash
     cargo build
     ```

5. **Run the Project**:
   - To execute the project, use:
     ```bash
     cargo run
     ```

The structure of the project will include:
- `src/main.rs`: The main Rust file.
- `Cargo.toml`: The configuration file for dependencies.


# Bianry vs Library {End User Application vs Reusable}

In Rust, when creating a new project, you can choose between a **binary** or a **library** application depending on the nature of your project. Here's a breakdown of the differences and how they are used:

### Binary Application
A **binary** in Rust is a standalone executable program. When you run a binary application, Rust generates an executable file (e.g., `.exe` on Windows, no extension on Linux/Mac) that can be executed directly from the command line or other environments.

- **Purpose**: Used for writing applications or tools that have a main entry point (like command-line programs or services).
- **Structure**:
  - The main file is typically located in `src/main.rs`.
  - Contains a `main` function which is the entry point of the program:
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

- **Cargo File**: In `Cargo.toml`, the `[package]` section defines it as a binary (this is the default behavior when creating a project via `cargo new`).
  
- **Compilation and Execution**: Compiled and executed with `cargo run` or `cargo build`, which creates an executable in the `target/debug/` directory.

### Library Application
A **library** in Rust is a collection of reusable code that can be used in other projects. It doesn't have an entry point like a binary and cannot be executed directly. Instead, it is imported and used by other projects (binary or library).

- **Purpose**: Used for creating reusable code, like utility functions, data structures, or algorithms that can be shared across multiple projects or used in other Rust programs.
- **Structure**:
  - The main file is located in `src/lib.rs`.
  - Does not contain a `main` function but instead exposes functions, types, or modules that other programs can use:
    ```rust
    pub fn hello() {
        println!("Hello from the library!");
    }
    ```

- **Cargo File**: In `Cargo.toml`, the `[lib]` section is used to define it as a library (by default, it looks for `src/lib.rs`).
  ```toml
  [lib]
  name = "my_library"
  ```

- **Usage**: To use a library, you add it as a dependency in another project (via `Cargo.toml`) or run `cargo build` to compile the library into a `.rlib` file, which can then be linked.

### Creating a Binary vs. Library in Rust

- **Binary Application**: By default, when you create a new project with `cargo new`, you get a binary project:
  ```bash
  cargo init // or
  cargo new my_binary
  # Creates src/main.rs
  ```

- **Library Application**: If you want to create a library project, you can specify it using the `--lib` flag:
  ```bash
  cargo init --lib // or
  cargo new my_library --lib
  # Creates src/lib.rs
  ```

### Hybrid Projects (Binary + Library)
A Rust project can have both binary and library components. You can use the library functions in the binary. This is useful when you want to organize reusable code into a library but still create a binary for execution.

- In this setup:
  - You have `src/lib.rs` for the library code.
  - You have `src/main.rs` for the binary's entry point, which uses the code from the library:
    ```rust
    // src/lib.rs
    pub fn hello() {
        println!("Hello from the library!");
    }

    // src/main.rs
    use my_library::hello;

    fn main() {
        hello();
    }
    ```

### Summary
- **Binary Application**: Executable program with a `main` function (`src/main.rs`).
- **Library Application**: Reusable code without a `main` function (`src/lib.rs`).
- **Hybrid**: A project with both a binary and a library component.

# Conditional Statements

## Write a code to print if a number is even or not

```rust
fn main() {
    let number = 5; // You can change this to test other numbers
    if is_even(number) {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
```

### Explanation:
- `is_even` is a function that takes an integer (`num`) as input and returns a boolean (`true` if the number is even, `false` otherwise).
- The main function tests the number using `is_even` and prints whether it's even or odd.

You can modify the `number` variable in the `main` function to check other numbers.