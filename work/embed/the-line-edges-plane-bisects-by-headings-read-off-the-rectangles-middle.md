---
id: the-line-edges-plane-bisects-by-headings-read-off-the-rectangles-middle
kind: issue
title: planes_through bisects a line edge by each face's heading to its rectangle's middle; a rectangle with no width across the edge gives a heading whose across part is rounding, and the other bisecting plane is taken
status: open
opened: 2026-11-23
priority: P1
cost: M
---


## Finding

Measured by EMBED's lane `embed/wedge-measure` (branch `embed/wedge-measure` at `6c403617c7`, `measure/wedge/REPORT.md` §4; code read on PR 7329's head), filed by the EMBED orchestrator from its report.

Over the per-solid census (1,301 tests), 19,190 crease cell pairs that the embedding question refuses for good have **both faces definite alone and on the same side of the separating plane**: the plane separates nothing. 19,164 of them are at the bisecting plane of one straight shared edge, **all in one test**: `sweep parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region` (edge `14v1`, faces `3v1` / `8v3`; also `8v1`, `13v1`, `15v1`, `3v1`), cylinder against cylinder, a concave crease whose departures are 90° apart.

`planes_through` builds a line edge's plane from the two faces' **headings**: `heading` is the vector from the edge's start to the middle of the face's cell rectangle (`crates/topo/src/injective.rs`, `heading`, read where the line arm bisects, through `bisector`). In this test one face's rectangle has almost no width across the edge (`u` 1.5708 to 1.5708 at four decimals), so its middle lies 2.3e-10 to 3.6e-7 m off the edge's line with the anchor 0.5 m along it. The heading is then `(along 1.0, across 2.3e-10)`, and its across part is anti-aligned with the face's real departure (cosine −1.000). `bisector` returns the **other** bisecting plane, which both faces leave at 45° on one side (`sign_on` reads `+0.707 / +0.707`). Under the right bisector the same `sign_on` reads `Positive / Negative` at ±0.707 on all 19,184 probed rows.

Not unsound: a plane with both faces on one side certifies nothing, so the pair refuses. A valid body refuses for good.

## The other 26

At a conic's own plane (`pl`), in four tests (`coincidence_door::the_coaxial_stage6_unions…`, `coincidence_door::every_band_a_revolve_mints…`, `germ_plane_torus_chord::tangs_tube_on_a_latitude…`, `bisector_nodes::a_rim_vertex_on_a_node…`): that plane is not a bisector and both faces leave it on one side. Under the bisector of the two departures they read opposite where the departures are 16° to 51° apart, and one face is undecided within 0.4° of tangent. A different matter: which separator is offered at a curved crease, not how a bisector is chosen.

## What a fix needs

The side a face leaves an edge on is a fact about the face at the edge, not about where the cell's rectangle happens to sit. Two readings exist that do not depend on the rectangle: the face's inward departure at the edge (what the measuring lane computed as `ux`, `uy`), and, once `embed/verdict-read` lands, tier 3's own turn for the edge on the finished body, which that unit already requires a proved sign to agree with. The unit that takes this row says which the plane is built from, and pins the thin-rectangle cell (a rectangle whose width across the edge is a rounding) reading the same plane as a wide one.

## Evidence

- Reproducer: `sweep parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region` with the at-rest gate asking (the census harness on `origin/embed/gate-measure`).
- Checked: the headings, the chosen plane and both planes' signs, printed, on five tests. Not checked: whether any test certifies wrongly (none can by this route), and whether other heading-built separators (`planes_through`'s vertex arm, the wedge plane) take a rounding the same way.
