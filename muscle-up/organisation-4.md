# Building a Task Tracker, Step by Step

**A practical, hands-on walkthrough of Chapter 7 concepts.**

You're going to build a small task tracker library, from scratch, in a series of small deliberate steps. At each step I'll tell you:

- **What to type** (commands, files, code)
- **What to expect** (does it compile? error?)
- **What to notice** (the technical concept being demonstrated)

You won't write anything clever. Every line is in service of the chapter's vocabulary: package, crate, module, path, `use`, `pub`, `super`. When something breaks, don't skip ahead — the *breakage is the lesson*.

Let's go.

---

## Step 1 — Create the package

```bash
cargo new task_tracker --lib
cd task_tracker
ls
```

**What you see:** a `Cargo.toml` and a `src/` directory containing `lib.rs`.

**Vocabulary check — say these out loud, don't skim:**

- You just created a **package**. The `Cargo.toml` is what makes it a package.
- The package contains **one library crate**, rooted at `src/lib.rs`.
- The crate is named `task_tracker` — **from the package name**, not from the filename `lib.rs`. (Remember: filenames are convention, crate names come from the package.)

**Notice:** `src/lib.rs` has no `main`. That's what makes it a *library* crate. If there were a `src/main.rs` instead, you'd have a *binary* crate.

---

## Step 2 — Your first module, inline

Open `src/lib.rs`. Delete the default content and write:

```rust
mod tasks {
    #[derive(Debug)]
    pub struct Task {
        pub title: String,
    }
}
```

`cargo build`.

**What happens:** It compiles. No error. But *nothing works yet* — because you haven't tried to use `Task` from outside the module.

**Notice the shape:** `mod tasks { ... }` is an **inline module**. Its code lives inside the braces. There's no separate file yet. A module is a *logical grouping*, and here you're placing it physically inline.

---

## Step 3 — Try to use it (this will fail on purpose)

Still in `src/lib.rs`, *below* the `mod tasks { ... }` block, add:

```rust
pub fn demo() {
    let t = tasks::Task {
        title: String::from("write Rust"),
    };
    println!("{:?}", t);
}
```

Run `cargo build`.

**What happens:** A compiler error. Something like:

```
error[E0603]: struct `Task` is private
```

**Read the error carefully.** It says `Task` is private. But you *did* mark the struct `pub`... right? Look again — actually yes, you did. So why is it private?

**Wait — read the error the compiler gives you in full.** The exact wording matters. If you marked `Task` `pub`, the error is likely about the **module** `tasks` being private, not the struct. Because `mod tasks` has no `pub`. Let me have you verify:

Change the line to:

```rust
mod tasks {
    pub struct Task { pub title: String }
}
pub fn demo() {
    let t = tasks::Task { title: String::from("x") }; // still fails
}
```

**The error now should say:** `module 'tasks' is private`.

**Realize this — the first big lesson of the chapter:**

> `pub` on a *struct* does not matter if the *module containing it* is private to the caller. To reach anything in a module from outside, the module itself must be `pub` first, **and then** the item you want must be `pub`.

**Fix it:** change `mod tasks` to `pub mod tasks`. Rebuild.

**What happens:** Now it compiles.

**Two-layer rule to lock in:** public *path* requires `pub` at **every step** from the caller to the item. Miss any step, and the compiler stops you.

---

## Step 4 — Group more related things (practice the hierarchy)

Inside `mod tasks`, add a second submodule. Change the block to:

```rust
pub mod tasks {
    pub mod state {
        #[derive(Debug)]
        pub enum State {
            Todo,
            Doing,
            Done,
        }
    }

    #[derive(Debug)]
    pub struct Task {
        pub title: String,
        pub state: state::State,
    }
}
```

Update `demo()`:

```rust
pub fn demo() {
    let t = tasks::Task {
        title: String::from("write Rust"),
        state: tasks::state::State::Todo,
    };
    println!("{:?}", t);
}
```

`cargo build`.

**What happens:** Compiles. But **notice how ugly those paths are getting.**

**Realize:** You now have a **module tree**:

```
crate
└── tasks
    ├── state
    │   └── State
    └── Task
```

And the path `tasks::state::State` is *long*. You're going to shorten it in the next step. But first, do the exercise: `cargo build` and *look* at how many times you had to repeat `tasks::state::` in the code. That friction is the motivation for `use`.

---

## Step 5 — Bring paths into scope with `use`

At the top of `lib.rs`, add:

