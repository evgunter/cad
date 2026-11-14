---
id: one-chart-table-and-the-lane-takes-every-analytic-chart
kind: unit
title: one table row per chart, and the trimmed lane meshes a cone, torus or spindle face from it
status: dispatched
opened: 2026-11-14
priority: P0
cost: H
branch: tesslane/chart-table
---


Unit 2 of the chart lane (`work/tesslane/plan.md`, The chart lane). Follows unit 1.

## What it is

What the mesh knows about a chart is one table row per chart (cylinder, cone, sphere, torus, spindle sheet, spline): the lattice steps for a target, the per-triangle certificate, the metric the triangulation runs in, the chart's singular rows (a sphere's poles, a cone's apex, a spindle's apices), and its outward normal. Today that knowledge is in five places with different rosters: `sizing.rs`, `cert.rs`, `curved::grid_counts`, `trimmed::Lane`, and the chord tightenings. `trimmed::trim_frontier`'s refusal is two of those rosters disagreeing.

The trimmed lane then meshes a cone, torus or spindle face from its row:

- the sphere arm's pole-side rule (`sphere_polygon_shape`, `at_pole`, `pole_side_columns`) is stated once for any singular row;
- `cert_cone` and `cert_spindle` get the per-corner singular flag the rectangle lane's `UvPoint.pole` carries;
- the fold check's outward direction is the chart's (the cone and torus have none in `TrimmedFace::outward`);
- `sizing::cone_face_rho_max` and `sphere_face_widest_cos` read circle carriers on the outer loop only, so a cone face bounded only by ellipses has no answer; the window comes from image boxes instead.

The new charts size at the lane's existing named `UNPROVED_HEADROOM`; no new debt opens (unit 5 is the theorem).

## Closes

`trimmed-tessellation-lacks-torus-and-plane-arms`: a torus wall bounded by a spiric and a cone face cut by an ellipse mesh and pass `check_mesh`, and `reach_probes::a_spiric_bounded_torus_wall_refuses_at_the_chart_roster` and `a_cone_face_bounded_by_a_section_ellipse_refuses_at_the_chart_roster` are re-pinned.

## Unmeasured, owed before the spec

Whether the torus wall's spiric-bounded loop lifts to a simple polygon, and how a lift reads at a cone apex (the lift measurement, branch `tesslane/measure-lift`).

## Unit (2026-11-14, TESSLANE)

**Letter H. Tier: dual review.** It is the first unit of the chart lane that turns a refusal into an emitted mesh, on three charts with singular rows, and its content is certificates and a pole rule generalised from one chart to four: a mesh that is wrong at an apex or folded on a torus can pass `check_mesh`, so believing it takes claims to falsify and a second reading.

Branch `tesslane/chart-table`, cut from `tesslane/chord-rule` (unit 1, in review): merge that branch as its fix pass lands, and main after it merges.

**Owed by the unit, beyond the sections above:**

- **Measure before building.** Run `reach_probes` and `lift_probes` on the branch head and say in the PR body where the two closing probes refuse today and what each of the five rosters holds per chart. The row above was written from reading; where it is wrong, the measurement wins and the PR says so.
- **The table first, with nothing moving.** The first commit is the table and its readers: `sizing.rs`, `cert.rs`, `curved::grid_counts`, `trimmed::Lane` and the chord pass's `FaceLattice` read one row per chart, and the faces that mesh today mesh to the same bytes (the D9 goldens, `docs/tess-budget-data` and the tour pins pass unedited). A byte that moves there is a stop: report it, do not re-baseline it. The new arms come after, as their own commits, so each can be read apart.
- **A chart the table lacks does not compile.** Each reader matches the chart exhaustively or reads a field every row must fill; no `_ =>` arm answers for a chart nobody thought about. `trim_frontier`'s roster check goes away because nothing can disagree, not because it is deleted.
- **The singular-row rule is one rule.** The sphere arm's pole handling is restated for "a row of the chart where a whole parameter line is one point", and the sphere reads it. The apex facts measured above are its test: `at_pole` is false at a cone's `v = 0` today, and `flat_points` keeps one apex point where the lift has two. Say what the rule is in the doc beside it, and pin it per chart with a row that reds when the apex or pole entries are dropped or doubled.
- **The window comes from the boundary images' boxes** (`Pcurve::chart_box`, unit 1's second fact), on every chart. `cone_face_rho_max` and `sphere_face_widest_cos` read it too, or the PR says why a circle-only reading stays.
- **Done when** a torus wall bounded by a spiric and a cone face cut by an ellipse mesh at both δ the probes use, pass `check_mesh`, and their volume or area lies in a bracket the row derives (an inside-out or folded patch must red it); the two `reach_probes` rows are re-pinned to that; and a spindle sheet face either has the same row or the PR names, measured, why no door builds a trimmed one today.
- **This unit is the first reader of three of unit 1's bounds.** At unit 1's head no lane reads a spiric, focal-section or off-sphere projected image's `speed_bound`: the sphere arm admits `Harmonic` and `Projected` rows, and the spline arm refuses all three forms before any bound is read. Unit 1's reviews measured the spiric bound loosening as a torus's tube fattens (the sampled speed is 0.33 of the wall's bound and 0.04 of the cap's at r/R near 1), in the safe direction; its fix pass is on that. The PR body gives, for each body the closing probes build, the chord count each boundary edge is asked for and the count a sampled speed would ask, so an over-chorded boundary is seen rather than shipped.
- **Not this unit:** a ring still refuses (unit 3), and a face is still routed by its outer loop's carriers (unit 4). The ring guards and `lane_of` are not edited beyond what the table forces. The new charts size at the lane's existing named `UNPROVED_HEADROOM`; no constant is added and no bullet of `crates/mesh/README.md` `[mesh.sizing]` changes what it decides. If a sentence there names a mechanism this unit moves, re-word that sentence and say so in the PR body; anything more is a stop.
- Gates are run as CI runs them: `bash scripts/run-gates.sh`, which sets the base the gates diff against.

