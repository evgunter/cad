---
id: the-walk-panics-on-a-slit-in-a-curved-face
kind: issue
title: The chart walk panics on a cylinder face carrying a slit, its parted end two declared vertices on one point
status: open
opened: 2026-10-30
priority: P1
cost: M
---


Found by SPLITDOOR's keel-on-a-ruling triage (branch
`splitdoor/keel-ruling-wedge`), measured at ε 1e-9.

## What

The block `[0, 4] × [−2, 2] × [0, 4]` with a half-bore of radius 1
about the z axis, less the V-trench over `(1, 0)`, `(1.6, ±0.3)` from
`z = 2` up out of the top
(`crates/sweep/tests/a_keel_on_a_bores_ruling.rs`): tier 3 has the
bore's cylinder face carry the keel as a slit, two coincident line edges
from the top to the tip at `(1, 0, 2)`, the top end parted into one
vertex per wedge. That form, built as the union of the block's two sides
of the trench (`slit_block` there), passes tiers 3 and 3′. `mesh::tessellate(&body, 5e-3, tol)` panics in
`crates/mesh/src/walk.rs` (the `coincident_declared` `debug_assert`
before the pole classification): "face … loop …: two DECLARED vertices
lie within eps 1e-9 of each other … Some((18, 12, 0.0)) … this is a
kernel bug". The two vertices are the slit's parted end, one per wedge,
on one point: legal at rest (`crates/topo/README.md` tier 3, "a slit of
that loop"), so the guard's premise does not hold for a slit. The
release profile keeps debug assertions, so it is live.

It panics on main too (PR 6052's
`crates/sweep/tests/a_slit_tip_on_a_curved_face_reads_its_full_turn.rs`
builds the same shape), so the gap is the walk's. `block ∖ trench` shipped
the one-edge form (the bore's face through the keel's wedge), which meshed;
PR 6061 has it refuse `EdgeInFaceGap::CurvedFace` instead
(`crates/topo/src/boolean/slit.rs`), and lifting that refusal is this
row's consumer, with
`work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`.

## Where to look

- `crates/mesh/src/walk.rs`, `coincident_declared`: a loop that passes
  one point at two declared vertices, a slit's parted end, is a pinch of
  the chart loop, which the planar lane already meshes
  (`tessellator-panics-on-a-self-slit-face-every-validator-passes`: "the
  doubled-slit form … meshes correctly").
- Either the walk meshes the pinch, or it refuses typed
  (`crates/topo/README.md`: consumers with no answer "refuse typed at
  the consumer"), never panics.

## Pinned by (CLEAVE, PR 6177)

`crates/sweep/tests/a_cylinder_through_a_hole_tangent_to_its_wall.rs`
`holds`: a cylinder through a plate, its wall tangent to a hole's along a
ruling, with only one rim's vertex on that ruling. That operand's wrap
edge lies along the other's wall, and the op keeping both walls refuses
`EdgeInFace { gap: CurvedFace }`; the pin goes red when the gap retires.
