---
id: pinches-on-sphere-cone-and-torus-faces-are-unmeasured
kind: issue
title: A pinched outer loop on a sphere, cone or torus face, and an unparted curved slit, are unmeasured in the mesh
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by TESSLIE's walk-slit unit (branch `tesslie/walk-slit`).

## What

A curved face whose outer loop passes one point record at two vertices
(`mesh::walk::pinches`) is routed to the trimmed lane, and the curved lane
refuses one typed (`curved::PINCHED_FACE_ROUTED_HERE`). Only the cylinder
is measured, by two rows in
`crates/sweep/tests/a_slit_tip_on_a_curved_face_reads_its_full_turn.rs`
(`the_slit_block_meshes`, `a_full_height_slit_in_a_full_bore_meshes`). The
predicate reads no chart, so sphere, cone and torus faces take the same
route, but whether the trimmed lane meshes a pinch there is unmeasured.

The cone attempt does not build: revolve `(0, 0)`, `(1.6, 8)`, `(0, 8)`
about y, subtract it from the box `x ∈ [−3, 3]`, `y ∈ [3, 6]`,
`z ∈ [−3, 3]`, then cut the keel prisms along a ruling. The Boolean door
refuses `CurvedPierceUnsupported`.

Also unmeasured: a slit in a curved face whose two edges end at ONE vertex
on the boundary (unparted). Its loop meets that vertex twice, which is no
pinch, so it reaches the curved lane. If its tip is off the bbox, the
lane's domain guard refuses it typed (`UnsupportedCurvedDomain`), but
whether it panics or meshes wrong first is unmeasured. No door is known to
build the form.
