---
id: five-doc-edit-arms-have-no-python-door
kind: issue
title: five DocEdit arms have no Python constructor
status: open
opened: 2026-09-09
---



Found by LIB-MEMBERS's first run of the member rule
(`work/lib/datum-crosses-name-for-name-as-two-types.md`, ruling (D)),
and chartered in the census as `B-DOC-EDITS`.

`editor_core::DocEdit` (`crates/editor-core/src/edit.rs:30`) has
nineteen arms. `crates/pncad-py/src/py/doc.rs` builds ten of them.
Four more are the appearance and metadata arms, which are not a
Python gap: `crates/pncad/src/document.rs:34-46` leaves `Attr`,
`AttrSet`, the record types and `MetaValue` off the curated list, so
those arms have no payload a consumer of that module can name in
either language.

**These five are a Python gap and nothing else:**

- `SetParam { node, slot, expr }` — a node's continuous slot
  expression AFTER insertion. Python can set a slot only at the
  constructor that mints the node.
- `SetExpression { path, expr }` — the same edit addressed by an
  `ExprPath`, which `EditError.path` already reads BACK
  (LIB-DOORS-1) with no door to write at.
- `Rebind { … }` — rebinding a stored name.
- `ReWitness { … }` and `ReWitnessBulk { … }` — a sketch node's
  witnesses.

`DocEdit::SetStructuralParam` is bound and is not in this list: the
three `bind_*_param` doors build it, one per named slot.

Invisible until now for the reason LIB-DOORS-3 recorded of its own
three doors: `DocEdit` is curated and `pncad.pyi` declares a top-level
`DocEdit`, so rule 1 accounted the enum WHOLE and which of its arms
Python could build was nobody's roster.

## What closing it looks like

One `DocEdit` constructor per arm in `crates/pncad-py/src/py/doc.rs`
with its `pncad.pyi` stanza, one test row per tag each can raise, and
the five `MEMBERS_NOT_BOUND` rows moving to `MEMBERS_BOUND_AS` —
which empties `B-DOC-EDITS` out of `FAMILIES`. Sizing is a brief's
job: the five are one family because they are one enum's unbuilt
arms, not because one unit must take all five.
