---
id: viewer-src-test-modules-restate-the-literal-doors
kind: issue
title: crates/viewer/src unit-test modules restate len and scl at fourteen sites, with no shared home they can reach
status: closed
opened: 2026-09-24
priority: P4
cost: D
closed: 2026-09-26
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
- **The same home question, one module wider** (added 2026-09-24 by
  the unit closing `the-rectangle-profile-is-still-written-longhand-beside-its-door`):
  `widgets.rs`'s `value_field_tests` also re-spells `tests/common`'s
  `edited` and `inserted` (its own doc says so), a longhand xy frame,
  and a 0.04 m square as a corner-by-corner `LoopProgram::polygon` —
  `common::{xy_frame, square}` in a module that cannot import them.
  A `src` test-support home would take those too.
- **The document insert door, spelled in `src` test modules** (added
  2026-09-24, same unit, measured at `5fe3dff36`). Scope:
  `crates/viewer/src/`, every `apply(` call, each classified test or
  production by its line against its file's first `#[cfg(test)]`, and
  the test-side ones read. Seven sites take `DocEdit::InsertNode`
  through `apply` directly, which is `tests/common`'s `inserted` /
  `insert_into`:
  - `drafts.rs` — a `let insert` closure in
    `a_committed_profile_is_not_drawn_again_as_its_preview` (`:1116`),
    `tests/common`'s `inserted` down to its expect text; and two inline
    inserts in `authored_by_the_form` (`:1294`, `:1310`);
  - `pane/profile.rs` — two inline inserts (`:489`, `:603`);
  - `session.rs` — two inline inserts (`:3055`, `:3077`);
  - `widgets.rs` — `value_field_tests`' own `edited` / `inserted`
    (above).
  Not members: `session.rs:3139` and `:3151` (a refusal premise and a
  replay fold over logged edits), `drafts.rs:1336` (a `SetParam`
  fold), and every production `apply` (`scene.rs`'s fallible `insert`,
  the session's own commit paths, `session/probe.rs`). Blind spot: the
  test/production split is a line proxy (no file here has production
  code below its first test module, checked by reading), and a
  method-form `doc.apply(..)` would be missed (none in `src`, by
  `git grep 'apply(&' -- crates/viewer/src`).
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

## Closed

Folded by the S-DUP lane `dup/b6-b`, 2026-09-26, cut from
`032999ff2`. PR: batch 6.

- **Census re-taken at the merge base**, with the row's instrument
  and without its `.expect` shape (`git grep -n 'Expr::literal' --
  crates/viewer/src/`, each hit's line against its file's first
  `#[cfg(test)]`): the fourteen, as the row counted them —
  `session.rs` 6, `drafts.rs` 4, `pane/profile.rs` 2, `widgets.rs` 2.
  The seven `apply(DocEdit::InsertNode)` sites the row added, and
  `widgets.rs`'s own `edited` / `inserted` / `node_insert`, longhand
  xy frame and 0.04 m square, all present. A second pass for the
  frame (`Datum::Frame {` / `DatumSpec::Frame {` in test code) found
  the xy frame written out in all four files — three of them through
  `session::author::datum_node`, which maps `DatumSpec::Frame` to
  `Datum::Frame` field for field, so the value is the same.
- **The home, and the sharing question the row raised.**
  `crates/viewer/src/test_support.rs`, declared `#[cfg(test)] mod
  test_support;` in `lib.rs` — and `tests/common/mod.rs` MOUNTS the
  same file by `#[path]` and re-exports its doors, so the unit-test
  modules and the integration suites read ONE definition. That is
  possible because the file names only `pncad` (a `crate::` or
  `viewer::` path would mean a different crate in each binary); it
  holds the literal family (`len`, `len_mm`, `scl`, `ang`, `len2/3`,
  `scl2/3`), `edited` / `inserted`, `frame` / `xy_frame` and
  `rectangle_loop` / `rectangle` / `square`, all moved out of
  `tests/common` rather than copied. No manifest, feature or public
  item changes. The module declares itself a **vocabulary**
  (`viewer-module-kinds.sh` passes; it names no driver).
- **Routed**: all fourteen literal sites, all seven inserts,
  `widgets.rs`'s five private doors, and the xy frame in all four
  files. `drafts.rs`'s `SetParam` fold and `session.rs`'s refusal
  premise and replay are not inserts and stay.
- **Plants**: a `panic!` in `inserted` (T1) reds every unit-test row
  that authors through it — `session` 3, `drafts` 4,
  `pane::profile` 1, `widgets::value_field_tests` 5 — and 308
  integration rows; doubling `len` (T2) reds 66 integration rows and
  no unit-test row, so the unit rows are reached (T1) and do not
  measure a length.
- **Not folded, filed**: `marks.rs`'s `delta()` and
  `pane/viewport.rs`'s `plate_index`, the "same shape of gap" this row
  named — the shared file cannot name `DocSession` or
  `viewer::scene`. `viewer-src-test-modules-hold-the-pick-index-build-no-shared-home-can-take`.
