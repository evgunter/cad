---
id: the-sphere-trim-shape-refusal-names-the-sphere-on-every-singular-row-chart
kind: issue
title: TessellateError::SphereTrimShape names the sphere but is raised on a cone or spindle sheet face too
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by `one-chart-table-and-the-lane-takes-every-analytic-chart` (branch `tesslane/chart-table`).

`mesh::trimmed::singular_row_shape` states the sphere arm's pole-side refusals once for every chart row with a singular row (`mesh::chart_table`): a sphere's poles, a cone's apex, a spindle sheet's apexes. It raises `TessellateError::SphereTrimShape { cause: PastPole | SeamTwice | BothNappes | SingularUndecided }` (a loop round a singular point inside the face, once `PoleInside`, now refuses `DomainUnread` at the ring door), and `chords::FaceLattice::speeds` raises `SphereTrimShape { cause: ThroughPole }` for an image through a singular row on any of those charts. The variant's name and `SphereTrimCause`'s say sphere and pole; on a cone face the row is the apex.

The variant is public: `crates/pncad-py/src/py/mesh.rs` and `crates/pncad-py/src/tags.rs` (`"sphere_trim_shape"`) project it, `scripts/payload-rung-sweep.py` names `SphereTrimCause`, and `crates/mesh/tests/errors.rs`, `crates/sweep/tests/tilted_sphere_pair.rs` and `crates/sweep/tests/spheres_crossing_off_every_edge.rs` match it. A rename to a chart-neutral name (and `ChordBudget` staying the sphere's) is a public-surface change with a binding tag, not done in the unit that widened what the variant covers; the variant's docs say what it covers now.
