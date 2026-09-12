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