```rust
use tasks::state::State;
```

Rewrite `demo()`:

```rust
pub fn demo() {
    let t = tasks::Task {
        title: String::from("write Rust"),
        state: State::Todo,
    };
    println!("{:?}", t);
}
```

`cargo build`.

**What happens:** Compiles. `State` is now usable by its bare name *in this scope*.

**Notice:** `use` is a **shortcut**, not a declaration. It does not "import" `State` like Python's `import` imports a module. It just tells the compiler: "in this scope, when I say `State`, I mean `tasks::state::State`."

**Exercise:** Move `demo()` inside a new module `pub mod app { pub fn demo() { ... } }` and rebuild. Does the `use` still apply inside `app`? (It shouldn't.) Why? Because **`use` only applies to the scope it's written in.**

**Realize:** You'd have to either put the `use` inside `app` too, or reference via `crate::` / `super::` from within `app`.

---

## Step 6 — `super` and parent-relative paths

Add a function inside `tasks` that wants to call something in its parent (the crate root). First, put a function at the crate root:

```rust
fn log(msg: &str) {
    println!("[log] {msg}");
}
```

Now inside `pub mod tasks`, add:

```rust
pub fn announce(task: &Task) {
    super::log(&format!("task: {}", task.title));
}
```

`cargo build`.

**What happens:** Compiles.

**Notice:** `super::log(...)` — `super` means "parent module." Here, `tasks`'s parent is `crate`, so `super::log` resolves to the crate root's `log`.

**Realize:** Compare with `crate::log(...)`. Both work here. Why would you prefer one?

- `super::` is **relative** — it goes up one level. If you later nest `tasks` inside another module, `super::` points to the new parent, and the call still means "the thing next to me." It *moves with the code*.
- `crate::` is **absolute** — it always points to the crate root, regardless of where the code sits.

Both are valid. The chapter's guidance: use `super::` when the item and caller are *likely to move together*; use `crate::` when they're independent.

**Exercise:** Move `log` into a new module `pub mod logging { pub fn log(...) }` at the crate root. Now update `announce` to call it. Try both `crate::logging::log` and `super::logging::log`. Which compiles? Why does only one?

---

## Step 7 — Struct fields vs. enum variants and `pub`

Change `Task` so its `title` is **not** public, but its `state` is:

```rust
pub struct Task {
    title: String,       // private field
    pub state: state::State,
}
```

**What happens:** `cargo build` fails at the `demo()` site because you construct `Task` with `title` from outside the module.

**Read the error.** It says `field 'title' of struct 'Task' is private`.

**Realize this — the second big lesson:**

> `pub` on a struct makes the *struct name* usable, but **fields stay private by default**, even on a public struct. To make a field public, `pub` it individually.

Why would you do this? To control construction. A private field means outside code **cannot** build a `Task` directly — they must go through a constructor you provide. That's how you enforce invariants (e.g., a title can't be empty).

**Fix it** by adding a constructor inside `impl Task`:

```rust
impl Task {
    pub fn new(title: &str) -> Task {
        Task {
            title: String::from(title),
            state: state::State::Todo,
        }
    }
}
```

And rewrite `demo()`:

```rust
pub fn demo() {
    let t = Task::new("write Rust");
    println!("{:?}", t);
}
```

`cargo build`. Now it compiles.

**Now try enums:** Add an enum inside `tasks`, mark it `pub`, and *don't* mark its variants. Build. It compiles fine.

**Realize the asymmetry:**
- **Struct fields** default to private even on a public struct.
- **Enum variants** default to public when the enum is public.

The reasoning from the chapter: an enum without accessible variants is useless, but a struct is often useful with private fields (as you just saw with `Task`).

---

## Step 8 — Move modules to files (the big one)

Time to stop stuffing everything in `lib.rs`. This is the step where the module/file relationship becomes real.

**Target layout:**

```
src/
├── lib.rs
└── tasks/
    ├── mod.rs        (or use tasks.rs — see below)
    └── state.rs
```

There are two idiomatic styles. **We'll use the modern one first**, then convert, so you feel the difference.

### 8a — Modern style: `tasks.rs` + `tasks/` folder

**Step 1:** In `lib.rs`, replace the entire `pub mod tasks { ... }` block with a single line:

```rust
pub mod tasks;
```

`cargo build`. **It fails.**

**Read the error.** It says something like: `file not found for module 'tasks'`. The compiler is telling you: "You declared a module `tasks`. I looked for its code and couldn't find it."

