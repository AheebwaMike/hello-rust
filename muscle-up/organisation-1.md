# Rust Module System — Hands-On Practice Guide

> Based on *The Rust Book*, Chapter 7: Managing Growing Projects with Packages, Crates, and Modules.
> Only uses concepts you've covered so far: datatypes, ownership, structs, enums, pattern matching, and this chapter.

---

## The Project: A Mini Music Library

You'll build a small **library crate** that models a music streaming service. It's small enough to finish, but has enough "parts" that you'll be forced to make real decisions about modules, paths, `pub`, and file organization.

**Conceptual structure** (the domain, in plain English):

- A **catalog** of songs, organized into **genres**
- A **player** that can play a song and report what's playing
- A **user** with a name and a current playlist

Enough surface area for a nested module tree, but nothing you haven't already learned.

---

## Step 0 — Create the package

```bash
cargo new music_lib --lib
cd music_lib
```

**Checkpoint:** You now have a **package** (`Cargo.toml`) containing one **library crate** rooted at `src/lib.rs`. Confirm for yourself:

- The package name is `music_lib`.
- The crate is a library crate (no `main`), rooted at `src/lib.rs`.
- The crate inherits the package's name.

Before moving on, say out loud (or write in a comment) the answer to: *"What's the difference between the package and the crate here?"* If you can't, re-read your notes.

---

## Step 1 — Modules inline, in one file

Start with everything in `src/lib.rs`, modules **inline**. This isolates modules from files so you focus on **module tree + visibility** first.

```rust
// src/lib.rs

mod catalog {
    pub mod genres {
        #[derive(Debug)]
        pub enum Genre {
            Rock,
            Jazz,
            Electronic,
        }

        pub fn all() -> Vec<Genre> {
            vec![Genre::Rock, Genre::Jazz, Genre::Electronic]
        }
    }

    #[derive(Debug)]
    pub struct Song {
        pub title: String,
        pub artist: String,
        pub genre: genres::Genre,
    }

    impl Song {
        pub fn new(title: &str, artist: &str, genre: genres::Genre) -> Song {
            Song {
                title: String::from(title),
                artist: String::from(artist),
                genre,
            }
        }
    }
}
```

**Exercises:**

1. Draw the **module tree** on paper. It should be:
   ```
   crate
   └── catalog
       ├── genres
       │   ├── Genre (enum)
       │   └── all (fn)
       └── Song (struct)
           └── new (impl fn)
   ```
2. From inside `catalog::genres::all`, write the **absolute path** to `Song`. (Hint: start with `crate::`.)
3. From inside `Song::new`, write the **relative path** to `Genre`.
4. Try to access `catalog::Song` from a *test function at the crate root* (inside `lib.rs`, outside any module). Does it compile? Why or why not?

---

## Step 2 — Practice `pub` at each level

Now deliberately break visibility and fix it. This is the point of the chapter.

**Exercises:**

1. Remove `pub` from `mod genres`. Try to use `catalog::genres::Genre` from the crate root. **Read the compiler error carefully.** What does it say is private?
2. Put `pub` back, but remove `pub` from `Genre`. Try to construct a `Genre` from the crate root. Again, read the error.
3. Remove `pub` from `Song.title`. Try to read `song.title` from outside `catalog`. Read the error.
4. Now the key question: **why does `Song` need `pub` on *both* the struct and each field you want to touch?** Write a one-sentence answer in a comment.

**Rule to internalize:** `pub` on a module only lets you *reach the module*. It does not make the module's contents public. Each item inside needs its own `pub`.

---

## Step 3 — `super` and the parent/child relationship

Add a `fix` function that demonstrates `super`.

```rust
// still inside src/lib.rs, at the crate root

fn log(msg: &str) {
    println!("[log] {msg}");
}

mod player {
    pub fn play(title: &str) {
        // Call log, which lives in the parent module.
        super::log(&format!("playing {title}"));
    }
}
```

**Exercises:**

1. Why does `super::log(...)` work but `log(...)` alone does not?
2. Rewrite the call using an **absolute path** instead of `super`. Which feels better here, and why?
3. Move `log` into a new module `logging` at the crate root. Now fix the call inside `player::play`. Did you use `super`, a relative path, or `crate::logging::log`? Justify your choice in a comment.

---

## Step 4 — Add `use` and feel the cleanup

You've probably been writing long paths repeatedly. Now shorten them.

```rust
// src/lib.rs (top of file)

use crate::catalog::Song;
use crate::catalog::genres::Genre;
```

**Exercises:**

