---
id: a-pinch-union-body-trips-the-watertight-census-in-one-member-order
kind: issue
title: The pinch union's body trips the watertight chord census in one member order
status: open
opened: 2026-09-30
---


## What

The pinch union of
`work/tang/a-pinch-union-refuses-ring-homing-in-one-member-order.md`
(plate `[0,3] × [0,2] × [0,1]`, `p1` = x∈(1.0,1.5), y∈(−1,1),
z∈(0.5,2.0), `p2` = x∈(1.5,2.0), y∈(1,3), z∈(0.47,1.7), footprints
touching at one corner) evaluates in the member order `[p1, p2, plate]`,
and tessellating its body at 1e-3 trips the debug assertion in
`mesh::tessellate` (`crates/mesh/src/tessellate.rs`, the census over
`unpaired_chord_segment`): "chord segment … is an edge of 1 face
triangles; a watertight emission uses every chord segment exactly 2
times". The other four publishing orders tessellate.

Either the body of that order differs from the others at the pinch
vertex, or the tessellator emits the pinch's chords unevenly; the
census cannot say which. Found by the obstacle-mechanism measurement
(branch `emit/borders-mechanism-probe`,
`crates/editor-core/tests/borders_probe.rs`, fixture `pinch`), whose
planar truth tessellates every result.

## Cause (TANG, 2026-10-02)

The body differs. The tessellator is not at fault. In `[p1, p2, plate]` the
two blocks are joined first, and their coincident corner edges pierce the
plate's top at (1.5, 1, 1) once each. Each pierce minted its own vertex, so
the top stayed one face whose outer loop passes through the point twice,
by two distinct vertices: F = 18, V = 33, against F = 19, V = 32 in the
other orders. The census tripped on that face. Branch
`tang/pinch-union-order` welds the two pierces into one vertex and two
faces (`crates/topo/src/boolean/finish.rs`, `weld_pinches`). The order now
tessellates, and `crates/editor-core/tests/union_pinch_member_order.rs`
tessellates every order's result. Nothing is left for the tessellator from
this row, which closes with that branch. The census change the branch
made, two uses per chord carrying a segment, admits a body `check_mesh`
refuses: `two-coincident-edges-between-one-vertex-pair-mesh-non-manifold`.
