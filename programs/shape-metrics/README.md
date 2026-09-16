# Shape Metrics

[![Rust](https://img.shields.io/badge/Rust-2026-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Learning Project](https://img.shields.io/badge/status-learning%20project-blue)](#)

A small Rust project that computes area and perimeter for several different shapes using enums and match-based logic.

This program is part of my learning journey in Rust. It focuses on core concepts such as enums, pattern matching, methods, iterators, and basic data modeling for a small geometry-based application.

## What it does

The program defines a set of shapes, including:

- Circle
- Triangle
- Rectangle
- Square

For each shape, it calculates:

- area
- perimeter
- a human-readable summary

It also identifies the shape with the largest area in the collection.

## Example output

```text
--- Shape Collection ---
Circle (radius: 1) — Area: 3.14 sq. units, Perimeter: 6.28 units
Circle (radius: 0.7) — Area: 1.54 sq. units, Perimeter: 4.40 units
Square (s: 6.2) — Area: 38.44 sq. units, Perimeter: 24.80 units
Triangle (a: 3, b: 4, c: 5) — Area: 6.00 sq. units, Perimeter: 12.00 units
Rectangle (s1: 2.3, s2: 10) — Area: 23.00 sq. units, Perimeter: 24.60 units

--- Shape with largest area ---
Largest: Square (s: 6.2) — Area: 38.44 sq. units, Perimeter: 24.80 units

--- Bonus ---
First shape is a circle with radius 1
```

## Project goals

This project was created to practice and reinforce:

- Enums for representing different shape types
- Pattern matching with `match`
- Reusable methods on a shared type
- Comparing computed values across a collection
- Writing clear, readable geometry logic in Rust

## Run it locally

From the project directory:

```bash
cargo run
```

## File overview

- src/main.rs: Shape enum, area/perimeter calculations, comparison logic, and example data

## Notes

This is a simple learning-focused project intended to show how Rust can model a variety of related values cleanly using enums and match expressions. The code stays compact and readable while still demonstrating real computation and organization.

<div align="right">

Michael Aheebwa

</div>
