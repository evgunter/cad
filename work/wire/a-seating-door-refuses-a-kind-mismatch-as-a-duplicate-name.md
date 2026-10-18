---
id: a-seating-door-refuses-a-kind-mismatch-as-a-duplicate-name
kind: issue
title: The name table's seating doors refuse a key/name kind mismatch as DuplicateName
status: dispatched
branch: wire/small-breaks
opened: 2026-10-18
priority: P4
cost: E
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