**Realize:** `mod tasks;` is a **declaration with a semicolon** — "the code is *elsewhere*." You need to create the file.

**Step 2:** Create `src/tasks.rs` and paste the *contents* of the old module body (just the inner parts — the `state` module and `Task` struct):

```rust
// src/tasks.rs
pub mod state {
    #[derive(Debug)]
    pub enum State {
        Todo,
        Doing,
        Done,
    }
}

#[derive(Debug)]
pub struct Task {
    title: String,
    pub state: state::State,
}

impl Task {
    pub fn new(title: &str) -> Task {
        Task {
            title: String::from(title),
            state: state::State::Todo,
        }
    }
}
```

`cargo build`. Now it should compile again. **The module tree is unchanged** — only the physical placement moved.

**Realize:** The file `tasks.rs` *is not the module*. The module is declared by `mod tasks;` in the crate root. The file is just *where the module's code happens to live*. Same module, new home.

### 8b — Extract `state` to its own file

**Step 1:** In `src/tasks.rs`, replace `pub mod state { ... }` with:

```rust
pub mod state;
```

Build. **Fails** again — same style of error.

**Step 2:** Create `src/tasks/state.rs`:

```rust
#[derive(Debug)]
pub enum State {
    Todo,
    Doing,
    Done,
}
```

Now delete the old `state` block if you haven't already. Build.

**What happens:** Compiles.

**Realize the folder rule:** Because `state` is a *child of `tasks`* (not of the crate root), its file goes in the `tasks/` folder — a folder named after its **parent module**. If you'd put `state.rs` in `src/` directly, the compiler would expect `state` to be a top-level module declared in the crate root.

**Exercise (deliberate breakage):** Move `state.rs` to `src/state.rs`. Build. **Read the error.** It says it couldn't find `tasks::state` (or that `state` isn't declared in the crate root). This is the compiler *enforcing the module tree on the filesystem.*

Move it back.

### 8c — Try the older style for one module

Rename `src/tasks.rs` to `src/tasks/mod.rs`.

Build. **It still compiles.** Both are valid:

- `src/tasks.rs` — modern style
- `src/tasks/mod.rs` — older style (from before Rust 2018)

**Realize:** The compiler accepts either. The modern style is preferred because "many `mod.rs` files open in an editor" is confusing. The compiler will complain if you use *both* styles for the *same* module simultaneously.

---

## Step 9 — `use` at the crate root

Now that things are split across files, look how ugly `demo()` is. Update `lib.rs`'s `demo` to use a `use`:

```rust
use tasks::Task;
use tasks::state::State;

pub fn demo() {
    let t = Task::new("write Rust");
    println!("{:?}", t);
}
```

Build. It compiles.

**But wait — `State` isn't used in `demo` anymore. See the warning?**

```
warning: unused import: `tasks::state::State`
```

**Realize:** Rust warns about unused `use`. Remove it. Rebuild. Clean.

**Realize deeper:** `use` doesn't *do* anything except create a shortcut. Unused shortcut = unused import warning. Useful signal.

---

## Step 10 — `pub use` (re-exporting)

Right now, a user of your crate would need to write:

```rust
task_tracker::tasks::Task::new("...")
```

That's fine, but the `tasks` layer is an implementation detail. Let's re-export so they can write `task_tracker::Task`:

In `lib.rs`, add:

```rust
pub use tasks::Task;
```

Build. Now `Task` is available at the crate root **for external users too**.

**Realize:**

- `use tasks::Task;` — *private* shortcut, only usable in this scope.
- `pub use tasks::Task;` — *public* shortcut, forms part of the crate's public API. External code can now say `task_tracker::Task`.

**This is re-exporting.** You're not moving `Task`, you're just exposing a shorter public path. The chapter calls this out specifically for designing clean public APIs — hide internal module structure, expose what users should see.

**Exercise:** Add `pub use tasks::state::State;` and verify from outside (see step 11) that `task_tracker::State` works.

---

## Step 11 — Add a binary crate alongside the library

This is the "one package, multiple crates" concept — in practice.

```bash
mkdir src/bin
```

Create `src/bin/demo.rs`:

```rust
use task_tracker::Task;

fn main() {
    let t = Task::new("ship it");
    println!("{:?}", t);
}
```

Run `cargo run --bin demo`.

**What happens:** It runs. You just used your library crate from a **separate binary crate** inside the *same package*.

**Realize:**

