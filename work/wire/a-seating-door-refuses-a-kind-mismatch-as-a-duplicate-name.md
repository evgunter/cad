---
id: a-seating-door-refuses-a-kind-mismatch-as-a-duplicate-name
kind: issue
title: The name table's seating doors refuse a key/name kind mismatch as DuplicateName
status: closed
opened: 2026-10-18
priority: P4
cost: E
closed: 2026-10-18
---

`NameTable::insert_ref` (`crates/editor-core/src/names/table.rs:473`)
and `insert_tied_ref` (`:590`) refuse a row whose key's entity kind
is not its name's `kind`, and they report it as `DuplicateName`, which
`table.rs:278` documents as "a duplicate-name insertion outside the tie
path". A kind mismatch is a different emission bug from a duplicate.
Whoever reads the error, or the `NamingError` it is folded into, is
told the wrong fact.

Since PR 5264 (`wire/selection-kind-at-the-door`), the selection doors
depend on this check. A selection's per-name kind is held at the edit
and load doors (`Select::fault`) on the premise that every entity a name
denotes is of that name's kind, and these two doors are what make that
premise true. The refusal that enforces it should therefore say what it
enforces.

Fix: give the mismatch its own typed refusal (a `KindMismatch { name,
key }` beside `DuplicateName`, or a field on a shared seating error).
Then name it in `Select::fault`'s doc, which currently describes the
check in prose.

## Resolution (2026-10-18)

The seating doors now return `SeatingRefusal`, which has two arms:
`Duplicate(DuplicateName)` and `KindMismatch(KindMismatch { name, found })`.
`found` is the offered entity's kind, not its key, because an
`EntityKey` never leaves the evaluation that minted it. It folds into a
new `NamingError::KindMismatch`, an emission bug that ends with the
kernel-defect ending. That arm is entered in the category map, the
display rows, the `NAMING_ERROR` census, the refusal-concision rows and
the Python tag table (`kind_mismatch`). `Select::fault`'s doc now names
it. Every door checks kind before any collision or list-shape check, so a
row with both defects is always `KindMismatch`. Pinned by
`names::table::carrying_door::a_kind_mismatched_seating_refuses_as_a_kind_mismatch`,
which checks every door: `insert`, `insert_tied` (each candidate) and
the carrying door's strict and numbered rows. It also checks the
two-defect rows: a held name, an already-seated candidate, a malformed
strict list and a one-candidate tie, each beside an edge. The sibling (a short tie
refused as a duplicate) is `a-short-tie-refuses-as-a-duplicate-name`.
