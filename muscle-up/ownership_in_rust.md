# Rust Ownership, Borrowing & Slices — Self-Assessment
*Covers: Chapter 4 of "The Rust Programming Language" (pp. 82–114) — Ownership, the Stack/Heap, Move/Clone/Copy, References & Borrowing, Dangling References, and Slices.*

Work through this without peeking at the answer key at the bottom. Grade yourself honestly — the point is to find the gaps, not the score.

---

## Part 1 — Multiple Choice (30 questions)

Each question has exactly one correct answer unless stated otherwise.

**1.** What is the primary problem that Rust's ownership system is designed to solve?
A) Making code run faster than C
B) Managing memory safety without a garbage collector
C) Allowing multiple owners of the same heap data
D) Preventing all runtime panics

**2.** Which of the following is **not** one of the three ownership rules?
A) Each value in Rust has an owner
B) There can only be one owner at a time
C) Ownership can be shared freely between threads
D) When the owner goes out of scope, the value is dropped

**3.** Data pushed onto the stack must have:
A) A size that can grow at runtime
B) A known, fixed size at compile time
C) At least one heap-allocated field
D) A `Drop` implementation

**4.** What does the memory allocator return when you allocate space on the heap?
A) A copy of the data
B) A pointer to the location
C) A new stack frame
D) A reference count

**5.** Why is pushing to the stack generally faster than allocating on the heap?
A) The stack uses a garbage collector
B) The allocator never has to search for space — it's always at the top
C) The heap is not contiguous in memory
D) Stack values are always smaller

**6.** According to the chapter, what is the *main purpose* ownership addresses?
A) Speeding up arithmetic operations
B) Managing heap data (tracking, minimizing duplication, cleaning up)
C) Enforcing type safety
D) Preventing integer overflow

**7.** In `let s = String::from("hello");`, what does `::` do?
A) Dereferences `s`
B) Namespaces the `from` function under the `String` type
C) Declares `s` as mutable
D) Casts the literal to a `String`

**8.** Why can string literals (`&str` from source code) be stored directly in the binary?
A) They are always short
B) Their contents and size are known at compile time
C) They implement `Drop`
D) The compiler heap-allocates them at build time

**9.** What special function does Rust call automatically when a variable goes out of scope?
A) `free()`
B) `drop()`
C) `dealloc()`
D) `clear()`

**10.** The RAII pattern mentioned in the chapter (familiar from C++) refers to:
A) Randomized Access Isolation Interface
B) Resource Acquisition Is Initialization
C) Reference Allocation In Iteration
D) Runtime Assertion of Invariant Integrity

**11.** After `let s1 = String::from("hello"); let s2 = s1;`, what happens to `s1`?
A) It still holds a valid copy of the data
B) It is moved and considered invalid
C) It is automatically cloned
D) It becomes a reference to `s2`

**12.** What would happen if Rust automatically deep-copied heap data on every assignment?
A) Nothing — this is what Rust already does
B) Assignments could become expensive for large data
C) It would cause double-free errors
D) It would violate the borrow checker

**13.** A "move" in Rust is best described as:
A) A shallow copy that also invalidates the original variable
B) A deep copy of both stack and heap data
C) A reference count increment
D) A compiler warning with no runtime effect

**14.** What compiler error do you get when you try to use a variable after it has been moved?
A) E0502
B) E0382 (borrow of moved value)
C) E0106
D) E0499

**15.** What is the key difference between `.clone()` and a plain assignment (`=`) for a `String`?
A) `.clone()` only copies the pointer
B) `.clone()` performs a deep copy, duplicating heap data
C) `=` always deep-copies, `.clone()` moves
D) There is no difference for `String`

**16.** Why does seeing a `.clone()` call in code act as a "visual indicator"?
A) It signals a possible compile error
B) It flags that potentially expensive, arbitrary code is running
C) It means the value is on the stack
D) It always indicates a bug

**17.** Which of these types does **not** implement the `Copy` trait?
A) `i32`
B) `bool`
C) `String`
D) `char`

**18.** A tuple `(i32, i32)` implements `Copy`. What about `(i32, String)`?
A) It also implements `Copy`
B) It does not implement `Copy`, because `String` doesn't
C) It implements `Copy` only in debug mode
D) Tuples never implement `Copy`

