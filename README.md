# Rust Learning Journey

[![Language](https://img.shields.io/badge/Language-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/Edition-2024-blue)](https://doc.rust-lang.org/edition-guide/)
[![Status](https://img.shields.io/badge/Status-Active_Learning-green)](https://github.com/)

A structured personal repository dedicated to learning and mastering the Rust programming language, ranging from fundamental language semantics to algorithmic challenges and CLI applications.

---

## Repository Overview

This workspace is organized modularly to capture exercises, self-contained mini-projects, and algorithmic problem-solving:

```text
.
|-- 001_guessing_game/    # Interactive CLI number guessing game
|-- hello, rust/          # Exploratory playground for core syntax & mutability
|-- kata/                 # Problem-solving exercises grouped by difficulty
|   |-- easy/             # Basic algorithms and manipulation tasks
|   |-- medium/           # Intermediate data structures & algorithms
|   |-- hard/             # Complex logic and performance challenges
|   `-- complex/          # Advanced multi-concept problem sets
`-- rustlings/            # Official Rustlings exercises and solutions
```

---

## Projects & Modules

### 1. hello, rust
* Location: `hello, rust/`
* Focus: Language fundamentals, variable binding, mutability, data types, and terminal formatting.

### 2. 001_guessing_game
* Location: `001_guessing_game/`
* Focus: Interactive CLI game implementing:
  * Standard I/O handling (`std::io::stdin`, `std::io::stdout`)
  * Random number generation with the `rand` crate
  * Control flow, loops, and pattern matching (`match`, `Ordering`)

### 3. Kata (Coding Challenges)
* Location: `kata/`
* Focus: Algorithmic problem solving across multiple difficulty tiers:
  * `easy/src/bin/fibonacci.rs` - Fibonacci sequence calculation
  * `easy/src/bin/temperature_converter.rs` - Celsius / Fahrenheit temperature conversion
  * `easy/src/bin/square_every_digit.rs` - Digit parsing and numeric manipulation
  * `easy/src/bin/what_is_the_biggest_number.rs` - Array traversal and value comparison
  * `easy/src/bin/middle_character.rs` - String slicing and inspection

### 4. Rustlings Exercises
* Location: `rustlings/`
* Focus: Small directed exercises for reading and writing Rust code, covering:
  * Variables, Functions, Control Flow, and Primitive Types
  * Ownership, References, and Move Semantics
  * Structs, Enums, Options, and Pattern Matching
  * Error Handling, Generics, Traits, and Lifetimes
  * Iterators, Smart Pointers, Concurrency, and Macros

---

## Getting Started

### Prerequisites
* Rust toolchain (Rust 2024 edition or latest stable)
* Cargo package manager

To install Rust on your system:
```powershell
winget install Rustlang.Rustup
```
Or follow instructions at [rustup.rs](https://rustup.rs).

### Running Projects

#### Run Hello Rust
```bash
cd "hello, rust"
cargo run
```

#### Run Guessing Game
```bash
cd 001_guessing_game
cargo run
```

#### Run Specific Kata Binaries
```bash
cd kata/easy
cargo run --bin fibonacci
cargo run --bin temperature_converter
cargo run --bin square_every_digit
cargo run --bin what_is_the_biggest_number
cargo run --bin middle_character
```

#### Run Rustlings Exercises
```bash
cd rustlings
rustlings watch
```

---

## Learning Goals & Topics Covered

- [x] Basic syntax, variables, mutability, and data types
- [x] Standard I/O and external crate integration (`rand`)
- [x] String manipulation, parsing, and formatting
- [x] Control flow and pattern matching
- [ ] Memory safety, ownership, and borrowing
- [ ] Structs, Enums, and Option/Result error handling
- [ ] Traits, Generics, and Lifetimes
- [ ] Iterators and Closures
- [ ] Concurrency and Multithreading
- [ ] Unsafe Rust and Advanced Macros
