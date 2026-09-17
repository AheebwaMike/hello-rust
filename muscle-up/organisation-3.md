# Practical Guide: Packages, Crates & Modules — Build `toolshed`

We're going to build a tiny **tool-lending library** package called `toolshed`, one piece at a
time. At each step you'll run a real command, sometimes hit a real compiler error on purpose,
and see exactly why Rust is complaining before fixing it. Follow along in your own terminal —
don't skip the "break it on purpose" steps, that's where the concept actually sticks.

Only concepts up through this chapter (plus ownership/structs/enums from earlier chapters) are
used — no traits, generics, or collections beyond what you already know.

---

## Step 1 — Create the package

```bash
cargo new toolshed --lib
cd toolshed
```

Run `ls` and `ls src`. You'll see:

```
toolshed
├── Cargo.toml
└── src
    └── lib.rs
```

**What just happened:** `cargo new --lib` created a **package** — a directory with a
`Cargo.toml` that Cargo uses to build one or more **crates**. Because you passed `--lib`, this
package holds exactly one **library crate**, and Cargo already knows `src/lib.rs` is its
**crate root** — the file the compiler starts reading from. Notice `Cargo.toml` never mentions
`src/lib.rs` by name; that's pure convention, not configuration.

A library crate has no `main` function and doesn't produce an executable — it exists to be
*used* by other code, which is exactly what we'll do near the end of this guide.

---

## Step 2 — Declare your first module (and break it on purpose)

Open `src/lib.rs`, delete the generated content, and write:

```rust
mod front_desk {
    fn checkout_tool(tool_name: &str) {
        println!("Checked out: {tool_name}");
    }
}

pub fn open_shop() {
    front_desk::checkout_tool("hammer");
}
```

Run:

```bash
cargo build
```

**You should get an error** — something like:

```
error[E0603]: function `checkout_tool` is private
```

