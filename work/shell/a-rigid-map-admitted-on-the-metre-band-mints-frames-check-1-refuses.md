---
id: a-rigid-map-admitted-on-the-metre-band-mints-frames-check-1-refuses
kind: issue
title: transform_rigid admits a map whose columns are off unit by up to ~eps, dimensionless and unlevered; the mapped cylinders of radius > ~1 m then refuse at check 1, at a door that does not name the map
status: open
opened: 2026-09-25
priority: P3
cost: D
refs: [ATREST-13]
---


## What

`topo`'s rigidity door (`crates/topo/src/transform.rs`, `check_rigid`,
reached through `transform_rigid`) admits a map when each of its seven
defects — `cᵢ·cᵢ − 1`, `cᵢ·cⱼ`, `det − 1` — decides `Zero` against the
LINEAR (metre) band. The defects are dimensionless and unlevered; the
site says so and flags it as ledger row **F10** (`decide_flagged(…,
"F10")`: "the natural arm is the model/session-box extent — an
arm-policy question deferred by the row";
`work/meta/decide-flagged-dimensional-debt-inventory.md`). So at
ε = 1e-9 a map whose columns are 4e-10 off unit is admitted as rigid.

ATREST-13 (PR 3238) made tier 3's check 1 read every analytic frame's
unit-ness and orthogonality to within ε of LOCUS MOVEMENT at the kind's
radius (`geom::Surface::representability_margins`,
`geom::Curve3::representability_margins`, the frame margins in
`crates/geom/src/convention.rs`). A frame mapped through an admitted
map carries the map's column defect `δ` into its `axis`/`u_ref`; every
mapped cylinder, sphere, torus or circle with `δ·r > ε` — at
δ = 4e-10, ε = 1e-9, any `r > 2.5 m` — then refuses at check 1
(`UnrepresentableSurfaceDatum { measure: Length, … }`), at a later door
that names the face, not the map. Found by reading, not measured: no
corpus body was seen doing it (ATREST-13's corpus instrument found no
frame beyond 0.15 ε of movement).

## The two ways it could close (not chosen here)

- **Lever the acceptance**: meter the map's defects by the extent they
  will be applied over (F10's own suggestion — the model or session
  box), so an admitted map moves no mapped point by more than ε.
- **Re-orthonormalize at mint**: keep the acceptance, and have the
  transform re-derive each mapped frame orthonormal (normalize `axis`,
  project and normalize `u_ref`) so the stored frame is unit and
  orthogonal to rounding whatever the map's slack.

## Fence

`crates/topo/src/transform.rs` (shell's ground per `work.py
territory`, shared with offset). Cross-reference F10.
