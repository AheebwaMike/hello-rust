# RPN Calculator

[![Rust](https://img.shields.io/badge/Rust-2026-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Learning Project](https://img.shields.io/badge/status-learning%20project-blue)](#)

A small Rust project that evaluates expressions in Reverse Polish Notation (RPN) using a stack-based approach.

This program is part of my learning journey in Rust. It focuses on core concepts such as enums, vectors, ownership, match expressions, and defensive error handling.

## What it does

The calculator accepts postfix expressions such as:

```text
3 4 * 3 -
```

This evaluates to:

```text
9
```

It supports these operators:

- Addition: +
- Subtraction: -
- Multiplication: *
- Division: /

It also detects common runtime issues:

- Not enough operands for an operation
- Division by zero
- Unknown operator input

## Example output

```text
=== Valid expression: 3 4 * 3 - ===
Current stack: [3.0]
Current stack: [3.0, 4.0]
Current stack: [12.0]
Current stack: [12.0, 3.0]
Current stack: [9.0]

 --- Log ---
OK : Addition operation successful
```

## Project goals

This project was created to practice and reinforce:

- Stack-based data structures
- Pattern matching with enums
- Safe program flow with guard conditions
- Logging and error reporting
- Building small, focused command-line programs in Rust

## Run it locally

From the project directory:

```bash
cargo run
```

## File overview

- src/main.rs: Core calculator logic and example expressions

## Notes

This is intentionally a simple learning-focused project. The code is designed to be easy to read and to demonstrate how a small Rust application can manage state, validation, and error handling without adding unnecessary complexity.

<div align="right">

Michael Aheebwa

</div>