**19.** Rust will refuse to let a type implement both `Copy` and:
A) `Clone`
B) `Debug`
C) `Drop`
D) `PartialEq`

**20.** In Listing 4-3, after `takes_ownership(s)` is called with a `String`, using `s` again in `main` would:
A) Work fine
B) Cause a compile-time error
C) Cause a runtime panic
D) Silently clone `s`

**21.** Passing an `i32` into a function that takes ownership of it (like `makes_copy`) means the caller:
A) Can no longer use the original variable
B) Can still use the original variable afterward, because `i32` is `Copy`
C) Must explicitly call `.clone()` first
D) Gets a compile error

**22.** What problem does returning tuples like `(String, usize)` from a function (Listing 4-5) mainly solve, and what's its downside?
A) It solves borrowing errors but is slower than references
B) It hands ownership back to the caller but requires a lot of ceremony
C) It avoids the stack entirely
D) It automatically implements `Copy` for the return type

**23.** A reference in Rust (created with `&`) is best described as:
A) A new owner of the data
B) An address you can follow to data owned by someone else
C) Always a mutable pointer
D) A guaranteed deep copy

**24.** What is the term used for creating a reference to a value?
A) Aliasing
B) Borrowing
C) Cloning
D) Dereferencing

**25.** Given `fn calculate_length(s: &String) -> usize`, when `s` goes out of scope at the end of the function:
A) The `String` data is dropped
B) Nothing happens to the underlying data, since `s` doesn't own it
C) A compile error occurs
D) The caller's variable becomes invalid

**26.** Why does this fail to compile?
```rust
fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
A) `push_str` doesn't exist on `String`
B) References are immutable by default; this one isn't marked `&mut`
C) `some_string` was moved into the function
D) `String` doesn't implement `Drop`

**27.** How many mutable references to the same value can exist at one time (within overlapping scopes)?
A) Unlimited
B) At most one
C) Exactly two
D) Depends on the type

**28.** Which combination is **not** allowed simultaneously (overlapping scopes) in safe Rust?
A) Two immutable references to the same value
B) One mutable reference and one immutable reference to the same value
C) One mutable reference alone
D) Zero references

**29.** What three conditions together define a data race, per the chapter?
A) Two+ pointers accessing data simultaneously, at least one writing, no synchronization
B) Two+ threads existing in the same program
C) A `Copy` type being mutated
D) A `String` being cloned twice

**30.** Why does Rust prevent dangling references at compile time?
A) It tracks lifetimes to ensure the referenced data outlives the reference
B) It uses a garbage collector to keep data alive
C) It converts all references to owned values automatically
D) It panics at runtime if a reference outlives its data

---

## Part 2 — True / False (10 questions)

**31.** In Rust, if a type has implemented the `Drop` trait, it can still also implement `Copy`.

**32.** A string slice (`&str`) is a reference and therefore does not own the data it points to.

**33.** `let r1 = &s; let r2 = &s;` (two immutable references, no mutable reference) is allowed simultaneously.

**34.** A reference's scope always extends all the way to the closing curly brace of the block it was created in, regardless of where it's last used (this is called "non-lexical lifetimes" being ignored).

**35.** Returning `&String` from a function that creates the `String` locally (without lifetimes/`'static`) will fail to compile.

**36.** `first_word(s: &str) -> &str` is strictly more restrictive (works on fewer input types) than `first_word(s: &String) -> &str`.

**37.** Slice range indices for `&str` must fall on valid UTF-8 character boundaries.

**38.** `&a[1..3]` on an array `[1,2,3,4,5]` produces a slice equal to `[2,3,4]`.

**39.** Passing an `i32` by value to a function always invalidates the caller's original variable.

**40.** Calling `.clone()` on a `String` guarantees the operation is cheap, similar to copying stack-only data.

---

## Part 3 — Structured / Short Answer (3 questions)

**S1. Trace the ownership.** For the code below, state for *each* line whether it compiles, and if not, explain exactly why (name the rule violated):
```rust
fn main() {
    let s1 = String::from("rust");
    let s2 = s1;
    let s3 = s2.clone();
    println!("{}", s1);
    println!("{}, {}", s2, s3);
}
```

