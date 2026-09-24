---
id: viewer-src-test-modules-restate-the-literal-doors
kind: issue
title: crates/viewer/src unit-test modules restate len and scl at fourteen sites, with no shared home they can reach
status: open
opened: 2026-09-24
priority: P4
cost: D
---


## Finding

- **Where**: `#[cfg(test)]` modules in `crates/viewer/src/`, fourteen
  `.expect`-shaped `Expr::literal(_, Dimension::{Length, Scalar,
  Angle})` sites in four files, measured on the tree that merged
  `origin/main` at `21352850f`:

  | file | sites | shape |
  | --- | --- | --- |
  | `session.rs` | 6 | a `len`/`scl` closure pair, a second `len` closure, three inline |
  | `drafts.rs` | 4 | two closure pairs, one named `length`/`scalar` and one `len`/`scl` |
  | `pane/profile.rs` | 2 | a `len`/`scl` closure pair |
  | `widgets.rs` | 2 | `fn len` / `fn scl` in `value_field_tests`, added on main after `cd9fdfd6b` |

  Every one is a private re-spelling of `tests/common`'s `len` / `scl`
  / `ang`, and every one sits after its file's first `#[cfg(test)]`.
  The `?`-shaped and `map_err`-shaped calls in the same tree
  (`drafts.rs`, `session/author.rs`, `props.rs`, `scene.rs`) are
  production code doing its job, not members.
- **Why it is a row and not a fold**: `tests/common` is a module of the
  `all` test binary, and a `src/` unit-test module cannot import it.
  The home would be a `#[cfg(test)] pub(crate)` support module inside
  `viewer`'s own `src/` — a new module on `view`'s ground, and the
  question of whether it and `tests/common` should share one
  definition (they cannot today) comes with it.
- **How it was missed**: the literal-door row's census carried a path
  argument while its prose said it did not, and the cross-crate row
  that ran next excluded `crates/viewer/` whole. This tree fell
  between them. The same shape of gap is why
  `crates/viewer/src/marks.rs` and `pane/viewport.rs` each hold a
  test-module copy of the pick-index build that no `tests/` census
  reaches.
- **Importance**: low. No oracle rides on a literal door.
- **Instrument, and its blind spot**: `git grep -l -E
  'Expr::literal\([^;]*Dimension::(Length|Scalar|Angle)\)\.expect' --
  crates/viewer/src/`, then each hit's line compared against its
  file's first `#[cfg(test)]`. It is `.expect`-shaped and line-shaped,
  so a call whose arguments wrap is missed, and "after the first
  `#[cfg(test)]`" is a proxy for "inside a test module" that a file
  with production code below its test module would defeat (none of the
  four is laid out that way, checked by reading).
- **Raised by**: the S-DUP lane closing the four viewer-suite door
  rows, in its merge pass, 2026-09-24.

## Why this sits on S-DUP's slate

The ground is `crates/viewer/src/`, which `view` owns, and a fold there
needs a module `view` would have to accept; any claimant may take it by
`git mv`. It sits here because the subject — one construction spelled
fourteen times — is S-DUP's charter.
