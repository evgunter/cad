---
id: a-seated-entity-collision-refuses-as-a-name-minted-twice
kind: issue
title: A seating door refuses an entity already named as SeatingRefusal::Duplicate, which says the name was minted twice
status: closed
opened: 2026-10-19
priority: P4
cost: E
closed: 2026-10-19
pr: 5407
branch: wire/p4-refusal-bundle
---


Found by `wire-census`'s sweep for the class of
`a-short-tie-refuses-as-a-duplicate-name` (a seating refusal that
reports another fact than the one it caught), and left for its own
unit by the STYLE review of PR 5324.

Two seating doors in `crates/editor-core/src/names/table.rs` refuse an
ENTITY that already has a name as `SeatingRefusal::Duplicate`:

- `NameTable::insert_ref`, its held-entity branch (the occupied
  `self.reverse.entry(ent)` slot);
- `NameTable::write_candidates`, its `self.reverse.contains_key(e)`
  check.

`NamingError::Duplicate`, which both fold into, then says the NAME
"was minted twice". Nothing was minted twice: one entity was offered a
second name, which is a different broken invariant (the reverse map's
injectivity, not the forward map's).

The same fact also refuses two ways depending on the door.
`insert_tied(name, [f1, f1])` deduplicates and refuses as
`SeatingRefusal::ShortTie { candidates: 1 }`, while
`insert_candidates_ref(name, [(Of(0), f1), (Of(1), f1)])` refuses the
repeated entity as `Duplicate`.

## Repair shape

Give an entity collision its own arm (for example
`SeatingRefusal::EntityNamed { name, held }`, carrying the name
already on the entity) and its `NamingError` arm, with the
kernel-defect ending, wired as PR 5316 wired `KindMismatch`. Decide
whether a repeated entity inside one offered list is that collision or
a short tie, and make the two doors agree. Also read
`TieRows::carry_as` and `TieRows::flush` in `names/defer.rs`: both
build `DuplicateName` directly, the first for a pair carried twice and
the second for a name with both carried and minted rows. Check each
against the fact it reports.
