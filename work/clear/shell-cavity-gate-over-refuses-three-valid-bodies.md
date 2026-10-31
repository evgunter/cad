---
id: shell-cavity-gate-over-refuses-three-valid-bodies
kind: issue
title: SHELL-4's cavity gate over-refuses three valid bodies: the corpus pot at Interval (sphere/cylinder vertex Departure), the klein elbow's cavity (Departure at both scalars) and the cone frustum's cavity (pair budget)
status: dispatched
opened: 2026-10-30
priority: P1
cost: H
branch: clear/cavitydiag
---


## Finding

SHELL-4's lane, `clear/shell4` at `7d97070458`. The gate is
`cavity_embedded` in `crates/topo/src/shell.rs`: a strictly-positive
self-intersection sweep over the moved clone, with a 65 536-pair budget.
It turns three existing green rows red. Each body is valid:

1. **The corpus pot**, a spherical belly over a cylinder foot.
   - Row: editor-core
     `lib_g17_shell_node::both_documents_evaluate_green_at_the_interval_scalar`.
   - The OPERAND itself refuses at `Interval`:
     `Uncertified { 3v1 sphere, 7v1 cylinder, Vertex(3v1), Departure }`,
     `cornered = 7`.
   - At `f64` it holds, `cornered = 0`.
   - This is RIMSCALAR's class (the `Interval` reading diverges from
     `f64`) at a new site: a sphere and a cylinder meeting at a vertex.
2. **The klein elbow.**
   - Row: sweep
     `torax_interval::interval_the_klein_elbow_hollows_to_its_measured_volume`.
   - The operand holds at both scalars.
   - The shelled cavity refuses `Uncertified Departure` at both,
     immediately, after 0 splits.
3. **The cone frustum.**
   - Row: sweep
     `sf2b_interval_probe::interval_offset_charts_together_cone_frustum`.
   - The operand holds at both scalars.
   - The cavity exhausts `Budget(Pairs 65536)` at both, taking 8.5 s,
     with `outside = 41 429`.

**SHELL-4 is parked on this row.** Over-refusing legitimate shells is
ruled out by `shell-curved-wall-clearance-window`. Unit CAVITYDIAG
(`docs/CAVITYDIAG-SPEC.md`) diagnoses each of the three and fixes what
it can.

## Diagnosis (CAVITYDIAG)

Each body is reproduced on `origin/main` by running the sweep on the
operand, or on the shelled body's cavity faces. The three rows live in
`sweep`'s `self_intersection_proves_injectivity`.

### 1. The corpus pot: class (a), fixed

- **Pair:** `3v1` (sphere half) and `7v1` (cylinder half), sharing
  `Vertex(3v1)`, where the seam meets the foot–belly junction circle.
  **Arm:** `Departure`.
- **At `Interval`:** refused, `cornered = 7`, 853 splits.
  **At `f64`:** holds, `cornered = 0`, 472 splits.
- **Cause:** the junction circle's pcurve on the cylinder (edge 13) is
  `v: Free { c: 0, a, b: 0, l: 0 }`.
  - At `f64`, `a = 0.0`.
  - At `Interval`, `a ∈ [−6.9e−18, 6.9e−18]`. That is the rim's `u_ref`
    (`y ∈ ±1.1e−16`) dotted with the point axis `(0, 1, 0)`, in
    `pcurve_cache::derive_class`'s cylinder arm.
  - `topo::clearance::iso_of` tests `exactly(a, 0.0)`. So at `Interval`
    the junction is not the line `v = 0`, but a curve the corner
    presentation does not read beside the seam.
- **Evidence:** a reader accepting a zero-holding `a` turns the
  `Interval` receipt into `f64`'s, bit for bit (`Holds`, 472 splits,
  `cornered = 0`).
- **The same cause** is behind the `cornered = 8` route that RIMSCALAR
  pinned on SHELL-4's vessel at `Interval`.
- **Fixed at the source.** In `pcurve_cache`'s cylinder moving arm,
  `cylinder_rim_axial` writes a rim's axial terms as zero where both
  are enclosures holding zero and their sup is decided `Zero` at
  `pcurve_cylinder_chart_axial`. That is the cone arm's
  `pcurve_cone_chart_axial` read on the cylinder; `f64` is unmoved.
- **A route split it does not remove.** On a cylinder whose axis is not
  a coordinate axis, `f64` rounds a rim's axial terms to nonzero values
  and reads a curve. `Interval` now reads the line. On main the two
  scalars read bit-identical receipts there.
  - Nine pipe rows (`a_seamless_pipe_cut_by_an_oblique_plane_measures`)
    split from a bit-identical `f64` receipt on main: all five
    `[1.5, 2.5, −0.5]` poses, four of five `[1000, −700, 400]` poses, and
    the `[1.5, …]` mitre. The fifth `[1000, …]` pose was already apart on
    main (351 vs 347 discharged) and moves too (to 445), as does the
    axis-aligned `[0, 0, 0]` mitre (260 → 438). The `Interval` work
    grows, for example 17 → 436 discharged, and the mitre 1158 → 1984.
  - Every verdict class still matches.

