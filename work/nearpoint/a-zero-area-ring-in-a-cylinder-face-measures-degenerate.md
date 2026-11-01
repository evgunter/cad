---
id: a-zero-area-ring-in-a-cylinder-face-measures-degenerate
kind: issue
title: Mass properties refuse DegenerateFace on a cylinder face carrying a zero-area ring of two coincident rulings
status: open
opened: 2026-10-30
priority: P2
cost: M
---


Found by SPLITDOOR's keel-on-a-ruling lane (PR 6061), ε 1e-9.

## What

The pipe (radius 1 about z, `z ∈ [−2, 2]`) less the box
`[−x, x] × [−0.3, 0.3] × [−0.3, 0.3]`, `x = √0.91`, whose four z-edges are
rulings of the pipe's wall (`crates/sweep/tests/verbs_ga_r2_probes.rs`,
`r2_a_box_with_on_carrier_rulings_builds_every_op_but_its_void_which_refuses_typed`). Each ruling's
material is two wedges with the wall between them, so tier 3 has the wall
carry each ruling as a zero-area ring of two coincident edges
(`crates/topo/README.md` tier 3). With the Boolean's parting built on a
curved face (commit `cace0851` of `splitdoor/keel-ruling-wedge`, before the
lane switched to refusing), the result gate refused it:
`ResultInvalid { errors: [VolumeUncomputable { solid, source: Face { face:
FaceKey(3v1), source: DegenerateFace } }] }`, the face being the pipe's wall
with its four rings. `PropsError::DegenerateFace` is raised in
`crates/geom-brep/src/props/curved.rs` and `props/quad/cylinder.rs`.

A zero-area ring bounds no area, so the wall's measure is the wall's
without it.

## Consumer

The Boolean refuses that parting on any curved face
(`EdgeInFaceGap::CurvedFace`, `crates/topo/src/boolean/slit.rs`), for this
row and for the mesh's
(`work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`). Lifting the
refusal needs both.

## Pinned by (CLEAVE, PR 6177)

`crates/sweep/tests/a_cylinder_through_a_hole_tangent_to_its_wall.rs`
`holds`: a cylinder through a plate, its wall tangent to a hole's along a
ruling, with only one rim's vertex on that ruling. That operand's wrap
edge lies along the other's wall, and the op keeping both walls refuses
`EdgeInFace { gap: CurvedFace }`; the pin goes red when the gap retires.
