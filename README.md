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

# Mutable Variables

In Rust, variables are immutable by default, meaning once you assign a value to a variable, you cannot change it unless explicitly marked as mutable. To make a variable mutable, you use the `mut` keyword.

### Example of Mutable Variables

Here’s a simple example that demonstrates how to declare and use a mutable variable:

```rust
fn main() {
    let mut x = 5;  // `x` is mutable
    println!("The value of x is: {}", x);

    x = 10;  // Mutating `x`
    println!("The value of x is now: {}", x);
}
```

### Key Points:
- **`let mut x = 5;`**: Declares a mutable variable `x` with the initial value of 5.
- **`x = 10;`**: You can assign a new value to `x` because it's mutable.
- If you remove the `mut` keyword, you’ll get a compile-time error if you try to change `x` later in the code.

### Example of Trying to Modify an Immutable Variable:
```rust
fn main() {
    let x = 5;  // Immutable variable
    println!("The value of x is: {}", x);

    // This will cause an error because `x` is not mutable
    x = 10;
}
```
This will generate a compile-time error similar to:
```
error[E0384]: cannot assign twice to immutable variable `x`
```

### Benefits of Immutable by Default:
- **Safety**: Immutable variables prevent accidental changes and make code easier to reason about.
- **Performance**: The compiler can make optimizations when it knows a value will not change.

### When to Use Mutable Variables:
- Use mutable variables when you need to update the value over time.
- If the value is constant and doesn’t need to be changed, prefer immutability for safer and more predictable code.

# Loops

Rust provides several ways to write loops: `loop`, `while`, and `for`. Each serves a different purpose based on the use case. Let’s go through each one with examples.

### 1. **`loop`: Infinite Loop**
   The `loop` keyword is used to create an infinite loop. It will continue running until you explicitly break out of it.

   ```rust
   fn main() {
       let mut counter = 0;

       loop {
           counter += 1;
           println!("Counter: {}", counter);

           if counter == 5 {
               break;  // Exit the loop when `counter` reaches 5
           }
       }
   }
   ```

   **Explanation**:
   - This loop will increment `counter` and print it.
   - The `break` statement is used to exit the loop when `counter` reaches 5.

### 2. **`while`: Conditional Loop**
   A `while` loop runs as long as a specified condition is true. It checks the condition before each iteration.

   ```rust
   fn main() {
       let mut number = 3;

       while number != 0 {
           println!("{}!", number);

           number -= 1;
       }

       println!("Liftoff!");
   }
   ```

   **Explanation**:
   - The `while` loop checks if `number != 0`. If it's true, it runs the loop body, printing the value of `number` and decrementing it.
   - When `number` reaches 0, the loop stops.

### 3. **`for`: Iterating over a Range**
   The `for` loop is used to iterate over a collection or a range of values. It’s commonly used for fixed-range loops.

   ```rust
   fn main() {
       for i in 1..5 {
           println!("i is: {}", i);
       }
   }
   ```

   **Explanation**:
   - The `for i in 1..5` loop iterates from 1 to 4 (`..` is exclusive, meaning it stops before 5).
   - You can also use `1..=5` to include 5 (`..=` is inclusive).

### 4. **`for` Loop with Iterators**
   Rust's `for` loop can iterate over any type that implements the `Iterator` trait, including arrays and vectors.

   ```rust
   fn main() {
       let numbers = [10, 20, 30, 40, 50];

       for number in numbers.iter() {
           println!("The number is: {}", number);
       }
   }
   ```

   **Explanation**:
   - The `for` loop here iterates over an array using `.iter()`, which returns an iterator for the array.

### 5. **Loop with Labels and Nested Loops**
   You can use labels to manage control flow in nested loops, allowing you to break out of a specific loop.

   ```rust
   fn main() {
       let mut count = 0;

       'outer: loop {  // Label for the outer loop
           println!("count = {}", count);
           let mut remaining = 10;

           loop {
               if remaining == 9 {
                   break;
               }
               if count == 2 {
                   break 'outer;  // Exits the outer loop
               }
               remaining -= 1;
           }

           count += 1;
       }

       println!("End of loop");
   }
   ```

   **Explanation**:
   - The `'outer` label allows you to specify that you want to break out of the outer loop when `count == 2`.
   - Without the label, `break` would only exit the inner loop.

### 6. **Returning Values from Loops**
   You can also return values from loops by specifying a value after the `break` keyword.

   ```rust
   fn main() {
       let mut counter = 0;

       let result = loop {
           counter += 1;

           if counter == 10 {
               break counter * 2;  // Return value after `break`
           }
       };

       println!("The result is: {}", result);
   }
   ```

   **Explanation**:
   - When `counter == 10`, the loop breaks and returns `counter * 2`, which is 20 in this case.
   - The result is assigned to the `result` variable.

---

### Summary:
- **`loop`**: Runs infinitely unless explicitly stopped with `break`.
- **`while`**: Runs as long as a condition is true.
- **`for`**: Iterates over a range or collection.

Loops in Rust are powerful and versatile, allowing for a wide range of use cases. Let me know if you need further clarification or examples!