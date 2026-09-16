# Number Guessing Game

[![Rust](https://img.shields.io/badge/Rust-2026-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Learning Project](https://img.shields.io/badge/status-learning%20project-blue)](#)

A small Rust console game where the player tries to guess a random number between 1 and 100 within a limited number of attempts.

This project is part of my Rust learning journey and focuses on key beginner concepts such as loops, conditionals, user input, random number generation, pattern matching, and basic input validation.

## What it does

The game randomly selects a secret number and asks the player to guess it.

Each round:

- the player enters a guess
- the program compares it to the hidden number
- the user is told whether the guess is too high or too low
- the player gets 10 chances to get it right

Example gameplay:

```text
╔══════════════════════════════╗
║   Number Guessing Game      ║
╚══════════════════════════════╝

Instructions
─────────────
• I’m thinking of a number between 1 and 100.
• You have 10 chances to guess it.
• I’ll tell you if your guess is too high or too low.

┌─ Attempt 1 of 10 ─┐
│ Chances left: 10 │
└────────────────────────┘
Enter your guess: 50
50 is too low. Try again.

┌─ Attempt 2 of 10 ─┐
│ Chances left: 9 │
└────────────────────────┘
Enter your guess: 75
75 is too high. Try again.

...

Correct! The number was 68.
```

## Project goals

This project was created to practice and reinforce:

- Random number generation with `rand`
- Reading user input from the terminal
- Handling invalid input safely
- Comparing values using `Ordering`
- Looping until the game ends
- Writing a simple interactive CLI program in Rust

## Run it locally

From the project directory:

```bash
cargo run
```

## File overview

- src/main.rs: Game logic, random number generation, input handling, and win/lose flow

## Notes

This is a simple learning-focused project intended to make Rust basics more practical and fun. The code is intentionally easy to follow and demonstrates how a beginner-friendly console game can manage state, validation, and repeated interaction in a clean way.

<div align="right">

Michael Aheebwa

</div>
