cone-axis-winding — FINAL REPORT, part 3 of 3: the PR body, second half. It ends at the ===== line.

## Rows

- `curved::tests::a_ruled_face_with_a_split_ruling_meshes` (new, slow set, 0.52–1.0 s locally). Bodies: a full and a half cylinder wall, cones through their apex at 45°, 80° and 5°, and a frustum. Each ruling is split at one point and at two, placed obliquely, at δ 0.1, 0.01 and 0.001. Each cell meshes, passes `check_mesh`, and encloses the unsplit body's volume to δ times the two meshes' area. The rulings include a full turn's seam, which its one face walks twice. It passes at the default ε, 10⁻⁶ and 10⁻¹².

## Mutants

- **Route removed** (the `lane_of` arm deleted). Red: `a_ruled_face_with_a_split_ruling_meshes`, `a_cone_face_met_on_its_axis_meshes_at_every_delta`, `a_multiply_carried_iso_side_…` and `a_split_then_placed_swept_face_is_not_refused`. The class faces reach the rectangle lane and refuse `SPLIT_SIDE_ROUTED_HERE`.
- **Route widened to every cylinder and cone face.** Red: 26 rows, among them both `d9_mesh_goldens` rows (`every_corpus_body_meshes_to_its_committed_digest` and `the_corpus_digests_the_same_bytes_at_one_and_four_threads`).
- **Same-kind across a separated junction** (the walk's own run rule, read instead of the `v` order). Red: `d9_mesh_goldens`' `slit` at δ 0.05 and 0.15, `iso_rectangle_door::a_zero_width_slit_…`, and both `loops_with_no_rim::…_refuses_single_column` rows. That rule catches a slit and a one-generator loop, which are no class faces.

## Named refusals

- **A one-face full-turn wall with a split seam** does not reach `SeamTwice`. It routes to the chart lane and meshes; it is in the new row (full cylinder wall and the three full cones).
- **A face on both nappes (`BothNappes`)**: no door builds one. The one body that holds one, the hand-built `witness_bodies::apex_crossing_bowtie`, refuses at the shape door (`props_rim_side`) before the route is asked, whether or not its generator is split at the apex (measured with `split_edge`). That refusal is unchanged.

## §5 sweep

The shape grepped for:
- a lane arm whose triangulation assumes a premise about the boundary that the route does not establish (the rulings' "bare sides");
- a chart-plane CDT in which one point lies on every column's straight line (an apex or a collapsed row).

Pattern: `grid_counts`' arms, `CdtPlane`, `singular_points`, `pole_columns`, `collapsed|degenerate row` over `crates/mesh/src`.

| hit | disposition |
|---|---|
| `curved::grid_counts` cylinder arm (`(nu, 1)`, theorem needs bare sides) | fixed: such a face is routed away; reaching it refuses typed |
| `curved::grid_counts` cone arm (fallback lattice) | fixed: deleted |
| `curved::grid_counts` sphere arm | not this class. Its sides are arcs that already carry chord points on its row lattice, and pole + two nodes of one meridian is not collinear (a great-circle arc). Measured: ball and sphere band, every edge split at {0.5}, {0.3129, 0.15645} and {0.9}, δ 0.1 to 0.001, all clean |
| `curved::grid_counts` torus arm | not this class: a split seam meridian is folded back by props' torus parse, and a split rim is a row point (`TOTAL_REFUSED`'s doc) |
| `curved::grid_counts` spindle arm | not this class, and no door builds a spindle face (`revolve` refuses it) |
| `trimmed` cone arm (`CdtPlane::Development`) | not reachable: `developed` puts every point of one column on one exact line through the origin, so spade cannot make the triangle |
| `trimmed` cylinder and sphere arms (`CdtPlane::Lattice`) | not this class: no point lies on every column; a split vertex is a trim point like any other, held clear of by `lay` |
| `trimmed` spline arm (`CdtPlane::Lattice`, `ChartRow::Spline`) | gap, below |

What the pattern could not match: a NURBS face whose boundary row collapses to one point (a spline cone or a spline pole). The spline arm triangulates in the lattice plane and lists no singular points, so such a face would put one point on every column. Second pass: grepped `crates/mesh` and `crates/geom-brep` for collapsed or degenerate spline rows and for spline poles and apexes. No fixture, door or row builds one, so there is nothing to measure; `[mesh.winding]`'s refusal at the join is the backstop if one arrives. Not filed: there is no body.

## Tracker

- Filed and closed: `work/tesslane/a-cylinder-wall-with-a-split-ruling-exceeds-its-certificate.md`.
- Re-pointed: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`. Its arm is deleted; the face is the chart lane's at the headroom until the strip unit.
- Added to the closed `curved-lane-triangulates-in-raw-chart-coordinates`: its closing premise was false for a split vertex.
- `one-element-grid-axes-drop-schedule`'s split-side member now points at the route.
=====
