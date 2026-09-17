# Rust Chapter 7 Exam — Packages, Crates, and Modules

Covers: crates, packages, crate roots, the module tree, absolute/relative paths, `pub`, `super`,
public structs/enums, `use`, idiomatic `use` paths, `as`, `pub use`, external packages, nested
paths, the glob operator, and splitting modules across files.

---

## Part 1 — Multiple Choice (30 questions)

Choose the single best answer for each question.

**1.** What is the smallest amount of code the Rust compiler considers at one time?
A) A module
B) A package
C) A crate
D) A workspace

**2.** How many library crates can a single package contain at most?
A) Zero
B) One
C) Two
D) As many as it has binary crates

**3.** What must every package contain at least one of?
A) A `Cargo.lock` file
B) A crate (binary or library)
C) A `mod.rs` file
D) A workspace member

**4.** By Cargo convention, which file is the crate root of a binary crate with the same name as the package?
A) `src/root.rs`
B) `src/crate.rs`
C) `src/main.rs`
D) `Cargo.toml`

**5.** Which file does Cargo treat as the crate root of a library crate?
A) `src/lib.rs`
B) `src/library.rs`
C) `src/mod.rs`
D) `src/main.rs`

**6.** How does a package end up with multiple binary crates?
A) By declaring multiple `fn main()` in `src/main.rs`
B) By placing separate files in `src/bin`
C) By adding multiple `[package]` sections to `Cargo.toml`
D) It isn't possible — a package has exactly one binary crate

**7.** What distinguishes a binary crate from a library crate?
A) Binary crates cannot contain modules
B) A binary crate must have a `main` function that defines what happens when it runs
C) Library crates cannot depend on external packages
D) Binary crates cannot be tested

**8.** What keyword declares a new module in Rust?
A) `module`
B) `pkg`
C) `mod`
D) `namespace`

**9.** If you write `mod garden;` in your crate root, where might the compiler look for its code? (pick the most complete answer)
A) Only inline in curly brackets
B) Only in `src/garden.rs`
C) Inline in curly brackets, in `src/garden.rs`, or in `src/garden/mod.rs`
D) Only in `src/mod.rs`

**10.** What is the implicit name of the root module of a crate's module tree?
A) `root`
B) `main`
C) `crate`
D) `self`

**11.** In the module tree, if module `hosting` is defined inside module `front_of_house`, what is their relationship?
A) `hosting` is the parent, `front_of_house` is the child
B) They are siblings
C) `front_of_house` is the parent, `hosting` is the child
D) They are unrelated

**12.** What is the default visibility of an item (function, struct, module, etc.) in Rust?
A) Public to the whole crate
B) Public to the whole package
C) Private to its parent module
D) Public only outside the crate

**13.** Which two forms can a path take when referring to an item in the module tree?
A) Relative and dynamic
B) Absolute and relative
C) Static and absolute
D) Local and global

**14.** An absolute path to an item in the current crate begins with which keyword?
A) `self`
B) `super`
C) `root`
D) `crate`

**15.** A relative path can start with all of the following EXCEPT:
A) `self`
B) `super`
C) an identifier in the current module
D) `crate`

**16.** Given `mod front_of_house { mod hosting { fn add_to_waitlist() {} } }`, why does `crate::front_of_house::hosting::add_to_waitlist()` fail to compile as written?
A) `add_to_waitlist` doesn't exist
B) `hosting` (and the function) are private by default, so outside code can't reach them
C) Absolute paths are not allowed for functions
D) `front_of_house` must be declared `pub` in `Cargo.toml`

**17.** What does adding `pub` before `mod hosting` actually make public?
A) The module and everything inside it, automatically
B) Only the ability for ancestor modules to refer to the module itself — contents stay private unless marked `pub` too
C) Nothing — `pub` only works on functions
D) Only submodules, not functions or structs

