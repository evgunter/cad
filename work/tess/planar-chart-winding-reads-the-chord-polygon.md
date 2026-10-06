---
id: planar-chart-winding-reads-the-chord-polygon
kind: issue
title: a planar face's mesh winding is read off its chord polygon, which a convex arc can make wind against a region thinner than the chord deviation
status: open
opened: 2026-10-06
priority: P3
---



Filed by CARVE (`sweep-cap-plane-winds-against-a-convex-arc-region`'s
sweep), outside its fence. Traced, not measured.

## The shape

`crates/mesh/src/planar.rs`'s `chart_frame` takes a planar face's
chart normal as Newell over the outer walk's chord points, and
`tessellate_planar` emits its triangles wound about that normal (the
module header's *"What 'the outward normal by construction' rests
on"*). The header grounds it in interior-left: the outer loop runs CCW
about the outward normal, so its Newell normal is outward. That holds
for the LOOP; the walk is its inscribed chord polygon. A convex arc
makes the chord polygon smaller than the region by about
`(2/3)·δ_s·(arc length)` (δ_s the sagitta budget `chords.rs` sizes
arcs to), and once that exceeds the region's own area the chord
polygon winds against it — the same defect CARVE fixed in `sweep`'s
cap planes, where the polygon was the vertices plus one apex per arc.

Precondition for a flip: a region whose mean width `2A/P` is below
roughly the sagitta budget, bounded mostly by convex arcs — a thin
C-ring (the `c_shape` in `crates/sweep/tests/m5_s10_face_sense.rs`,
radial width 0.1) meshed at a `δ` near or above its width.

## What it would do

The whole patch flips at once, so `check_mesh` reports
`MismatchedWinding` against the neighbouring faces: loud, not a
silent inside-out mesh. The cost is a refusal on a valid body at a
coarse `δ`, and a header sentence that claims more than it rests on.

## The fix's shape (TESS's call)

The planar face's winding is decidable without the chord polygon:
the face's stored plane with its `sense` folded in IS the outward
normal on a body tier 3 certified (check 6 now reaches arc-bearing
loops), or the region's arc-exact signed area as `profile`'s
`loop_orientation` computes it. Either replaces the inscribed
polygon's sign; the header's S10 CATEGORY B note says why folding
`sense` was avoided and would need revisiting.

The curved and trimmed lanes read their flip the same way
(`crates/mesh/src/curved.rs`'s `area2`, `crates/mesh/src/trimmed.rs`'s
`shoelace2(&poly2)`), over UV chord walks; worth the same look.
