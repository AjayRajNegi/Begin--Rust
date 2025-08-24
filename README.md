# Rust Learning Project

A collection of Rust examples and exercises demonstrating fundamental concepts including vectors, enums, pattern matching, structs, and option types.

## Project Overview

This project contains various Rust code examples organized into different modules, each focusing on specific Rust concepts:

- **Collections**: Vector operations and filtering
- **Option Enum**: Working with Option types and string manipulation
- **Pattern Matching**: Enum-based shape calculations with match expressions
- **Structs**: User-defined data structures

## Project Structure

```
rust1/
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── README.md           # This file
├── collections.rs      # Vector operations example
└── src/
    ├── main.rs         # Main application with vector filtering
    ├── optionEnum.rs   # Option type examples
    ├── patternMatching.rs # Pattern matching with shapes
    └── struct.rs       # Struct examples
```

## Features

### 1. Vector Operations (`main.rs` & `collections.rs`)

- Creating and manipulating vectors
- Filtering even numbers from a vector
- Demonstrates basic vector operations

### 2. Option Enum (`optionEnum.rs`)

- String character searching using Option types
- Pattern matching with `Some` and `None` variants
- Safe handling of potentially missing values

### 3. Pattern Matching (`patternMatching.rs`)

- Enum-based shape definitions (Circle, Square, Rectangle)
- Area calculations using match expressions
- Demonstrates exhaustive pattern matching

### 4. Structs (`struct.rs`)

- User-defined data structures
- Struct instantiation and field access
- String handling in structs

## Getting Started

### Prerequisites

- Rust installed on your system (version 1.70+ recommended)
- Cargo package manager

### Installation

1. Clone or download this project
2. Navigate to the project directory:
   ```bash
   cd rust1
   ```

### Running the Examples

#### Main Application

```bash
cargo run
```

This runs the main application with vector filtering functionality.

#### Individual Examples

To run specific examples, you can modify the `main.rs` file or create separate binaries for each module.

## Code Examples

### Vector Filtering

```rust
fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}
```

### Option Type Usage

```rust
fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32)
        }
    }
    return None;
}
```

### Pattern Matching with Enums

```rust
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side
    }
}
```

### Struct Definition and Usage

```rust
struct User {
    name: String,
    age: u32,
    active: bool
}

let user = User {
    name: String::from("Alice"),
    age: 30,
    active: true
};
```

## Learning Objectives

This project demonstrates:

- Basic Rust syntax and control flow
- Vector operations and collections
- Option types for safe null handling
- Pattern matching with enums
- Struct definitions and usage
- String manipulation
- Error handling patterns

## Contributing

Feel free to add more examples or improve existing code. This is a learning project, so contributions that add educational value are welcome.

## License

This project is open source and available under the MIT License.

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
