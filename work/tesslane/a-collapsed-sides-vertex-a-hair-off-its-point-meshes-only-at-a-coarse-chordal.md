---
id: a-collapsed-sides-vertex-a-hair-off-its-point-meshes-only-at-a-coarse-chordal
kind: issue
title: a collapsed side's vertex within the band of the side's point but not bitwise it meshes only at a coarse chordal: the lift turns the face there and the polygon holds the vertex once
status: open
opened: 2026-11-23
---


Filed by TESSLANE's `tesslane/collapsed-side` lane. The trimmed lane reads a
spline face's vertex as the point of a collapsed side only when the vertex's
stored point IS the side's stored point (`crates/mesh/src/trimmed.rs`,
`CollapsedSides::holds`, `one_stored_point`), with no band, as the side itself
is read at its home (`geom_brep::chart_singularity::collapsed_sides`). The
loop's lift decides the same incidence at the run's band
(`topo` `pcurves/decision.rs`, `singular_at`, through
`vertex_at_singular_point`). The two readings disagree on a vertex within the
band of the side's point and not bitwise it.

**Measured** (`trimmed::collapsed_rows::a_vertex_in_a_sides_band_but_not_its_point_holds_one_copy_and_meshes_coarse`):
the wedge of `crates/sweep/tests/pinned_corners_build.rs` with every vertex
point restated an ulp up (`topo::test_support::restate_geometry`). The lift
reads and turns the face at the vertex. The polygon holds the vertex once, and
its side runs from the top rim's last chord point diagonally to the bottom
rim's first, across the collapsed column, as every collapsed side did before
the side was walked. That shape is sound: the corner the polygon holds lies
on the side. The face meshes at δ = 0.1 and refuses
`CertificateExceeded` at 1e-2. Nothing folds or panics.

**Which door could make one.** The loft cannot: it writes a corner it decides
pinned as its first section's point (`sweep` `skin.rs`, `pin_strips`), so the
vertex and the net's row are one stored point. Step import could: a STEP
B-spline face with a degenerate side whose vertex is written apart from the
net's degenerate row by rounding. Import heals the SIDE to one point at ε_in
(`NormalizationKind::CollapsedSideMadeOne`), but whether it writes the vertex
as that point is unmeasured. Measuring that, and making the import write the
vertex as the side's point, is the likely cure; a mesh read at the band would
put ε into the choice of the mesh (`[tolerance.chordal]`).
