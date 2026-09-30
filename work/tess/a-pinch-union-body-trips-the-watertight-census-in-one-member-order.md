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
