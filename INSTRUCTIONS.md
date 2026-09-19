# How to Add Content to the Book

Personal notes for maintaining `docs/` (the mdBook published at
https://aheebwamike.github.io/hello-rust/).

---

## The mental model

- **Source files** live in `docs/src/` — this is what you edit.
- **`docs/src/SUMMARY.md`** is the table of contents. A page will NOT appear
  in the sidebar unless it's listed here.
- **`docs/book/`** is generated output. Never edit it, never commit it.
- **Push to `main`** → GitHub Actions rebuilds and redeploys automatically.

---

## The routine (adding one page)

### 1. Create the file

Put it in `docs/src/` (or a subfolder). Example:

    docs/src/journal/2026-09-19-first-entry.md

### 2. Add it to `docs/src/SUMMARY.md`

Paths are relative to `docs/src/`. Example:

    # Summary

    - [Introduction](./introduction.md)
    - [Why I'm Learning Rust](./why-rust.md)
    - [How This Repo Is Organized](./repo-tour.md)

    ---

    # Journal

    - [Journal Index](./journal/README.md)
    - [2026-09-19 — First Entry](./journal/2026-09-19-first-entry.md)

    ---

    # Guide

    - [Ownership: The First Wall](./guide/ownership.md)

### 3. Preview locally (optional)

    cd docs
    mdbook serve

Open http://localhost:3000 — it hot-reloads on save.

### 4. Commit and push

    git add docs/
    git commit -m "Add journal entry: first entry"
    git push

### 5. Wait ~30 seconds

The Action runs, goes green, and the site updates at
https://aheebwamike.github.io/hello-rust/

---

## Adding a whole new section

1. Create a folder inside `docs/src/` (e.g. `docs/src/projects/`).
2. In `SUMMARY.md`, add a section heading as its own line:

       # Projects

   Lines starting with `#` that are NOT list items are section separators,
   not pages.
3. List the pages under it.

---

## Rules to remember

1. If it's not in `SUMMARY.md`, it doesn't exist in the sidebar — even if
   the file is present.
2. Never edit `docs/book/` — it gets overwritten on every build.
3. Never commit `docs/book/` — it's in `.gitignore` (line 17).

---

## File naming convention

For journal entries, use a date prefix so they sort naturally:

    YYYY-MM-DD-short-slug.md

Examples:

    2026-09-19-first-entry.md
    2026-09-25-borrow-checker-fight.md
    2026-10-02-guessing-game-done.md

Guide chapters are thematic (no date), since they get edited over time:

    ownership.md
    borrowing.md
    structs-and-enums.md

---

## Troubleshooting

**Page doesn't show up in the sidebar.**
→ It's not listed in `SUMMARY.md`. Add it.

**Build fails on GitHub Actions.**
→ Check the Actions tab. Usually a typo in `SUMMARY.md` pointing at a
   file that doesn't exist.

**Local preview shows old content.**
→ `mdbook serve` should hot-reload; if not, Ctrl+C and restart it.

**Theme looks wrong.**
→ mdBook stores your theme choice in browser localStorage. Open in an
   incognito window, or clear site data in DevTools → Application →
   Local Storage.

---

## Where things live

    hello-rust/
    ├── README.md                  # Repo overview
    ├── INSTRUCTIONS.md            # This file
    ├── hello, rust/               # Learning sandbox
    ├── kata/                      # Challenges by difficulty
    ├── muscle-up/                 # Study notes
    ├── programs/                  # Standalone mini-projects
    └── docs/                      # The book
        ├── book.toml              # mdBook config
        └── src/                   # ← EDIT THESE
            ├── SUMMARY.md         # Table of contents
            ├── introduction.md
            ├── why-rust.md
            ├── repo-tour.md
            ├── journal/
            │   └── README.md
            └── guide/
                └── ownership.md