### 2. The klein elbow's cavity: class (b), not built

- **Pair:** cap `5v1` (plane `x = 1.15`) and torus `7v1`, sharing
  `Edge(11v1)`. **Arm:** `Departure`. Refused at the root, 0 splits, at
  both scalars. The operand holds.
- **Cause:** the cavity's caps are the operand's meridian caps moved
  0.05 along their normals. That moves them off the torus's axis, so
  each meets the cavity's torus in a **spiric** section, not a circle.
  - The plane side: `present_plane` reads lines, and conics through
    `chart_curve`, and a spiric is neither. `Unpresented::Structural`.
  - The torus side: the edge's pcurve on the torus chart is not
    harmonic, so `u.harmonic` is `None`. `Structural`.
  - With both sides structural, `departure` refuses at once.
- **The design problem.** The chart is the torus's, against a plane
  carrier, across a spiric shared edge. A presentation would have to
  argue:
  - **(i) The torus face, in a cell beside the edge.** It is the region
    on one side of the edge's chart curve, which is no coordinate line
    and no harmonic row. It is the zero set of
    `ψ(u, v) = (S(u, v) − o)·n` on the torus chart: a quartic, not a
    graph of either axis everywhere. So the region needs a graph
    reading of that curve per cell, monotone over one axis, with the
    curve's turning points as cell boundaries.
  - **(ii) Departure.** The cap's own plane is the separator, `ψ`
    above. Its sign on the torus part is read off `∂ψ` across the
    curve, and that is nonzero wherever the cap and the torus are
    transversal. The plane's own part needs nothing: `ψ = 0` on it.

  So the carrier route needs only (i), the torus side presented beside
  a spiric. Reading the plane side, a planar non-conic in the plane's
  chart, is the alternative. Torus charts are not read by the corner
  arm at all today (`reading`'s note).

### 3. The cone frustum's cavity: class (c), not built

- **Pair:** the two cavity cone halves, `1v3` and `8v1`, across their
  seams. `Budget(Pairs 65536)` at both scalars, about 10 s each in
  debug; every other face pair holds within a few pairs.
- **Not cavity-specific.** A frustum operand with top radius `3/64`
  exhausts the same budget at both scalars. The `2/64` operand holds
  only because its top slant, `0.1288`, is exactly half its base slant,
  `0.2577`: the window's first `v` bisection lands on the rim. Its apex
  cells are then within `few_ulps` of the extent, and read `Vacuous`
  across the seam.
- **Where the cells go.** About 179 000 exhibit-arm splits, all on cell
  pairs `v ∈ [−ε, 0]`, the apex, at every depth to 30 and beyond.
  - `clearance::window_of` widens a cone's `v` window to the apex's
    `0`, which a face may hold. This face does not: its extent is
    `v ∈ [−0.2164, −0.1037]`.
  - A cell holding the apex is never `Outside`: `cell_region` must
    clear the carrier's singular point, the apex, and it never can.
  - Once the cell is clear of the seam edges, the pair is not touching,
    so it goes to the distance test. The two halves' apex cells
    coincide in space, the separation is `[0, …]`, and they are split.
- **Why they don't discharge.** Nothing on the non-touching path reads
  the face's chart extent (`Boundary::extent`). The touching path does
  (`sheets` → `Vacuous`), which is how the `2/64` operand escapes.
- **The design problem.** Discharge a cell that misses the face's
  extent: its `v` range strictly past the extent's hull, which is a
  superset of the face, so the cell holds no point of it. That reading
  needs an argument, on the non-touching path, that the extent is a
  sound superset of the face there. `Boundary::extent`'s docs give it
  for a loop that does not wrap. Alternatively, stop `window_of` at
  the boundary's own `v` range when the apex is not on the face.
- **Reviewer's view (CAVITYDIAG's review):** of the two, ending
  `window_of` at the boundary's `v` range when the apex is not on the
  face looks the simpler.

## Rows

In `crates/sweep/tests/self_intersection_proves_injectivity.rs`:
- `the_corpus_pot_holds_at_both_scalars`. Red under the fix reverted:
  `Interval` refuses as on main. So is `shell_4s_vessel_holds`.
- In `geom-brep`'s `pcurve_cache`:
  `a_rims_axial_terms_are_zeroed_only_where_both_hold_zero_within_the_band`,
  red under a lax decision, under `b`'s gate dropped, and under the fix
  reverted.
- `the_klein_elbows_cavity_refuses_at_its_spiric_caps`: pinned typed,
  at both scalars.
- `a_cone_frustums_cavity_exhausts_the_budget_at_its_apex`: pinned
  typed, at both scalars, with the `3/64` operand.