**S2. Borrow checker diagnosis.** The following function fails to compile. Identify the exact conflict (which borrows overlap and why), then rewrite it so it compiles while preserving the intended behavior (append `"!"` to the string and then print the original + the addition separately):
```rust
fn shout(s: &String) -> &str {
    s.push_str("!");
    s
}

fn main() {
    let msg = String::from("hello");
    let result = shout(&msg);
    println!("{}", result);
}
```

**S3. Design justification.** Explain, in your own words, why Rust's `first_word` function is better designed as `fn first_word(s: &str) -> &str` rather than returning a `usize` index (as in the early version of the function). Your answer should reference at least: (a) what problem the index-based version has, (b) how the borrow checker protects against that problem when slices are used instead, and (c) why the `&str` parameter type is more flexible than `&String`.

---

## Kata — "The Sentence Trimmer" (Challenging)

**Goal:** Practice ownership, borrowing, and slices together by writing a small text-processing module with **zero unnecessary heap allocations** — only borrow and slice, and hit every borrow-checker rule from this chapter on purpose.

### Setup
Create a new binary crate (`cargo new sentence_trimmer`) and implement the following in `src/main.rs`. No external crates.

### Requirements

1. **`fn first_word(s: &str) -> &str`**
   Same as the chapter's final version — return the first word of a string slice.

2. **`fn last_word(s: &str) -> &str`**
   Return the last word of a string slice, without allocating a `String`. (Hint: work backward through the byte slice.)

3. **`fn nth_word(s: &str, n: usize) -> Option<&str>`**
   Return the `n`-th word (0-indexed) as a slice, or `None` if there aren't enough words. Must not collect into a `Vec<String>` — you may use `.split_whitespace()` (which yields `&str` slices) but you must reason about *why* the returned slice can safely outlive the function call (tie the explanation to lifetimes/borrowing, in a comment above the function).

4. **`fn longest_word(s: &str) -> &str`**
   Return the longest word in the sentence as a slice. If there's a tie, return the first one found. This must not clone or allocate any new string data — only slice into the original.

5. **`struct SentenceStats<'a>`** — a struct that borrows from an input `&str` and holds:
   - `original: &'a str`
   - `first: &'a str`
   - `last: &'a str`
   - `longest: &'a str`

   Implement `fn analyze(s: &str) -> SentenceStats` that builds this struct using the functions above. **This is the crux of the kata**: you must satisfy the borrow checker so that all four fields borrow from the *same* underlying string with compatible lifetimes, and the struct cannot outlive the string it borrows from.

6. **The trap (do this on purpose, then fix it):** Write a `main` that:
   - Creates a `String` (not a literal) holding a sentence.
   - Calls `analyze(&s)` to get a `SentenceStats`.
   - Then calls `s.clear()`.
   - Then tries to print a field from the `SentenceStats`.

   Confirm this **fails to compile**, paste the exact compiler error into a comment block at the bottom of `main.rs`, and explain in 2–3 sentences *why* the compiler is right to reject it (reference which chapter rule is being enforced).

