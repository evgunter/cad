---
id: vectorslot-slots-has-no-reader
kind: issue
title: VectorSlot::slots() is public and unread: deleting it leaves the workspace green
status: open
opened: 2026-09-12
---



Filed by DOOR's `vectorslot-all-has-no-reader` unit, whose compile
measurement of the neighbouring member turned this one up. Filed here
because `crates/editor-core/src/node.rs` is DOCM's ground
(`work.py territory`), not DOOR's.

## The finding

`VectorSlot::slots` (`crates/editor-core/src/node.rs:433`, "All three
of this family's slots, component order (x, y, z)") is `pub`, is
re-exported through `pncad::document` with its type
(`crates/pncad/src/document.rs:78`), and **nothing reads it**.

Measured by deletion rather than by grep: with the function removed,
`cargo check --workspace --all-targets` finishes clean in 8.4 s. Every
`.slots()` call site in the tree — 36 of them — has a `Node`,
`ProfileProgram` or `TubeWindow` receiver, never a `VectorSlot`. The
other three members of the same `impl` all have readers and are not in
this class: `slot` is read by `pncad-py`'s `direction_expr`
(`crates/pncad-py/src/py/doc.rs`) and by `slots`/`dimension` here;
`label` and `dimension` are read by `SlotId::label` in this file and by
the properties panel (`crates/viewer/src/pane/properties.rs`).

## Why it is not just "a convenience nobody happens to call"

The workspace is `publish = false` throughout (root `Cargo.toml`:
*"Nothing is publishable until the project has its name (Q9)"*), so
there is no downstream this door could be for; and the type does not
cross the Python boundary at all — `crates/pncad-py/tests/test_binding_census.py`
lists `VectorSlot` in the not-bound set as `different-shape`, with the
reason that a Python caller addresses a slot through a door that names
it.

## The decision this asks for

Delete it, or name the consumer. It is one line and its body
(`Axis3::ALL.map(|axis| self.slot(axis))`) is a two-token composition
of a door that IS read, so nothing is lost by deleting it and nothing
is gained by keeping it unread. DOOR did not take it: one PR, one row,
and this is a second row's file.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/door/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the fix is written in the row and it is one PR on a file another program owns, which is DOOR's test. Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## The decision it asks for was already taken next door (DOOR orchestrator, 2026-09-20)

This row reads *"Delete it, or name the consumer"*, which looks like a
decision and is not one any more: **its own sibling closed by deletion**.
`vectorslot-all-has-no-reader` (PR 2446, closed 2026-09-12) deleted the
neighbouring unread member from this same `impl` in this same file,
after re-measuring the premise two ways. This row was filed BY that
unit's compile measurement and differs from it only in which member it
names.

So the row stays on DOOR's slate through the 2026-09-20 design-free
sweep: the fix is written (delete `VectorSlot::slots`, and the
re-export of it through `crates/pncad/src/document.rs`), and the
precedent for the call is one PR old on the same lines.

**What the lane still does rather than inherits.** Re-take the deletion
measurement at your own merge base — the row's evidence is
`cargo check --workspace --all-targets` clean in 8.4 s with the function
removed, and 36 `.slots()` call sites all having a `Node`,
`ProfileProgram` or `TubeWindow` receiver — and check the Python surface
again rather than trusting the note that `VectorSlot` sits in the
binding census's not-bound set.

**Fence:** `crates/editor-core/src/node.rs` is EDIT's (territory; the
row's own text says DOCM's, which closed on 2026-09-13). Announce there.
