---
id: every-crate-root-reexport-is-a-second-path-not-the-only-one
kind: issue
title: The crate root re-export is a SECOND path to an item, never the only one: pub mod camera plus pub use camera::cursor_projection gives two
status: open
opened: 2026-09-06
refs: [2089]
---



Found by the style review of #2089.

## The claim

PR #2089, its item's `## Closed` section and `work/view/log.md` all
say the same sentence about the `cursor_projection` move:

> that crate-root re-export moved from the `pub use marks::{…}` list
> to `pub use camera::{…}`, so there stays exactly **ONE** path to the
> function

and set it against `session-shims-and-test-imports`, whose deviation is
"two spellings of every moved path".

## It is two, before and after

`crates/viewer/src/lib.rs:55` is `pub mod camera;` and
`crates/viewer/src/lib.rs:65` is `pub mod marks;`. Both modules are
public, so `crates/viewer/src/camera.rs:882`'s `pub fn
cursor_projection` is reachable as **both**
`viewer::camera::cursor_projection` and — through
`crates/viewer/src/lib.rs:127`'s `pub use camera::{…, cursor_projection}`
— `viewer::cursor_projection`. Before the move the same two paths
existed through `marks`. What the move preserved is the COUNT, not a
uniqueness that was never there.

The module-path spelling is not hypothetical in this crate:
`crates/viewer/tests/datum_draw.rs:23` reaches `viewer::datums::{self,
DatumKind, …}` through exactly that door, and
`session-shims-and-test-imports` exists because 32 test files do it for
`session`.

## Why it is a class and not a slip

Every item in `lib.rs`'s `pub use` blocks has two public paths, because
every module it re-exports from is `pub mod`. The distinction
`session-shims-and-test-imports` actually draws is between two paths
where one is a `pub use` **shim inside a module** (a lie about where
the item lives) and two paths where the second is the crate root (a
convenience). That is a real distinction and worth stating; "exactly
one path" is not it, and a reader who believes it will conclude that a
crate-root re-export removes a spelling rather than adding one.

**Where else to look**: every claim in `work/view/` and in
`crates/viewer/README.md` that a re-export leaves one spelling, and the
`## What it takes` section of `session-shims-and-test-imports`, which
plans a sweep whose end state is described the same way.

## Confidence

`sure` that both paths resolve — it follows from `pub mod camera;`
alone. `likely` that the sentence is worth correcting in all three
places it appears rather than only the PR body, since two of the three
(the item and the log) are the durable ones.
