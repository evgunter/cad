---
id: which-operand-names-a-vertex-vertex-pinch-vertex-follows-the-cone-split
kind: issue
title: N2 does not say which operand names a result vertex at a vertex-vertex pinch: the name follows the operand key the cone split leaves kept, so it moves with the insertion's pairing
status: open
opened: 2026-11-12
priority: P2
cost: M
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
---


Found by GAUGE's shared-vertex pairing build (PR 6832) and its full
review (2026-11-12).

## What

Names README N2 (`crates/editor-core/src/names/README.md`, near lines
399-401) says a vertex of either operand is named as that vertex. It
does not say which operand names a result vertex where a vertex of
each meets at a pinch and the result holds several vertices there, one
per cone. `emit_topo` reads the kept key first: the operand key the
boolean leaves at the point, then its fusion partners
(`crates/editor-core/src/names/emit_topo.rs`, near line 2918, the
`operand_vertex_keys` read). So the name follows which vertex the cone
split (`topo`'s `zip::split_cones`) moves, and that follows the
insertion's pairing.

Measured while building PR 6832: face, edge and vertex counts are
equal throughout.
- `touch_reread_rows::a_vertex_read_twice_names_its_result_in_every_op`
  counted the operand corners held as several result vertices at 7 on
  main, at 8 under a pairing that followed the records' order, and at
  7 under the kept-side pairing. The kept-side 7 is a different set of
  scenes from main's:
  - `x ∩ y` crossing the island in the void, and hanging across the
    near-flat quad void, lose `x`'s fragment;
  - `y − x` over the quad void in the quad arch, and in the bare quad
    arch, gain one.
- In `x ∩ y` crossing the island in the void, the result's two
  vertices at the apex took one name from each operand under one
  pairing, and both from `x` on main.

PR 6832 pins the count with the vertex pairs reversed too
(`a_vertex_read_twice_names_alike_with_its_vertex_pairs_reversed`), so
a name that reads the records' order goes red. Which operand should
name such a vertex is unsettled. PR 6832's reviewer was unsure which
naming is right.

## Owed

A ruling: should a pinch vertex take a name independent of which
operand key the zip keeps, for example both operands' corners, or a
fixed operand? Then that rule in N2 and `emit_topo`, and the rows
re-pinned to it.
