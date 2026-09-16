# Library Manager

[![Rust](https://img.shields.io/badge/Rust-2026-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Learning Project](https://img.shields.io/badge/status-learning%20project-blue)](#)

A small Rust project that manages a simple library catalog and tracks whether books are available or checked out.

This program is part of my learning journey in Rust. It focuses on core concepts such as structs, vectors, methods, ownership, and state changes in a small command-line application.

## What it does

The program creates a list of books and displays their current status. Each book includes:

- ID
- title
- author
- publication year
- current borrower

It supports a basic checkout and return flow:

- A book can be checked out by a borrower
- If it is already checked out, the action is rejected
- A returned book becomes available again
- The catalog can be displayed at any point

## Example output

```text
--- Library Catalog ---

Booklist:
ID: 1 | 'Things Fall Apart' by Chinua Achebe (1958) - Available
ID: 2 | 'Deep Learning Book' by Ian GoodFellow et al. (2006) - Available
ID: 3 | 'The science behind portals' by Michael Aheebwa (2027) - Available

Attempting to checkout to Alice...
LibSys-v1: Success. Checked out to Alice

Attempting to checkout to John...
LibSys-v1: Sorry John, this book is already checked out to Alice

Updated Book list:
ID: 1 | 'Things Fall Apart' by Chinua Achebe (1958) - Checked out to Alice
ID: 2 | 'Deep Learning Book' by Ian GoodFellow et al. (2006) - Available
ID: 3 | 'The science behind portals' by Michael Aheebwa (2027) - Available

LibSys-v1: Book 'Things Fall Apart' Returned by Alice

Updated Book list:
ID: 1 | 'Things Fall Apart' by Chinua Achebe (1958) - Available
ID: 2 | 'Deep Learning Book' by Ian GoodFellow et al. (2006) - Available
ID: 3 | 'The science behind portals' by Michael Aheebwa (2027) - Available
```

## Project goals

This project was created to practice and reinforce:

- Defining data models with structs
- Managing collections with `Vec`
- Updating object state through methods
- Handling conditional logic for availability checks
- Building a small interactive catalog in Rust

## Run it locally

From the project directory:

```bash
cargo run
```

## File overview

- src/main.rs: Book model, catalog logic, checkout and return behavior, and example data

## Notes

This is a small learning-focused project designed to demonstrate how a basic library system can be modeled with Rust structs and methods. The code keeps things simple and readable while showing how state can be tracked and updated in a practical console application.

<div align="right">

Michael Aheebwa

</div>
