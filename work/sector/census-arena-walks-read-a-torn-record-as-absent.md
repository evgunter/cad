---
id: census-arena-walks-read-a-torn-record-as-absent
kind: issue
title: The census's own arena walks read a torn record as absent
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

(Found by the TOPO lane that converted
`torn-hops-read-as-absent-in-the-split-the-chord-join-and-the-reach-rules`,
whose remit in `census.rs` was the reach rules only: `face_reach_in`,
`boundary_axial`, `boundary_reach`, `edge_reach_in`. Those now panic
on a hop past the face or edge they resolve.)

The census runs on bodies tier 1 admits, so a record miss inside it is
a kernel bug, and D2 row 4 (Ev on PR 4006) says it panics naming its
premise. The census's own walks read one as absent instead. Candidates
from `get_*(` / `.<arena>.get(` followed by `?`, `continue`,
`filter_map`, `if let Some` or `is_none()`, in production code
(line numbers ride along and may be stale):

- `edge_is_line` (`census.rs:1058`): a torn curve reads as "not a line",
  so the edge leaves the exact sweeps.
- `face_cycles` (`:1075`): a lost loop or an unwalkable cycle answers
  `Err(loop)`, and each caller says what that means.
- `snapshot` (`:1094`, `:1107`, `:1135`, `:1154`): a vertex whose point
  is torn is dropped by `filter_map`, an edge whose half-edge, vertex
  or point is torn is dropped by `?`, a face whose surface is torn is
  read as not planar.
- `sweep_cross_solid_backstop` (`:4731`, `:4756`, `:4775`, `:4783`,
  `:4847`, `:4948`, `:4982`, `:5059`, `:5145`): faces, half-edges and
  surfaces read through `if let Some` / `let Some(..) else`, a miss
  skipped or read as no box.
- `confirm_curve_and_patch_records` (`:5634`): a contact's edge curve
  read through `.and_then(..)`.

**HOLD (D10, PR 3990).** `ee_cross_backed` (`:2057`),
`sweep_conformal_patches` (`:2369`) and the patch arm of
`confirm_curve_and_patch_records` (`:5690`) read declared face pairs
and contact records: their own logic reads a declaration, so they wait
for D10. Where a declared key is stale, `StaleContactDeclaration` is
the honest typed answer and stays.

## Direction

As the boolean's record-hop unit did it (PR 4066): per site, decide
whether the key is the caller's (typed) or a record hop past a
resolved record (`live::linked` / `proven` / `face_surface_linked` /
`edge_curve_linked` / `Body::face_loops_linked`, a panic naming the
premise), with a torn-body witness per converted read that goes red
under a mutation restoring the old reading.