**18.** Which statement about `pub` and structs is correct?
A) `pub struct Foo` automatically makes all of its fields public
B) `pub struct Foo` makes the struct public, but each field must be marked `pub` individually to be public
C) Struct fields are public by default and `pub` is only needed on the struct name
D) You cannot make a struct public without making all its fields public

**19.** Which statement about `pub` and enums is correct?
A) Like structs, each variant must be marked `pub` individually
B) Marking an enum `pub` automatically makes all of its variants public
C) Enum variants are private even if the enum itself is public
D) `pub` has no effect on enums

**20.** Why might a public struct with a private field need a public associated function (like `summer` in the `Breakfast` example)?
A) Associated functions are required for all structs
B) Without it, outside code has no way to construct an instance, since it can't set the private field directly
C) It's purely a style convention with no functional purpose
D) Private fields cannot be read, only written

**21.** What does the `super` keyword do at the start of a relative path?
A) Refers to the crate root, bypassing all modules
B) Refers to the parent of the current module
C) Refers to the current module itself
D) Imports every item from the standard library

**22.** What is the main purpose of the `use` keyword?
A) To declare a new module
B) To create a shortcut so a full path doesn't need to be repeated in that scope
C) To make an item public outside the crate
D) To rename a crate in `Cargo.toml`

**23.** If you write `use crate::front_of_house::hosting;` in the crate root, and then move a function that calls `hosting::add_to_waitlist()` into a new child module `mod customer { ... }`, what happens?
A) It still compiles — `use` applies everywhere in the crate
B) It fails to compile, because `use` only creates the shortcut within the scope it's written in
C) It compiles but emits no warnings
D) `use` statements are automatically inherited by all child modules

**24.** For bringing a *function* into scope, what is considered idiomatic?
A) `use` the function's full path directly, e.g. `use crate::front_of_house::hosting::add_to_waitlist;`
B) `use` the function's parent module, then call it as `hosting::add_to_waitlist()`
C) Never use `use` for functions
D) Always alias functions with `as`

**25.** For bringing a *struct or enum* into scope, what is considered idiomatic?
A) Bring in the parent module, not the type itself
B) Specify the full path directly to the type itself, e.g. `use std::collections::HashMap;`
C) Always use the glob operator
D) Structs and enums cannot be brought into scope with `use`

**26.** You need both `std::fmt::Result` and `std::io::Result` in the same scope. Which approach avoids a naming conflict?
A) It's impossible — Rust disallows two types with the same name in a scope regardless of path
B) Bring in the parent modules (`use std::fmt; use std::io;`) and refer to them as `fmt::Result` / `io::Result`, or rename one with `as`
C) Only one of the two can ever be used in a single crate
D) Use the glob operator for both

**27.** What does `use std::io::Result as IoResult;` do?
A) Deletes the original `Result` type
B) Creates a local alias `IoResult` for `std::io::Result` so it doesn't collide with another `Result` in scope
C) Converts `io::Result` into `fmt::Result`
D) Makes `Result` public outside the crate

**28.** What does `pub use` accomplish that plain `use` does not?
A) It brings an item into scope privately, same as `use`
B) It re-exports the item, making it available for external code to bring into their own scope from this new path
C) It declares a new module
D) It is required for glob imports to work

**29.** Which line correctly brings `Ordering` and `io` into scope from `std` using a nested path?
A) `use std::cmp::Ordering, std::io;`
B) `use std::{cmp::Ordering, io};`
C) `use std::[cmp::Ordering, io];`
D) `use std::cmp::Ordering + std::io;`

**30.** What does `use std::io::{self, Write};` bring into scope?
A) Only `Write`
B) Only `std::io`
C) Both `std::io` itself and `std::io::Write`
D) Nothing — `self` is invalid here

---

## Part 2 — True / False (10 questions)

Write **True** or **False** for each statement.

**31.** A single `.rs` file passed directly to `rustc` is still considered a crate by the compiler.

**32.** A package can contain zero crates as long as it has a valid `Cargo.toml`.

