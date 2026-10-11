---
id: a-blend-or-shell-that-consumes-a-recorded-cell-refuses-the-record
kind: issue
title: A blend or shell that consumes a recorded cell refuses the record: what its new faces inherit is undecided
status: open
opened: 2026-10-11
priority: P1
cost: M
design: true
refs: [topo-surgery-verbs-drop-a-bodys-contact-records]
---

## What

A surgery verb's carry refuses, `CarryRefusal::Unmapped`, a record
naming a cell the verb consumed and names no replacement for
(`ContactRecords::carried_through`, `crates/topo/src/boolean/records.rs`):

- `fillet_edges` and `chamfer_edges` (`Blended::carry_contacts`,
  `crates/sweep/src/blend/build.rs`): the requested edge (its blend
  face is `BlendNaming::blends`' row), the corner vertex a corner patch
  replaces (`BlendNaming::corners`), a vertex a foot retracts from
  (`BlendNaming::feet`);
- `shell_open` (`Shelled::carry_contacts`, `crates/topo/src/shell.rs`):
  a designated chart's merged-away face and the faces, seams and slits
  in `ShellRetired`;
- `merge_coplanar_faces` (`MergeCoplanarOutcome::carry_contacts`): a
  group's `killed_vertices`, which the field's own doc calls consumed.

That refusal is honest: nothing is dropped and nothing is invented. But
it stops a real composition, such as rounding an edge that ends at a
declared corner kiss. Whether such a record is consumed, moves onto the
blend face or corner patch that replaced its cell (D1's "rewrites every
record naming it onto the replacement, by substitution"), or splits
across several new cells is a choice with several viable answers. It
should be weighed before anyone builds it.

Measured on the corner kiss of `crates/sweep/tests/blend_carries_records.rs`.
