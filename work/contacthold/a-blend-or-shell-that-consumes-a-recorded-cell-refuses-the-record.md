---
id: a-blend-or-shell-that-consumes-a-recorded-cell-refuses-the-record
kind: issue
title: A surgery verb that removes a recorded touch's cell or material refuses the record; whether the record is consumed or inherited is undecided
status: closed
opened: 2026-10-11
priority: P2
cost: M
refs: [topo-surgery-verbs-drop-a-bodys-contact-records, surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key]
closed: 2026-10-12
pr: 4809
branch: contacthold/one-record-carry
---

## What

Each surgery verb's carry maps its operand's records through the verb's
lineage (`ContactRecords::carried_through`,
`crates/topo/src/boolean/records.rs`). Two verbs can remove what a
record stands on, and each one refuses today. None of them drops the
record:

- **A cell the verb consumes.** `fillet_edges` and `chamfer_edges`
  (`Blended::carry_contacts`, `crates/sweep/src/blend/build.rs`) replace
  the requested edge with a blend face (`BlendNaming::blends`), a corner
  vertex with a patch (`BlendNaming::corners`), and retract vertices to
  feet (`BlendNaming::feet`). `shell_open` retires a designated chart's
  merged-away faces (`ShellRetired`). A record on any of these is refused
  `CarryRefusal::Unmapped`. No reachable blend instance was found: on
  the corner kiss of `crates/sweep/tests/blend_carries_records.rs`, a
  fillet of an edge ending at the kiss vertex is refused first by the
  battery (`FaceClearance` against the other cube's face).
- **Material the verb removes from under a touch on a cell it keeps.**
  `shell_open` designating the slab's top face, under a cube resting one
  corner on that face, keeps the face's key on the annular rim. The
  record carries by key, and the census refuses it
  `StaleContactDeclaration`, because the corner now points into the
  opening (`a_rim_that_opens_under_a_resting_corner_leaves_its_record_stale`
  in `crates/topo/tests/surgery_carries_records.rs`). The two solids
  touch nothing, so the result is a valid body refused.

In each case the record could be consumed, because the touch went with
the material. Or it could move onto the cell that replaced its own
(D1's "rewrites every record naming it onto the replacement, by
substitution": a blend face, a corner patch). A merge's killed cells are
not in question: they lie inside the face the merge keeps, and
`MergedGroup::substitutions` carries them there. Or the
verb could refuse, as it does now. The answer may differ between a
consumed cell and removed material. Weigh it before building it
(`memories/orchestration-model.md`). Whatever is decided has to keep
the census's rule that it never scans to bless: a record cannot be
dropped because the census found no witness for it.

## Resolved (PR 4809)

Ev ruled on `[ev]` PR 4749: consumed where a cell was removed or
divided with its remainder removed, stale otherwise; a blend face
inherits nothing. `Shelled::lineage` and `Blended::lineage` state the
removals; the rim under a resting corner and the record on a blended
edge are now consumed (`surgery_carries_records`,
`blend_carries_records`).
