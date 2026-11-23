---
id: the-trimmed-sphere-arm-has-no-worst-triangle-theorem
kind: issue
title: the trimmed lane's sphere arm has no worst-triangle theorem, so it sizes at UNPROVED_HEADROOM = ½ of δ
status: closed
opened: 2026-10-28
priority: P2
cost: M
closed: 2026-11-23
---


Filed by TESS's sphere-arm unit
(`sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`),
which wrote the trimmed lane's sphere arm. It belongs with the analytic
lanes' sizing theorems (TESSSIZE's charter, once that program's slate
exists; filed here until then, beside its cylinder sibling
`the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`).

**Where it is.** `mesh::trimmed::UNPROVED_HEADROOM` (0.5), read by the
sphere arm at `trimmed::sphere_lattice_steps`
(`sizing::sphere_grid_steps(δ·UNPROVED_HEADROOM, r, widest)` over the
face's latitude window) and by `mesh::chords::edge_target`, which
chords every edge of a trimmed sphere face at the same share of δ.
`chords::tighten` then bounds each boundary chord's UV step
by those same lattice steps.

**What is known.** The lattice's cells are the curved lane's sphere
cells, and `sphere_grid_steps`' theorem
(`sizing::tests::sphere_grid_steps_land_the_worst_triangle_at_delta`)
puts a cell-half at the steps' target: at the headroom, δ/2. A trim
chord is a chord of a circle on the sphere, so the triangle on it alone
deviates no more than its own sagitta. What is missing is the
constrained Delaunay triangle BESIDE a trim chord: its apex can be a
lattice point more than one cell from the chord's ends, and nothing
bounds by how much. The CDT runs in the chart's metric at the trim
box's mid latitude (`(u·cos v_mid, v)` to a power of two), which is
near-isotropic only near that latitude, so on a face spanning many
latitudes the cells far from it are anisotropic in the CDT's own
coordinates. The per-triangle certificate (`cert::cert_sphere`) refuses
typed where it is exceeded; the unit's end-to-end row
(`trimmed::tests::an_over_delta_sphere_arm_triangle_refuses_typed`)
shows that refusal.

**What would settle it.** A lemma bounding the 3-D circumradius of a
constrained Delaunay triangle on a trim chord in the arm's lattice, in
terms of the lattice steps and the chord's UV step (which the arm now
bounds by the lattice steps), giving the triangle's deviation
`r − √(r² − R²)` in closed form; then the headroom is that lemma solved
for the step, and the factor goes.

- 2026-11-14 — moved from TESSSIZE to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.


## Closed (2026-11-23, `tesslane/certificate-decides`) — not owed

`a-refused-certificate-adds-a-point-and-no-face-sizes-below-delta` deletes `UNPROVED_HEADROOM`: every chart-lane face sizes at δ from its chart's cell bound, and a triangle whose certificate reads over δ adds a point and the face is triangulated again, under its own round budget (`trimmed::MAX_CORRECTION_ROUNDS`, `crates/mesh/README.md` `[mesh.sizing]`). The bound this row asked for, on the triangle beside a trim chord, was owed only so that the step alone could hold that triangle; the certificate holds it now. Measured over the `mesh` suite, `sweep`'s meshing modules and `step-import` at ε `10⁻⁹`, `10⁻⁶` and `10⁻¹²`: no face is lost to the certificate, and no face takes more than 3 rounds.

Two things stay true. **The correction's termination is a budget, not a proof**: nothing bounds the rounds a face needs, and a face past the budget refuses `CertificateExceeded`. **`a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`** is a walked face, sized by the walk's own theorems, and this closing does not touch it.
