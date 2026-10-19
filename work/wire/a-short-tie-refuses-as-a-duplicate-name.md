---
id: a-short-tie-refuses-as-a-duplicate-name
kind: issue
title: The name table's tie doors refuse a tie of fewer than two candidates, or an empty candidate list, as SeatingRefusal::Duplicate
status: closed
opened: 2026-10-18
priority: P4
cost: E
closed: 2026-10-19
---


Found by `wire-small`'s sweep for the class of
`a-seating-door-refuses-a-kind-mismatch-as-a-duplicate-name` (a seating
refusal that reports another fact than the one it caught).

`NameTable::insert_tied_ref` (`crates/editor-core/src/names/table.rs`)
refuses a tie that deduplicates to fewer than two candidates as
`SeatingRefusal::Duplicate`, and `NameTable::insert_candidates_ref`
refuses an empty pair list the same way. Neither is a duplicate: nothing
is seated twice. `NamingError::Duplicate`, which both fold into, then
tells the reader the name was "minted twice". `SeatingRefusal::Duplicate`'s
doc was widened to cover them so the doc is not false, but the arm still
reports the wrong fact.

Fix: give a short tie its own arm (for example `SeatingRefusal::ShortTie
{ name, candidates }`) and its `NamingError` arm, with the kernel-defect
ending. `sure` as a reading of the source, and `unlikely` to be reached
while the emitters keep handing in lists they built from two or more
entities.

## Resolution (2026-10-19)

A tie that deduplicates to fewer than two candidates
(`NameTable::insert_tied_ref`), and an empty carried list
(`NameTable::insert_candidates_ref`), refuse as
`SeatingRefusal::ShortTie(ShortTie { name, candidates })`, which folds
into `NamingError::ShortTie` with the kernel-defect ending. It is wired
into the category map, `display_tests`, the `NAMING_ERROR` census, the
feature-tree census row, the Python tag `short_tie` and the binding
census. Test: `names::table::carrying_door::a_short_tie_refuses_as_a_short_tie`.
The sibling the sweep found, an entity already named refusing as a
name minted twice, is
`work/wire/a-seated-entity-collision-refuses-as-a-name-minted-twice.md`.
