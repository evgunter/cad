---
id: a-boolean-corner-on-a-curved-wall-is-read-against-the-wall-not-its-section
kind: issue
title: A Boolean vertex read on another solid's curved face decides its residual, not its distance from the section a grazing planar face of its own cuts there
status: open
opened: 2026-10-12
priority: P3
cost: M
---


Found by the sweep of
`the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`
(branch `cleave/vertex-against-section`). Unmeasured past a frontier
refusal; this row records the hypothesis and the pose to measure.

## What

That unit reads a vertex ON a plane against the section the plane cuts
in each curved face at the vertex, `δ / sin ψ`
(`vertex_section::decide_vertex_on_plane`), at the split and at the
Boolean's vertex against a planar face of the other solid. The mirrored
reading is not covered: in `boolean::reduce::curved_face_arm`, a vertex
of one solid is decided ON a curved face of the other by its residual
(`bool_vertex_face_side` over `geom_brep::implicit_residual`). A planar
face of its own solid at that vertex, grazing the curved face at
dihedral `ψ`, cuts it in a section that passes `δ / sin ψ` from the
vertex, which the join then certifies.

The pose: a box `[1, 2] × [0, 1] × [0.5, 1.5]` whose corner sits on the
unit cylinder's wall at `(1, 0, 0.5)`, its face `x = 1` turned about
the corner onto `n = cos ψ·w + sin ψ·ẑ`, the box moved `δ` along `n`,
the cylinder less the box (`VS_DOOR=mirror` in
`crates/sweep/examples/vertex_against_section_probe.rs`). Today every
row with `|δ| < ε` refuses `CurvedPierceUnsupported` first, at
ε = 1e-9 and 1e-12, so the reading is not reached.

## Owed

Once the pierce lane serves a corner on a curved wall, measure the pose
over the ψ ladder (90°, 30°, atan 0.1, 3°) at every ε. Where the miss
shows at the join, decide the corner against the sections its own
planar faces cut there, through the same rule.