1. Rewrite the `play` function from Step 3 so it takes a `&Song` instead of a `&str`, and prints `song.title` and `song.genre`. Use the `use` shortcuts.
2. Now add a **nested-path** `use` that combines those two lines into one.
3. Add an alias: bring in `std::fmt::Result` and `std::io::Result` both, using `as` on one of them. (You don't need to use them yet — just prove the two can coexist.)

**Rule:** idiomatic `use` for *functions* brings the **parent module** into scope (`use ...::hosting;` then call `hosting::add_to_waitlist()`). Idiomatic `use` for *structs/enums* brings the **full path** (`use std::collections::HashMap;`). Follow this convention; note where you deviate.

---

## Step 5 — `pub use` (re-exporting)

Right now, external users of your crate would have to write `music_lib::catalog::genres::Genre` — awkward. Fix it.

```rust
// src/lib.rs

pub use crate::catalog::genres::Genre;
pub use crate::catalog::Song;
```

**Exercises:**

1. After adding these, from the crate root, is `crate::Genre` a valid path? Test it.
2. What's the *external* user's new shortest path to `Genre`? Write it in a comment.
3. Why is this called **re-exporting**? (Hint: it's a `use` **plus** `pub` — the shortcut itself becomes public.)

---

## Step 6 — Split into files (the real test)

Now move each module to its own file. The compiler errors here are the most instructive in the whole chapter.

**Target layout:**

```
src/
├── lib.rs
├── catalog.rs
└── catalog/
    └── genres.rs
```

**Do this in order, compiling after each move:**

1. **Extract `catalog` first.**
   - In `lib.rs`: replace the entire `mod catalog { ... }` block with `pub mod catalog;`
   - Create `src/catalog.rs` containing the old *body* of `catalog`.
   - Add a `use crate::catalog::genres;` inside `catalog.rs` if needed.
   - `cargo build` — fix errors until clean.

2. **Extract `genres` next.**
   - In `src/catalog.rs`: replace `pub mod genres { ... }` with `pub mod genres;`
   - Create `src/catalog/genres.rs` with the old body.
   - `cargo build` — fix errors.

**Exercises:**

1. When you extracted `genres`, why did the file go in `src/catalog/` and not `src/`? Write the rule.
2. Deliberately break it: move `genres.rs` to `src/genres.rs` and rebuild. Read the compiler error. What does it tell you about the module tree?
3. Try the **older style**: rename `src/catalog/genres.rs` to `src/catalog/genres/mod.rs` and adjust. Does it still compile? What warning if any do you see?

---

## Step 7 — Add a binary crate alongside the library

The chapter talks about packages with **both** a library and a binary crate. Add one.

```bash
mkdir -p src/bin
```

Create `src/bin/demo.rs`:

```rust
// src/bin/demo.rs

use music_lib::{Genre, Song};
// or music_lib::catalog::genres::Genre, depending on your re-exports.

fn main() {
    let song = Song::new("Blue in Green", "Miles Davis", Genre::Jazz);
    println!("{:?}", song);
}
```

**Exercises:**

1. Run `cargo run --bin demo`. Does it work?
2. Run `cargo build`. What two crates did Cargo build? (One library, one binary — both from the same package.)
3. Try to access `music_lib::catalog::Song` *directly* from `demo.rs` **without** using the re-exports. Does the full path work? Should it, given your `pub use` statements? Explain.
4. Add a second binary: `src/bin/stats.rs`. Have it print the number of genres. Confirm `cargo run --bin stats` works and both binaries coexist.

**This step cements:** one package → one library crate + multiple binary crates, all sharing the library's public API.

---

## Step 8 — Final consolidation exercise

Add a `User` type that pulls everything together:

```rust
// somewhere sensible in your module tree — you decide where
pub struct User {
    pub name: String,
    playlist: Vec<Song>,   // private field on purpose
}

impl User {
    pub fn new(name: &str) -> User { /* ... */ }
    pub fn add_to_playlist(&mut self, song: Song) { /* ... */ }
    pub fn playlist_len(&self) -> usize { self.playlist.len() }
}
```

**Exercises:**

1. Where did you put `User`? Justify your module choice based on **cohesion**, not convenience.
2. Why is `playlist` private while `name` is public? What does this model?
3. From `src/bin/demo.rs`, construct a `User`, add a couple of songs, and print `playlist_len()`. Confirm you *cannot* touch `user.playlist` directly — read the error.
4. Add a module `auth` with a private helper `fn validate_name(s: &str) -> bool`. Re-export nothing from `auth`. Confirm external code has no way to call `validate_name`. If you can't break in from a binary crate, you've done it right.

---

## Self-Check Checklist

Before declaring the chapter done, you should be able to answer **all** of these without looking:

- [ ] What is a package, and what single file makes something a package?
- [ ] What is a crate, and how is it different from a package?
- [ ] What is a crate *root*, and where does it live for a library vs. a binary crate?
- [ ] Why does the crate's **name** come from the package rather than the file name?
- [ ] What are the three places the compiler looks for a module declared with `mod foo;`?
- [ ] What's the difference between `mod` and `use`?
- [ ] Why doesn't `pub mod foo` make `foo`'s *contents* public?
- [ ] What's the difference between an **absolute** and a **relative** path? When do you prefer each?
- [ ] What does `super` refer to, and when is it useful?
- [ ] Why are enum variants public by default, but struct fields private by default?
- [ ] What does `pub use` do, and why is it called *re-exporting*?
- [ ] What's the idiomatic `use` pattern for functions vs. structs/enums?
- [ ] What does the glob operator (`*`) do, and why is it a footgun?
- [ ] What's the difference between `src/foo.rs` and `src/foo/mod.rs`?
- [ ] Why can a package have many binary crates but at most one library crate?

If **any** box is unchecked, go back to that step before proceeding.

---

## Notes on What You're *Not* Using

To stay within your current knowledge, this guide deliberately avoids:

- traits (you'll need them for `impl Display`, but `#[derive(Debug)]` covers you here)
- generics and lifetimes beyond what structs already require
- collections beyond `Vec` and `String` (already covered)
- error handling with `Result` in any non-trivial way
- closures, iterators, or async

Everything here uses only: **datatypes, ownership, structs, enums, pattern matching, and the module system.**

---

