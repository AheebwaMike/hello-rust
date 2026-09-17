# Rust Learning Journey

[![Language](https://img.shields.io/badge/Language-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)

A personal workspace for learning Rust through small experiments, kata-style problems, and standalone mini-projects.

---

## Repository Structure

```text
.
|-- README.md                 # Repository overview and notes
|-- .gitignore                # Ignored build output and local-only folders
|-- hello, rust/             # Main Rust learning sandbox
|-- kata/                     # Challenge folders grouped by difficulty
|   |-- README.md             # Kata notes and guidance
|   |-- easy/                 # Beginner exercises
|   |-- medium/               # Intermediate exercises
|   |-- hard/                 # More advanced exercises
|   `-- complex/              # Multi-step challenge work
|-- muscle-up/               # Notes and deeper study material
|-- programs/                # Small standalone applications
|   |-- guessing-game/        # Number guessing CLI game
|   |-- library-manager/      # Library-style data management app
|   |-- rpn-calculator/       # Reverse Polish notation calculator
|   `-- shape-metrics/        # Geometry/shape calculation exercises
`-- .gitignore                # Root ignore rules
```

> Note: This repository intentionally excludes generated target directories and local-only folders listed in the root .gitignore.

---

## Project Areas

### hello, rust
Location: `hello, rust/`

This is the main playground for experimenting with core Rust concepts, including:
- variables and mutability
- functions and control flow
- ownership and borrowing basics
- strings and simple type usage
- small syntax and implementation experiments

### kata
Location: `kata/`

A challenge workspace split by difficulty:
- `easy/` for beginner tasks
- `medium/` for intermediate problems
- `hard/` for more advanced work
- `complex/` for broader multi-step exercises

### programs
Location: `programs/`

Standalone mini-projects and small applications:
- `guessing-game/` — CLI guessing game
- `library-manager/` — basic library manager
- `rpn-calculator/` — expression evaluator using RPN
- `shape-metrics/` — shape-based calculations

### muscle-up
Location: `muscle-up/`

This folder contains notes and focused study material for deeper Rust concepts and practice.

---

## Typical Commands

### Run the main playground
```bash
cd "hello, rust"
cargo run
```

### Run a small program
```bash
cd programs/guessing-game
cargo run
```

### Run another project
```bash
cd programs/library-manager
cargo run
```

### Run a calculator project
```bash
cd programs/rpn-calculator
cargo run
```

---

## Learning Focus

Current learning goals in this workspace include:
- [x] Basic Rust syntax
- [x] Variables, functions, and flow control
- [x] Small CLI and console projects
- [x] Kata-style practice problems
- [x] Ownership and borrowing
- [ ] Structs, enums, and error handling
- [ ] Traits and generics
- [ ] Iterators and smart pointers
- [ ] Concurrency and advanced patterns

---

## Notes

This repository is organized as a flexible Rust learning workspace rather than a single application. Each tracked folder serves a different purpose: experimentation, challenge solving, notes, or compact standalone programs.
