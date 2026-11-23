---
id: pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face
kind: issue
title: A vertex the band places ON a cone's apex at one ε and OFF it at another is meshed two ways at one δ: the guard DESIGN says pole identification owes is not built
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by unit 4c's implementer (PR 7096) in its sweep for places where ε chooses between two meshes, 2026-11-23. The trigger is main's (one classification of one vertex against the apex); on a head with the walk its consequence is wider, as a review of that pull request noted.

## What

`docs/DESIGN.md` `[tolerance.chordal]`: "The mesh layer never reads ε for sizing … the one ε read that is a classification — pole identification — owes a guard so an ε flip can never move emitted coordinates with δ held fixed (#896)."

The read is `AnalyticRow::is_singular` at the run's band, through `trimmed::singular_vertices` on the structural arms and through the route's apex fact (`chart_table::at_a_singular_point`, `trimmed::walkable`) on the walk. Where the band cannot place a vertex on or off the apex, both refuse `SphereTrimShape { SingularUndecided }`: a refusal against a mesh, which the page allows.

Where the band DECIDES the vertex on the apex at one ε and off it at another (a split nearer the apex than the band at 1e-6 and farther than it at 1e-12), the face is meshed two ways at one δ: with an apex row, or with the vertex as a rim point. On the structural arms that is two polygons in one lane at one target. With the walk it also switches the LANE (walked against structural), and with the lane the face's target (δ against the headroom's share of it) and so the chords of the rims its neighbours share. `split_rulings::a_split_beside_the_apex_of_a_needle_cone_refuses_undecided` pins a cell that refuses at the default ε and meshes walked at 1e-12. That is the case the page says owes a guard, and no guard is built on either lane.

## What closing it takes

A body that reaches it, pinned at two ε (a cone face with a side split within 10ε of its apex at the larger ε); then the guard the page names: a decision that does not move emitted coordinates when ε moves, or a refusal on the cells where it would.

## A second reader (2026-11-23)

The spline arm's collapsed sides are a second reader of this classification
(`tesslane/collapsed-side`, PR 7517): `trimmed::CollapsedSides::holds` decides a
loop vertex on a spline chart's collapsed side at the run's band, through the
one read `chart_table::at_a_singular_point`, and the polygon walks the side
through the vertex where it is decided on. It owes the same guard. On every
body a door builds the vertex is the side's own stored point, decided on at
any band, and `trimmed::collapsed_rows::the_class_meshes_to_one_digest_at_every_eps`
pins the loft's class to one byte stream at ε 1e-9, 1e-6 and 1e-12. A vertex
within the band of a side at one ε and outside it at another (an import could
write one) would mesh with the side walked at the first and held once at the
second.

## The spline witness, and why this is P2 (2026-11-23)

The delta review of PR 7517 built the case this section's last sentence supposed, and ran it.

- **Body:** the wedge (a square lofted to a hinge at 0.5, v-degree 1), every stored vertex point restated by +1e-8 (`topo::test_support::restate_geometry`), so the vertex on the collapsed side sits 1e-8 off the side's point. Meshed with `mesh::tessellate` at δ 0.1, once per ε (`CAD_TOLERANCE_EPS`).
- **ε 1e-6:** 248 triangles, the side walked, digest `25e3945334089fe8`. **ε 1e-9 and 1e-12:** 240 triangles, the vertex held once, digest `53228aa5bd157cc1`. Both meshes are closed (`check_mesh`).
- The same split at δ 3e-2 (656 against 648) and at an offset of 1e-7. At offsets 1e-10 and 1e-11 it is ε 1e-12 that parts from the other two. At 1e-13 and 1e-5 every ε gives one digest. Where the band cannot decide (offset 1e-9 at ε 1e-9, 1e-6 at ε 1e-6) the face refuses `SphereTrimShape { SingularUndecided }`, typed.
- Topo's lift and the mesh decided every vertex of the sweep the same way: they call one predicate at one band.
- On main before PR 7517 the spline arm read no side, so this input meshed to the same bytes at every ε. That PR made it a second instance of this debt; it merged with the debt named, since no door builds such a vertex (the loft writes a pinned corner as the side's own point).

Raised from P3 to P2 on that: two readers, a witness that runs, and an instance that a merged change added rather than inherited.

Three things the same review read beside it, for whoever builds the guard:

- **Where the guard most likely lives.** `docs/DESIGN.md` `[extensibility]` gives the constructor's half ("a constructor that decides such a side collapsed at its band builds it exactly one point"). Nothing gives the vertex's half: a constructor or an import that decides a vertex on a singular point at its band writes it AS that point. With that, every reader's band answer is the same at every ε, for a pole, an apex and a collapsed side alike. No row tracks it but this one.
- **A sentence of the mesh page that waits on the guard.** `crates/mesh/README.md` `[mesh.winding]`: "No ε is read. The mesh stays a function of (body, δ), as `[tolerance.chordal]` rules." The bullet is the winding verdict's, where it is true; its second sentence is general, and pole identification is the exception `[tolerance.chordal]` itself names. It is true of the whole mesh once the guard exists.
- **The corner's own error.** A vertex decided ON a singular point while up to the band's reach from it keeps its own position in the emitted mesh, not the point's. The mesh there is off the surface's singular point by up to that reach (a multiple of ε), against a promise read as δ plus ε. By inspection, at analytic poles on main and at spline sides since PR 7517.