**33.** Making a module `pub` automatically makes every function and struct inside it public as well.

**34.** Items in a child module can access private items defined in their ancestor modules.

**35.** Items in a parent module can freely access private items defined inside a child module.

**36.** Choosing absolute paths over relative paths is generally preferred because item definitions and their call sites are often moved independently of each other.

**37.** The `use` keyword changes which files the compiler includes when building a crate.

**38.** For a submodule `mod vegetables;` declared inside `src/garden.rs`, the compiler will look for its code in `src/garden/vegetables.rs`.

**39.** Using the older `mod.rs`-style path (e.g. `src/front_of_house/mod.rs`) for one module and the newer `src/front_of_house.rs`-style path for a different module in the same project is allowed.

**40.** The glob operator (`*`) brings both public and private items from a path into scope.

---

## Part 3 — Programming Exercise

**Build a small library-style package that models a bookstore's inventory system**, using only
concepts covered so far (ownership/borrowing, structs, enums, modules, paths, `pub`, `use`) —
no traits, generics, closures, or collections beyond what you've already learned.

### Requirements

1. Create a library crate (`cargo new bookstore --lib`).
2. Organize it into (at least) these modules, split across **separate files** (not all in one
   `lib.rs`), following the patterns from this chapter:
   - `inventory` — containing:
     - a `pub struct Book` with fields such as `title: String`, `author: String`, and a
       **private** field `copies_in_stock: u32` (use an associated function to construct it,
       similar to the `Breakfast::summer` pattern).
     - a `pub enum Genre` (e.g. `Fiction`, `NonFiction`, `Reference`) — remember enum variants
       don't need individual `pub`.
     - a submodule `catalog` with functions like `add_book`, `find_book` (empty/stub bodies are
       fine, just get the module structure and privacy right).
   - `checkout` — containing functions like `checkout_book`, `return_book`, which need to call
     into `inventory` using a path (your choice: absolute or relative — be ready to justify
     which you picked and why).
3. In your crate root, use `pub use` to **re-export** something from a nested module so that
   external code could call it via a shorter path than its full internal path.
4. Somewhere in `checkout`, use `super` to reach an item defined in its parent module.
5. Write a public function `run_demo()` in the crate root (or in a `bin` if you prefer a binary
   crate wrapping the library) that:
   - Constructs a `Book` using its public constructor function.
   - Attempts to directly set the private `copies_in_stock` field from outside the `inventory`
     module — **comment this line out** and write a one-sentence comment explaining why it
     won't compile.
   - Calls a function from `catalog` using a `use`-shortened path.
   - Calls a function from `checkout`.

### Self-check questions (answer briefly in comments or a short README)

- Which of your module boundaries would break if you moved `checkout` into a new
  `operations` parent module — the absolute paths or the relative ones? Why?
- Where did you choose to make a field private vs. public, and what does that protect?
- Where does the module-tree layout on disk (folders/files) mirror the `mod` declarations in
  your code?

---

## Answer Key — Part 1 (Multiple Choice)

1. C — 2. B — 3. B — 4. C — 5. A — 6. B — 7. B — 8. C — 9. C — 10. C
11. C — 12. C — 13. B — 14. D — 15. D — 16. B — 17. B — 18. B — 19. B — 20. B
21. B — 22. B — 23. B — 24. B — 25. B — 26. B — 27. B — 28. B — 29. B — 30. C

## Answer Key — Part 2 (True/False)

31. True
32. False — a package must contain at least one crate.
33. False — `pub` on a module only lets ancestors refer to the module; contents need their own `pub`.
34. True
35. False — parents can't access a child's private items; only the reverse.
36. True
37. False — `mod` (not `use`) determines what's compiled; `use` only creates in-scope shortcuts.
38. True
39. True — mixing styles across *different* modules is allowed (though discouraged for readability); mixing both styles for the *same* module is an error.
40. False — the glob operator only brings *public* items into scope.