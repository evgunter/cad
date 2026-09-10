---
id: four-debug-walks-are-spelled-and-placed-two-ways
kind: issue
title: DocSession's Debug walk sits 1,780 lines below its declaration while the other three sit beside theirs
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
`session.rs` the only `std::fmt` sites are `prefs.rs:106`, `:140`,
`:322` and `drafts.rs:390`; every other `fmt` impl in the crate,
32 `Display`s included, is `core::fmt`.

Nothing decides between them for a crate that is not `no_std`, so this
is taste, not a defect. What makes it worth a row is that the PR chose
both, in one change, for four impls it is presenting as one mechanism
— and the one that took the crate's majority spelling is the sibling,
not the two it wrote from scratch. `session.rs:1952` (`DocSession`,
`std::fmt`) is pre-existing and is what the two new ones matched.

## Three walks sit beside their declarations and the fourth does not

`Derived`'s walk is at `session.rs:302-331`, three lines below
`Derived::none` (which ends at `:299`); `LandedRun`'s is at
`:419-460`, directly below its struct; `PickCache`'s is at
`pickcache.rs:163-195`, directly below its struct. `DocSession`'s is
at `session.rs:1953-2000` — the last thing in a 2,000-line file, 1,780
lines below the declaration at `:173`, under three unrelated free
functions (`assembly_shaped`, `badge`, `session_dir`). Each range runs
from the walk's doc comment to its closing brace, and the distance is
that start minus the declaration line; all five numbers were
re-derived by subject on 2026-09-08 (four of them were already stale
before that pass, and the `std::fmt` citations in the section above
are left as written because they name lines in #2093's diff, not in
the tree).

The property the PR is buying is that **a reader adding a field is
made to visit the walk**. The compiler enforces that for all four, so
nothing is broken. But the whole point of moving from
`finish_non_exhaustive`-as-a-shrug to a compiler-held list is to make
the decision visible at the moment it is made, and for the largest of
the four values the decision lives at the far end of the file. The PR
rewrote that impl; it did not move it.

## The spelling half is answered on #2093; the placement half is this file's live subject

All four walks are `core::fmt` now, the crate's majority spelling: the
two the PR wrote from scratch in `session.rs` and `DocSession`'s
pre-existing one, which is what they had matched. `session.rs` keeps
`std::fmt` nowhere.

**The placement is declined, deliberately.** `DocSession`'s walk still
sits at the end of the file, 1,780 lines below its declaration, under
three unrelated free functions. Moving it is a pure move inside a PR
whose warrant is a mechanism change, and this program spent 2026-09-06
learning what mixing those costs. Nothing is broken — the compiler
enforces the visit for all four — so this is a readability row and it
waits for a pass that is allowed to move code.

That move is what this file is now about.
