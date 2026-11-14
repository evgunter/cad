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

## Measured (2026-11-14; `crates/mesh/tests/lift_probes.rs`)

Both things this row called unmeasured are run, on the bodies the reach probes build.

- **The spiric-bounded torus wall's outer loop is a simple polygon** on all three walls (the vessel quarter, both faces of the klein elbow): no crossing or touch at 8, 32 or 128 chords an edge, all four joints meet, the winding is (0, 0), and it runs clockwise on `sense` false, which is what `tessellate_trimmed`'s orientation check asks of an analytic chart.
- **A lift runs along a cone's apex row through the face's turn there, as at a sphere pole.** On each half of a full cone the arriving generator ends at `(u, 0)` and the leaving one starts at `(u ∓ π, 0)`, both at `v == 0.0` exactly; on a wedge the step is the wedge's angle. What `trim_polygon` does with it today is not that: `trimmed::at_pole` is false at `v = 0`, and `flat_points` keeps a traversal's last point only under `ChartPoles::Kept`, which only the sphere arm passes, so a cone polygon would keep one apex point, at the leaving azimuth.
- **`Lift::window` and `azimuth_window` answer `None` on exactly the loops this unit adds**: every loop with a `Spiric`, `Projected`, `IsoLine` or `IsoArc` row. They answer only where every row is `Harmonic` or `FocalSection`. The face's window therefore comes from the image boxes unit 1 supplies.
- Every one of the 181 door-built curved loops sampled reads a lift that closes and is a simple polygon, counter-clockwise on an outer loop exactly when `sense` is true.
