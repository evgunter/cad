cone-axis-winding — FIX PASS REPORT, part 3 of 3: the PR body, second half. It ends at the ===== line. My runs and notes follow after it.

## Rows

All in `crates/mesh/tests/split_rulings.rs` but one, and each red under the mutant named:

- `a_ruled_face_with_a_split_ruling_meshes` (slow set, 4.6 s locally): 243 cells. One-face full turns, which walk their seam twice: a cylinder wall, a frustum wall, a slender wall and a needle wall, each face outer and reversed inner. Faces through the axis, which a full revolve builds as two half faces: cones through their apex at 45°, 80° and 5°, and a frustum; and a half cylinder wall. Each line edge split at one point, at two and at five, and every line edge at once at one point; placed obliquely; δ 0.1, 0.01, 0.001. Every cell meshes and passes `check_mesh` and `reach_probes::geometry` (every winding resolved and outward, every triangle sampled within δ of its surface, the volume the unsplit body's to the chordal deficit, read off the unsplit body since a body `split_edge` leaves a vertex on is not finished), but the three thin-annulus cells pinned as `Triangulation(Plane)`. The same at the default ε, 10⁻⁶ and 10⁻¹². Red with `SeamTwice` restored.
- `a_notched_wall_with_a_split_ruling_refuses_at_the_shape_door`: `reach_probes`' notched pipe wall with each line edge split, the notch's rulings among them, refuses `NotIsoRectangle` at the door as the unsplit wall does. Red with the route's door confirmation dropped (the notched wall would mesh on the chart lane).
- `curved::tests::a_ruled_face_with_a_point_inside_a_side_refuses_here`: a hand-built call of the rectangle lane on a cylinder wall with a split seam (the vertex stands in for the chord point no body builds) refuses `UnsupportedCurve { note: SIDE_CHORD_POINT }` on the line edge. Red with the guard deleted (the lane meshes it on its `(nu, 1)` grid).
- `a_routed_face_is_chorded_for_the_chart_lane`: a cylinder wall and a frustum wall, seam split, δ 0.01, pinned to their triangles per patch. With the chord pass blind to the route (the class face chorded as the rectangle lane would), every count falls (the cylinder wall's 314, 64, 64, 64 to 296, 55, 64, 55), and the thin-annulus cells of the first row mesh. That is what the chord pass's reading of the route decides: density, and through it where the thin-arc class lands.
- `a_ruling_split_a_hair_from_its_end_folds_a_triangle` and `a_ruling_split_within_the_developments_resolution_refuses`: the two residuals above, pinned as today's answers, each stating what it expects at each ε.
- `reach_probes::a_cone_face_met_on_its_axis_meshes_at_every_delta`: T1's 18 cells.

## Mutants

- **Route removed** (the `lane_of` arm deleted). Red: `a_ruled_face_with_a_split_ruling_meshes`, `a_cone_face_met_on_its_axis_meshes_at_every_delta`, `a_multiply_carried_iso_side_…` and `a_split_then_placed_swept_face_is_not_refused`.
- **Route widened to every cylinder and cone face.** Red: 27 rows (the review's count), among them both `d9_mesh_goldens` rows.
- **Same-kind across a separated junction** (the walk's own run rule, read instead of the `v` order). Red: `d9_mesh_goldens`' `slit` at δ 0.05 and 0.15, `iso_rectangle_door::a_zero_width_slit_…`, and both `loops_with_no_rim::…_refuses_single_column` rows.
- **`SeamTwice` restored on the development.** Red: `a_ruled_face_with_a_split_ruling_meshes`, `a_routed_face_is_chorded_for_the_chart_lane`.
- **The route's door confirmation dropped.** Red: `a_notched_wall_with_a_split_ruling_refuses_at_the_shape_door`.
- **The rectangle lane's side guard deleted.** Red: `a_ruled_face_with_a_point_inside_a_side_refuses_here`.
- **The chord pass blind to the route.** Red: `a_ruled_face_with_a_split_ruling_meshes` (the thin-annulus cells mesh), and the counts pinned by `a_routed_face_is_chorded_for_the_chart_lane` move.

## Named refusals

- **`SeamTwice`** no longer stands between a one-face full-turn cone wall and the chart lane; such walls mesh (the first row).
- **A face on both nappes (`BothNappes`)**: no door builds one. The one body that holds one, the hand-built `witness_bodies::apex_crossing_bowtie`, refuses at the shape door (`props_rim_side`) before the route is asked, whether or not its generator is split at the apex (measured with `split_edge`). That refusal is unchanged.

## §5 sweep

The shape grepped for:
- a lane arm whose triangulation assumes a premise about the boundary that the route does not establish (the rulings' "bare sides");
- a chart-plane CDT in which one point lies on every column's straight line (an apex or a collapsed row).

Pattern: `grid_counts`' arms, `CdtPlane`, `singular_points`, `pole_columns`, `collapsed|degenerate row` over `crates/mesh/src`.

| hit | disposition |
|---|---|
| `curved::grid_counts` cylinder arm (`(nu, 1)`, theorem needs bare sides) | fixed: such a face is routed away; a chord point inside a side refuses typed |
| `curved::grid_counts` cone arm (fallback lattice) | fixed: deleted |
| `curved::grid_counts` sphere arm | not this class. Its sides are arcs that already carry chord points on its row lattice, and pole + two nodes of one meridian is not collinear (a great-circle arc). Measured: ball and sphere band, every edge split at {0.5}, {0.3129, 0.15645} and {0.9}, δ 0.1 to 0.001, all clean |
| `curved::grid_counts` torus arm | not this class: a split seam meridian is folded back by props' torus parse, and a split rim is a row point (`TOTAL_REFUSED`'s doc) |
| `curved::grid_counts` spindle arm | not this class, and no door builds a spindle face (`revolve` refuses it) |
| `trimmed` cone arm (`CdtPlane::Development`) | not reachable: `developed` puts every point of one column on one exact line through the origin, so spade cannot make the triangle |
| `trimmed` cylinder and sphere arms (`CdtPlane::Lattice`) | not this class: no point lies on every column; a split vertex is a trim point like any other, held clear of by `lay` |
| `trimmed` spline arm (`CdtPlane::Lattice`, `ChartRow::Spline`) | gap, below |

What the pattern could not match: a NURBS face whose boundary row collapses to one point (a spline cone or a spline pole). The spline arm triangulates in the lattice plane and lists no singular points, so such a face would put one point on every column. Second pass: grepped `crates/mesh` and `crates/geom-brep` for collapsed or degenerate spline rows and for spline poles and apexes; no fixture or row builds one. The grep is blind to the doors most likely to make one, STEP import (a `B_SPLINE_SURFACE` with a degenerate edge) and loft (a section shrunk to a point). `[mesh.winding]`'s refusal at the join is the backstop. Filed: `a-spline-face-whose-boundary-row-collapses-to-a-point-has-no-singular-row`.

## Tracker

- Filed and closed: `work/tesslane/a-cylinder-wall-with-a-split-ruling-exceeds-its-certificate.md`.
- Re-pointed: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`. Its arm is deleted; the face is the chart lane's at the headroom until the strip unit.
- Added to the closed `curved-lane-triangulates-in-raw-chart-coordinates`: its closing premise was false for a split vertex.
- `one-element-grid-axes-drop-schedule`'s split-side member now points at the route.
- Filed (P3): `a-ruling-split-a-hair-from-its-end-folds-a-triangle`, `a-ruling-split-within-the-developments-resolution-merges-two-points`, `a-spline-neighbour-can-put-a-chord-point-on-a-ruled-faces-straight-side`, `a-spline-face-whose-boundary-row-collapses-to-a-point-has-no-singular-row`.
- Evidence added: `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta` (the slender wall's annulus).

## Not this PR's

- A face's lane is decided per edge per side in the chord pass (`lane_of` called from `chords::tighten` and `FaceLattice::of`); deciding it once per face is PR 7041's (`Routes::of`).
=====

RAN LOCALLY on the pushed head 58565e3f0d. All green, slow set included (default nextest profile):
- fmt; run-gates; doc-gate --workspace-pass; clippy for mesh and geom-brep (--all-features), the viewer (--features app) and demos/tour.
- mesh 503/503. The split-ruling, T1, side-guard, multiply-carried and split-placed rows at ε 1e-6 and 1e-12: 9/9 each.
- mesh --features budget clippy and test; tools (k-lint, tess-lint, tess-meter: fmt, clippy, test).
- step-import 329/329; stl 18/18; sweep's 63 meshing modules 584/584; viewer 1151/1151 (7043's re-pinned rows included); demos/tour whole with --cargo-profile ci-tour 106/106 (1233 s).
Left to CI: the nightly rows.

What slowed me: the residual's attribution needed instrumenting all three lanes' Triangulation sites. The first class row had to move to an integration module, because `reach_probes::geometry` reads a finished body's volume and a split body is not finished. The disk filled from three targets, so I wiped and rebuilt.