7. **Bonus (optional, harder):** Add `fn word_count(s: &str) -> usize` and a method `impl<'a> SentenceStats<'a> { fn summary(&self) -> String { ... } }` that *is* allowed to allocate (it's producing an owned, formatted report), demonstrating that you understand *when* allocating with `String::from`/`format!` is the right call versus when a slice is correct.

### Constraints
- No `.to_string()`, `.to_owned()`, or `String::from()` anywhere except in `main`'s initial sentence and in the optional `summary()` bonus method.
- No `unsafe`.
- Must compile with zero warnings (`cargo build` clean).
- Every function that returns a slice must have its lifetime relationship either inferred correctly or made explicit — if the compiler forces you to write an explicit lifetime annotation somewhere, leave it in and add a one-line comment explaining what it means.

### Stretch goal
Modify `nth_word` and `longest_word` to work generically over anything that can be viewed as a sequence of "words" — i.e., make them work on both `&str` sentences *and* a `&[&str]` slice of pre-split words, using either a small trait or two overloaded-by-name functions (`fn longest_word_str` / `fn longest_word_slice`). This pushes past the chapter into thinking about how slices generalize beyond strings (the chapter's closing section, "Other Slices").

---
<br>

## Answer Key

<details>
<summary>Click to reveal (Part 1 — MCQs)</summary>

1-B, 2-C, 3-B, 4-B, 5-B, 6-B, 7-B, 8-B, 9-B, 10-B,
11-B, 12-B, 13-A, 14-B, 15-B, 16-B, 17-C, 18-B, 19-C, 20-B,
21-B, 22-B, 23-B, 24-B, 25-B, 26-B, 27-B, 28-B, 29-A, 30-A

</details>

<details>
<summary>Click to reveal (Part 2 — True/False)</summary>

31 – **False** (a type with `Drop` cannot also implement `Copy`).
32 – **True**.
33 – **True** (multiple immutable references are fine).
34 – **False** (a reference's scope ends at its last use, not necessarily the closing brace — this is exactly what allows Listing 4-6/4-7-style code to compile).
35 – **True** (this is the dangling-reference / missing-lifetime-specifier error, E0106).
36 – **False** — it's the opposite: `&str` is *more* flexible (accepts both `&String` and `&str` via deref coercion), not more restrictive.
37 – **True**.
38 – **False** — `&a[1..3]` yields `[2, 3]` (end index is exclusive), not `[2,3,4]`.
39 – **False** — only true for non-`Copy` types; `i32` is `Copy`, so the original remains valid.
40 – **False** — `.clone()` is explicitly *not* guaranteed cheap; that's the whole point of it being a visible, deliberate call.

</details>

<details>
<summary>Click to reveal (Part 3 — Structured, model answers)</summary>

**S1.**
- `let s1 = String::from("rust");` — compiles, `s1` owns the data.
- `let s2 = s1;` — compiles; this **moves** `s1` into `s2`. `s1` is now invalid.
- `let s3 = s2.clone();` — compiles; `s2` is deep-copied into `s3`. Both `s2` and `s3` remain valid and independently own their own heap data.
- `println!("{}", s1);` — **fails to compile** (E0382, borrow of moved value: `s1`). `s1`'s value was moved into `s2` on the earlier line, so `s1` is no longer valid — this is the "one owner at a time" rule.
- `println!("{}, {}", s2, s3);` — compiles fine on its own; both are valid owners of their own data.

**S2.**
The conflict: `shout` takes `s: &String` (an **immutable** reference) but then calls `s.push_str(...)`, which requires a **mutable** borrow. You cannot mutate through an immutable reference — references are immutable by default, and `&String` was never declared `&mut String`. A fix that preserves the intent (append `"!"`, then print original and the addition) while satisfying the borrow checker:
```rust
fn shout(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut msg = String::from("hello");
    shout(&mut msg);
    println!("{}", msg);
}
```
This takes a mutable reference explicitly, mutates in place, and there's no overlapping immutable/mutable borrow because there's only one borrow active at a time.

**S3.** (Model answer — grade yourself on hitting these three points)
(a) The index-based version returns a bare `usize` that is only meaningful in the context of the original `String` at the moment it was computed. Nothing ties that number to the string's state, so if the string is later mutated (e.g. `s.clear()`), the index silently becomes garbage — the bug shows up later and is easy to miss, since the code compiles and runs without any error.
(b) A slice (`&str`) is a reference tied to the original data via the borrow checker's lifetime tracking. If you try to mutate the string (e.g. call `.clear()`, which needs `&mut`) while a slice derived from it is still in use, the compiler rejects the code (E0502) because an immutable borrow (the slice) and a mutable borrow (`clear()`) would overlap. The bug becomes a compile-time error instead of a silent runtime inconsistency.
(c) `&str` accepts both actual string slices *and* references to `String` (via deref coercion — `&String` coerces to `&str`), so a function taking `&str` works on string literals, whole `String`s, and partial slices of either, without the caller needing to convert anything. A function that insists on `&String` can't accept a plain `&str` literal directly.

</details>

---
*Good luck — if you score well here you've genuinely internalized the hardest conceptual chapter in early Rust. The borrow checker only gets more nuanced (lifetimes proper, `Rc`/`RefCell`, interior mutability) from here, but this chapter is the foundation everything else stands on.*

> Enjoy