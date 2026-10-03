---
id: five-doc-edit-arms-have-no-python-door
kind: issue
title: five DocEdit arms have no Python constructor
status: closed
opened: 2026-09-09
closed: 2026-09-09
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

## Closed

Closed at LIB-EDITS with TWO of the five arms bound and the other
three re-filed, each under the reason it is not a missing constructor.

`DocEdit.set_param` and `DocEdit.rebind` exist, take the address a
refusal answers with (a slot's WORD, a name's TEXT), and are exercised
one row per tag in `crates/pncad-py/tests/test_slot_edits.py`. Both
rows left `MEMBERS_NOT_BOUND` entirely rather than moving to
`MEMBERS_BOUND_AS`, because Python spells both namesake for namesake
and rule 1 accounts a member the stub spells.

The item's premise held for three of the five and not for two:

- `ReWitness` / `ReWitnessBulk` — the sentence the item wrote for the
  appearance four is theirs too. `WitnessDatum` and
  `BranchCertification` are not on `crates/pncad/src/document.rs`'s
  curated list, and `pncad-py` depends on `pncad` alone, so there is
  no type for a constructor to take.
  `work/lib/the-witness-edits-need-a-facade-type.md`.
- `SetExpression` — the constructor is mechanical and the payload IS
  curated; what blocks it is its own refusal.
  `EditError::PathOffTree`'s `Display` renders the address through
  `Debug`, and the binding's prose gate is a `debug_assert` live under
  release, so the door would panic where it must refuse. Observed by
  writing the door and provoking it.
  `work/lib/the-expression-path-edit-cannot-refuse-as-prose.md`.

`B-DOC-EDITS` therefore stays in the census's `FAMILIES` with three
rows and a charter that says what each needs before a constructor is
the work.
