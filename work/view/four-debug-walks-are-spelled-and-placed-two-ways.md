---
id: four-debug-walks-are-spelled-and-placed-two-ways
kind: issue
title: the PR adds std::fmt walks to a crate whose other fmt impls are core::fmt, and leaves DocSession's walk 1750 lines from its declaration
status: open
opened: 2026-09-06
refs: [2093]
---



Found by the style review of #2093.

The PR writes four walks against one rule and spells them two ways,
in one diff.

## `std::fmt` in a crate that says `core::fmt`

The two impls it adds to `session.rs` are `std::fmt`
(`crates/viewer/src/session.rs:314-315`, `:429-430`); the sibling it
adds in the same PR is `core::fmt`
(`crates/viewer/src/pickcache.rs:170-171`). Across
`crates/viewer/src/`, `core::fmt::` appears 68 times and `std::fmt::`
14 — and 6 of those 14 are the three walks in `session.rs`. Outside
`session.rs` the only `std::fmt` sites are `prefs.rs:98`, `:132`,
`:304` and `drafts.rs:390`; every other `fmt` impl in the crate,
32 `Display`s included, is `core::fmt`.

Nothing decides between them for a crate that is not `no_std`, so this
is taste, not a defect. What makes it worth a row is that the PR chose
both, in one change, for four impls it is presenting as one mechanism
— and the one that took the crate's majority spelling is the sibling,
not the two it wrote from scratch. `session.rs:1952` (`DocSession`,
`std::fmt`) is pre-existing and is what the two new ones matched.

## Three walks sit beside their declarations and the fourth does not

`Derived`'s walk is at `session.rs:302-333`, three lines below
`Derived::none` (which ends at `:299`); `LandedRun`'s is at `:421-448`, directly below its
struct; `PickCache`'s is at `pickcache.rs:163-186`, directly below
its struct. `DocSession`'s is at `session.rs:1941-1974` — the last
thing in a 1974-line file, 1,779 lines below the declaration at `:173`,
under three unrelated free functions (`assembly_shaped`, `badge`,
`session_dir`).

The property the PR is buying is that **a reader adding a field is
made to visit the walk**. The compiler enforces that for all four, so
nothing is broken. But the whole point of moving from
`finish_non_exhaustive`-as-a-shrug to a compiler-held list is to make
the decision visible at the moment it is made, and for the largest of
the four values the decision lives at the far end of the file. The PR
rewrote that impl; it did not move it.
