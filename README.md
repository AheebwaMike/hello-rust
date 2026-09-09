# Rust Learning Journey

[![Language](https://img.shields.io/badge/Language-Rust-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/Edition-2024-blue)](https://doc.rust-lang.org/edition-guide/)
[![Status](https://img.shields.io/badge/Status-Active_Learning-green)](https://github.com/)

A personal Rust learning workspace covering fundamentals, practice projects, algorithmic exercises, and guided study material.

---

## Repository Structure

```text
.
|-- README.md                  # Project overview and notes
|-- hello, rust/              # Main playground for Rust basics and experiments
|-- kata/                      # Coding challenge folders by difficulty
|   |-- easy/                  # Beginner-level tasks
|   |-- medium/                # Intermediate practice problems
|   |-- hard/                  # More advanced challenge work
|   |-- complex/               # Broader multi-step problem sets
|   `-- README.md              # Kata instructions and notes
|-- muscle-up/                # Notes and focused learning material
|-- practice/                 # General Rust practice and small prototypes
|-- programs/                 # Small standalone applications
|   |-- guessing-game/         # Number guessing CLI game
|   `-- library-manager/       # Simple library management project
|-- rustlings/                # Rustlings exercises and solutions
|   |-- exercises/            # Exercise prompts
|   |-- solutions/            # Completed solutions
|   |-- Cargo.toml            # Workspace manifest
|   `-- rust-analyzer.toml    # Rust Analyzer config
`-- .gitignore                # Git ignore rules
```

---

## Project Areas

### 1. hello, rust
Location: `hello, rust/`

This is the main learning sandbox for core Rust concepts such as:
- Variables and mutability
- Functions and control flow
- Ownership and borrowing basics
- String and type handling
- Small experiments and syntax practice

### 2. kata
Location: `kata/`

Challenge work sorted by difficulty:
- `easy/` for beginner tasks
- `medium/` for intermediate problems
- `hard/` for deeper algorithmic work
- `complex/` for more advanced combined concepts

### 3. practice
Location: `practice/`

A place for focused Rust exercises and quick prototype code outside the main learning modules.

### 4. programs
Location: `programs/`

Standalone mini-projects:
- `guessing-game/` — CLI guessing game using input and comparison logic
- `library-manager/` — small application/project for managing basic library-style data

### 5. rustlings
Location: `rustlings/`

This contains the Rustlings curriculum with:
- guided exercises in `exercises/`
- reference solutions in `solutions/`
- workspace-level configuration for the course

### 6. muscle-up
Location: `muscle-up/`

This folder is used for notes, study material, and deeper learning documents related to Rust concepts and development practice.

---

## Typical Commands

### Run the main playground
```bash
cd "hello, rust"
cargo run
```

### Run a practice project
```bash
cd practice
cargo run
```

### Run a program project
```bash
cd programs/guessing-game
cargo run
```

or

```bash
cd programs/library-manager
cargo run
```

### Run Rustlings
```bash
cd rustlings
cargo run
```

If the repository is configured for the `rustlings` CLI, you can also use:
```bash
cd rustlings
rustlings watch
```

---

## Learning Focus

Current topics in this workspace include:
- [x] Basic Rust syntax
- [x] Variables, functions, and flow control
- [x] Data types and string usage
- [x] Small CLI projects
- [x] Coding challenge solving
- [ ] Ownership, borrowing, and lifetimes
- [ ] Structs, enums, and errors
- [ ] Traits and generics
- [ ] Iterators and smart pointers
- [ ] Concurrency and advanced Rust patterns

---

## Notes

This repository is intentionally organized as a flexible learning workspace rather than a single app. Each folder is meant to hold a different kind of Rust practice, from small experiments to structured exercises and mini-projects.