- You now have **one library crate** (`task_tracker`, rooted at `src/lib.rs`) and **one binary crate** (`demo`, rooted at `src/bin/demo.rs`).
- Both live in **one package** (`Cargo.toml`).
- The binary crate sees only the library's **public API** — it uses `Task` via the re-export. It cannot reach `task_tracker::tasks::Task`'s private `title` field, or the `tasks` module if you hadn't made it `pub`.

**Try to break it:** In `demo.rs`, write `task_tracker::tasks::state::State::Todo` and use it directly. Does it compile? (It should, since `tasks` and `state` are `pub`.) Now try to touch `t.title`. Compile error — `title` is private. **The binary crate is a *user* of the library, subject to the same privacy rules as anyone else.**

**Exercise:** Add a second binary `src/bin/stats.rs`. Have it print something (e.g., the number of enum variants, or just a static message). Run `cargo run --bin stats`. Confirm both binaries coexist.

**Realize:** *This* is why a package can have many binaries — each is a separate entry point, but they share the same library. If `src/lib.rs` didn't exist, this pattern would be impossible.

---

## Step 12 — Idiomatic `use` style, and a conflict

The chapter says: idiomatic `use` brings **modules** for functions, but **full paths** for types.

Add a helper module and function:

```rust
// src/lib.rs
pub mod helpers {
    pub fn slugify(s: &str) -> String {
        s.to_lowercase().replace(' ', "-")
    }
}
```

Now from a hypothetical use site, the chapter says: write `use task_tracker::helpers;` then call `helpers::slugify(...)`. **Not** `use task_tracker::helpers::slugify;`.

**Realize the convention:**
- **Function** → import its **parent module** so you write `parent::function()`. This makes it visually obvious the function isn't locally defined.
- **Type** (struct, enum, trait) → import the **full path** so you write `Type` bare. This is what you did with `Task` and `State`.

**Now the conflict case:** bring two `Result` types into scope:

```rust
use std::fmt::Result;
use std::io::Result;
```

Build. **Compile error** — two `Result` in the same scope.

**Fix with `as`:**

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

Build. Clean.

**Realize:** Name collisions within a scope are resolved with `as` (rename), or by not importing the leaf — importing the parent module instead and using `parent::Result`.

---

## Step 13 — Glob operator (and why to avoid it)

Add temporarily:

```rust
use std::collections::*;
```

Build.

**Realize:** This brings every public item in `std::collections` into scope. It compiles, but:

- You can't tell where a name came from by reading the code.
- Adding a new item to `std::collections` in a future Rust version could shadow something and change meaning silently.

**Remove it.** The chapter calls it out as a footgun, acceptable mainly in tests.

---

## Step 14 — Final: verify the mental model

Run:

```bash
cargo build
cargo run --bin demo
cargo run --bin stats
```

Then answer these **without looking at the chapter**:

1. What is the **package** here, and what file makes it one?
2. How many **crates** are in this package? Name each.
3. Where is each crate's **root**?
4. Draw the **module tree** for the library crate.
5. Why does `demo.rs` have to `use task_tracker::Task` instead of `use tasks::Task`? (It's a **different crate** — it can't see your crate's internal module tree. It only sees the public API.)
6. Where would you put a new module `ui` that's a **child of `tasks`**? Where if it's a **child of the crate root**?
7. Why can this package have two binaries but only **one library**?

If you can answer all seven, you own this chapter.

---

## Wrap-Up Checklist

By the time you finish, you will have *practically* experienced:

| Concept | Where you hit it |
|---|---|
| Package vs. crate | Step 1 |
| Library crate | Step 1 |
| Inline module | Step 2 |
| `pub` at module vs. item level | Steps 3, 7 |
| Module tree | Steps 4, 14 |
| Absolute vs. relative paths | Step 6 |
| `use` as a scope-bound shortcut | Step 5 |
| `super` | Step 6 |
| Private struct fields, public enum variants | Step 7 |
| Extracting modules to files | Step 8 |
| Modern vs. `mod.rs` style | Step 8c |
| `pub use` (re-export) | Step 10 |
| Multiple binary crates in a package | Step 11 |
| Idiomatic `use` (functions vs. types) | Step 12 |
| `as` aliasing | Step 12 |
| Glob operator | Step 13 |

The errors you hit along the way — the `E0603`s, the `file not found for module`, the `two Result types in the same scope` — are the *real* chapter. The prose in the book describes rules; the compiler is the referee. You just played the game.