**Why:** In Rust, every item — functions, structs, enums, modules — is **private to its parent
module by default**. `checkout_tool` lives inside `front_desk`, and `open_shop` lives outside
`front_desk` (they're actually *siblings*, both defined directly in the crate root). Being
siblings is why Rust even let you write the path `front_desk::checkout_tool` without
complaining that `front_desk` itself doesn't exist — but the function inside it is still
sealed off. This is Rust deliberately defaulting to "hidden implementation details," the same
philosophy behind private struct fields.

---

## Step 3 — Fix it halfway (and watch it still fail)

Add `pub` to the **module** only, not the function yet:

```rust
pub mod front_desk {
    fn checkout_tool(tool_name: &str) {
        println!("Checked out: {tool_name}");
    }
}
```

Run `cargo build` again. **Still an error** — same `checkout_tool` is private message.

**Why:** `pub` on a module doesn't cascade. It only means *"ancestor modules are now allowed to
refer to this module by name."* It says nothing about what's inside. Making a module public is
like unlocking the front door of a building — you still can't walk into any locked office
inside it.

---

## Step 4 — Fix it for real

Now mark the function public too:

```rust
pub mod front_desk {
    pub fn checkout_tool(tool_name: &str) {
        println!("Checked out: {tool_name}");
    }
}

pub fn open_shop() {
    front_desk::checkout_tool("hammer");
}
```

`cargo build` should now succeed.

**What changed:** `open_shop` can see `front_desk` (siblings), `front_desk` is `pub` so its name
is visible, and `checkout_tool` is `pub` so its contents are reachable. All three checks had to
pass.

Try rewriting the call as an **absolute path** instead:

```rust
crate::front_desk::checkout_tool("hammer");
```

Both compile identically here, because we're calling from the crate root itself. The
difference only matters once code moves around — `crate::...` always starts from the root
(like `/` in a filesystem), while `front_desk::...` is relative to *where the calling code
currently sits* (like a relative filesystem path).

---

## Step 5 — Add a struct with a private field (another privacy lesson)

Add a second module, `inventory`, with a public struct that has one private field:

```rust
pub mod inventory {
    pub struct Tool {
        pub name: String,
        condition: u8, // private: 0-100, internal detail
    }
}
```

Now try, inside `open_shop`, to build one directly:

```rust
pub fn open_shop() {
    front_desk::checkout_tool("hammer");

    let hammer = inventory::Tool {
        name: String::from("Hammer"),
        condition: 95,
    };
}
```

`cargo build` fails again — this time with something like:

```
error[E0451]: field `condition` of struct `Tool` is private
```

**Why:** Marking `Tool` itself `pub` only makes the *struct* public — same rule as with
modules, visibility doesn't cascade. Each field needs its own decision. `name` is `pub`, so
outside code can read/write it directly; `condition` is not, so it's sealed. This is
deliberate: `condition` is an internal detail the `inventory` module wants to control.

---

## Step 6 — Provide a public constructor

Since outside code can't set `condition` directly, give `inventory` a public associated
function to build a `Tool` correctly:

```rust
pub mod inventory {
    pub struct Tool {
        pub name: String,
        condition: u8,
    }

    impl Tool {
        pub fn new(name: &str) -> Tool {
            Tool {
                name: String::from(name),
                condition: 100, // every tool starts brand new
            }
        }
    }
}
```

Update `open_shop`:

```rust
let hammer = inventory::Tool::new("Hammer");
```

This now compiles. Notice the pattern: **a public struct with a private field almost always
needs a public constructor**, because outside code has no other legal way to produce a valid
instance.

*(Side note, no code needed: if `inventory` instead had `pub enum ToolCategory { Power, Hand,
Measuring }`, every variant would be public automatically the moment the enum is — enums don't
require per-variant `pub` the way structs require per-field `pub`. Feel free to add this enum
as a `category: ToolCategory` field on `Tool` for practice.)*

---

## Step 7 — Nested modules and `super`

Inside `front_desk`, add a private submodule `checkout` that needs to call a *parent*
function. This models "the checkout desk logs every transaction back at the front desk."

```rust
pub mod front_desk {
    pub fn checkout_tool(tool_name: &str) {
        println!("Checked out: {tool_name}");
    }

    fn log_transaction(tool_name: &str) {
        println!("[log] {tool_name} left the building");
    }

    mod checkout {
        pub fn process(tool_name: &str) {
            super::log_transaction(tool_name);
        }
    }
}
```

**Why this compiles:** `checkout` is a *child* of `front_desk`, and children can always see
their ancestors' private items (the reverse is never true — a parent can't reach into a
child's private internals). `super` means *"start from my parent module"* — the same idea as
`..` in a filesystem path. We didn't need `pub fn log_transaction` because `checkout` doesn't
need outside permission to see upward into its own family.

Go ahead and run `cargo build` — you'll likely get an *"unused function"* warning for
`checkout::process` since nothing calls it yet. That's fine; warnings aren't errors.

---

## Step 8 — Split modules into separate files

Right now everything lives in one `lib.rs`. Real projects split modules out. Let's move
`front_desk` to its own file.

1. In `src/lib.rs`, replace the whole `pub mod front_desk { ... }` block with just:

   ```rust
   pub mod front_desk;

   pub fn open_shop() {
       front_desk::checkout_tool("hammer");
       let hammer = inventory::Tool::new("Hammer");
   }

   pub mod inventory {
       pub struct Tool {
           pub name: String,
           condition: u8,
       }

       impl Tool {
           pub fn new(name: &str) -> Tool {
               Tool { name: String::from(name), condition: 100 }
           }
       }
   }
   ```

2. Create `src/front_desk.rs` with the code that *used to be inside the curly braces*:

   ```rust
   pub fn checkout_tool(tool_name: &str) {
       println!("Checked out: {tool_name}");
   }

   fn log_transaction(tool_name: &str) {
       println!("[log] {tool_name} left the building");
   }

   pub mod checkout {
       pub fn process(tool_name: &str) {
           super::log_transaction(tool_name);
       }
   }
   ```

Run `cargo build`. It should still compile, unchanged in behavior.

**Why this works without touching any call sites:** `mod front_desk;` (note the semicolon, no
braces) tells the compiler *"the body of this module lives in a file, go find it."* It checks
`src/front_desk.rs` (and would also accept the older `src/front_desk/mod.rs` style — pick one
convention and stick to it project-wide). Nothing about *how you call* `front_desk::...` had to
change, because `mod` only affects where the compiler *finds* code — it has nothing to do with
paths.

Now split `checkout` out too, since it's a submodule of a module that now lives in its own
file:

1. In `src/front_desk.rs`, replace `pub mod checkout { ... }` with just:
   ```rust
   pub mod checkout;
   ```
2. Create a **directory** `src/front_desk/` and put the checkout code in
   `src/front_desk/checkout.rs`:
   ```rust
   pub fn process(tool_name: &str) {
       super::log_transaction(tool_name);
   }
   ```

**Why the directory name matters:** for a submodule of `front_desk`, the compiler looks inside
a directory named after its parent — `src/front_desk/checkout.rs`. This is the same rule as
Step 1's crate root, just one level deeper: the on-disk folder structure mirrors the module
tree.

`cargo build` should succeed again.

---

## Step 9 — `use`, and a scope trap

Every call so far has spelled out `front_desk::checkout_tool`. Shorten it:

```rust
use front_desk::checkout_tool;

pub fn open_shop() {
    checkout_tool("hammer");
    let hammer = inventory::Tool::new("Hammer");
}
```

This compiles — `use` just creates a shortcut name for the current scope.

Now, on purpose, create the classic mistake. Add a new child module and move the call into it:

```rust
use front_desk::checkout_tool;

mod shop_ui {
    pub fn run() {
        checkout_tool("hammer"); // <-- moved in here
    }
}
```

`cargo build` fails:

```
error[E0433]: failed to resolve: use of undeclared crate or module `checkout_tool`
```

plus a warning that the original `use` is now unused.

**Why:** `use` only creates its shortcut **in the scope where it's written** — it is *not*
inherited by child modules, the same way a local variable isn't visible outside its block. Fix
it either by moving the `use` line inside `shop_ui`, or, to reuse the outer one, writing
`super::checkout_tool("hammer");` inside `shop_ui::run`. Try both and confirm each one compiles
— you should see the actual behavior, not just take it on faith.

Once you're done experimenting, you can delete the `shop_ui` module or keep it — it isn't
needed for later steps.

---

## Step 10 — Idiomatic `use`, and re-exporting with `pub use`

Two style rules worth internalizing by doing, not just reading:

**Functions:** bring in the *parent module*, then call `module::function()` — this is what you
already did with `checkout_tool` implicitly avoided; try changing it to the idiomatic form:

```rust
use front_desk::checkout;
// call as: checkout::process("hammer");
```

instead of `use front_desk::checkout::process;` followed by a bare `process(...)`. The
idiomatic form makes it obvious at the call site that the function isn't locally defined.

**Types (structs/enums):** the opposite convention — bring in the full path to the type
itself:

```rust
use inventory::Tool;
// construct as: Tool::new("Hammer")
```

instead of stopping at the module. Try it and confirm `cargo build` still passes.

**Re-exporting:** right now, anyone depending on your `toolshed` crate has to write
`toolshed::inventory::Tool`. If you'd rather expose a flatter public API, re-export it from the
crate root with `pub use`:

```rust
pub use inventory::Tool;
```

This doesn't move or duplicate the type — it just makes `toolshed::Tool` *also* a valid public
path to the exact same struct, in addition to the original. This is purely an API-design tool;
internal code can stay organized however makes sense to you while external users get a
simpler surface.

---

## Step 11 — A quick nested-path / naming-collision detour

Add this function anywhere in `front_desk.rs`, to see two real standard-library items with the
same name coexist:

```rust
use std::fmt;
use std::io::{self, Write};

pub fn describe(tool_name: &str) -> fmt::Result {
    Ok(())
}

pub fn save_receipt(tool_name: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "Receipt for: {tool_name}")
}
```

`cargo build` should succeed (you'll need `use std::io::Write;` in scope for `writeln!` to
work on a handle, which the nested `{self, Write}` import already gives you).

**Why no conflict:** `std::fmt::Result` and `std::io::Result` are two *different* types that
happen to share a name. Because we brought in the **parent modules** (`fmt` and `io`) rather
than the types directly, we refer to them unambiguously as `fmt::Result` and `io::Result`. If
you'd written `use std::fmt::Result; use std::io::Result;` in the same scope, that's a compile
error — Rust won't let two same-named items share a scope. The fix in that case would be
`use std::io::Result as IoResult;` to rename one on the way in.

*(One more thing worth knowing but not doing: `use std::collections::*;` — the glob operator —
pulls in every **public** item from a path at once. Handy in tests, risky in regular code
because it obscures where a name came from.)*

---

## Step 12 — Give the library a binary front-end

So far `toolshed` is library-only. Packages commonly ship a thin binary crate that's just a
client of the library's own public API — proving your API is actually usable from the outside.

Create `src/main.rs` alongside `src/lib.rs`:

```rust
use toolshed::Tool;

fn main() {
    toolshed::open_shop();
    let hammer = Tool::new("Hammer");
    println!("Ready to lend: {}", hammer.name);
}
```

Run:

```bash
cargo build
cargo run
```

**Why `toolshed::` and not `crate::`:** from `main.rs`'s point of view, the library crate is an
*external* dependency, referred to by the package name, exactly the way any other crate would
be. `main.rs` can only reach what `lib.rs` marked `pub` — it has no special access, even though
it lives in the same package. That's the whole point of the split: your binary is forced to be
just another *user* of your public API, which is a good way to catch an API that's awkward to
use from the outside.

---

## What you actually exercised

By the end of this guide your `toolshed` package contains: one library crate and one binary
crate; a multi-file module tree (`src/lib.rs`, `src/front_desk.rs`,
`src/front_desk/checkout.rs`); both absolute (`crate::...`) and relative paths; `super` for
reaching a parent from a child; a public struct with a mixed public/private field set and a
constructor; an enum you were pointed at but built yourself; scoped `use` (including watching
it *fail* to leak into a child module); the idiomatic `use` conventions for functions vs.
types; a `pub use` re-export; a resolved same-name collision via parent-module paths; and a
binary crate consuming its sibling library crate through its public API only.

From here, the book moves on to collections (`Vec`, `String`, `HashMap`) — a natural next step
once your module organization habits feel automatic rather than deliberate.
