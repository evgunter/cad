cone-axis-winding — FINAL REPORT, part 2 of 3: the PR body, first half. It continues in part 3 and ends at the second ===== line.
=====
# mesh: a ruled face whose straight side holds a point is the chart lane's; the rectangle lane's fallback lattice goes

Closes the interim of `work/tesslane/a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md` ("The designers' answer, and what this unit builds"). Stacked on PR 7043 (`tesslane/lattice-clearance`).

## The triangle

`reach_probes`' T1 pose, the box's edge on the cone's axis, ops `A ∪ B`, `B ∪ A`, `A ∖ B`. The cone face that refused `UnresolvedWinding` is on the **rectangle lane** (`curved::tessellate_curved`), not the chart lane's development. The face is half the cone: chart box `u ∈ [π, 2π]`, `v ∈ [−2.4, 0]` (`s = 0.6`), with the apex row at `v = 0`. The apex is one mesh vertex, entered twice at `(π, 0)` and `(2π, 0)`. Each side ruling is split once, at slant 1.039, by the vertex where the box meets it. So `bare_sides` was false, and `curved::grid_counts`' cone arm laid its fallback lattice: 17 columns by 11 rows at δ 5·10⁻³, and 36 by 23 at 10⁻³.

Every unresolved triangle has the same three corners:
- an apex copy;
- the lattice node `(u₁, −j·h_v)`;
- the lattice node `(u₁, −(j+1)·h_v)`, where `u₁` is the interior column next to the split side.

In the chart this is a real triangle. In space all three corners lie on one ruling. For example, at `s = 0.6`, δ 5·10⁻³, the two nodes sit at exactly 1× and 2× the offset `(−0.0200, −0.1890, −0.1072)` from the apex `(0, 2.0785, 0)`. It has no area by construction.

The chart CDT made it. The side from the apex copy down to the split vertex is one bare constraint about 1.04 long, and Delaunay fans the apex copy down column `u₁` to about mid-side. Every fan triangle that takes two nodes of that column is flat, because the apex lies on every column's ruling and the chart cannot say so. The coarse cells meshed because there the fan reached only one node of `u₁`.

The class is wider than T1. Every cylinder or cone face whose straight side holds a point (a ruling split into several edges, or a cone's apex between two nappes) breaks the rectangle lane's theorem for a ruled face, which holds every triangle between two neighbouring rulings. The split vertex is a boundary point off its grid. Measured on the base, obliquely placed, with one ruling split at one point and at two:

| body | δ 0.1 | δ 0.01 | δ 0.001 |
|---|---|---|---|
| full cylinder wall (split seam) | meshes | `CertificateExceeded` | `CertificateExceeded` |
| half cylinder wall (split ruling) | meshes | `CertificateExceeded` | `CertificateExceeded` |
| cone through apex, 45° | meshes | `UnresolvedWinding` | `UnresolvedWinding` |
| cone through apex, 80° | meshes (one split) / `CertificateExceeded` (two) | `UnresolvedWinding` / `CertificateExceeded` | the same |
| cone through apex, 5° | `UnresolvedWinding` | `UnresolvedWinding` | `UnresolvedWinding` |
| frustum | meshes | meshes | meshes |

## The change

- **The route** (`walk::splits_a_straight_side`, read in `tessellate::lane_of` after the existing arms). It is one function of the face's outer loop, read before any chord exists. On a cylinder or cone, two consecutive line traversals continue one straight side exactly when their far ends lie on opposite sides of the vertex between them in the chart's `v`.
  - That covers a split ruling, and a cone side running through its apex onto the other nappe.
  - Two rulings that meet at an apex as a corner come from one side of it, and so does a ruling the loop walks out and back (a slit, a one-generator loop). Neither is caught.
  - A face that holds such a side is sent to the chart lane only if `curved::require_iso_rectangle_face` admits it. A non-rectangle stays the rectangle lane's to refuse with the door's cause.
  - It reads the props band through the door and no ε.
  - `lane_of` takes the band. `trimmed::face_target`, `trimmed::arm_steps` and `chords::compute_chords` take the run's `SizingTols`, which `tessellate` now mints before the chord pass, so the chord pass and the lanes read one route.
- **The deletion.** `bare_sides` and `curved::grid_counts`' fallback lattice go, so the cone arm is `(nu, 1)` on every face. A cylinder or cone face that reaches `tessellate_curved` with a point between a side's rows refuses `MissingEntity { what: SPLIT_SIDE_ROUTED_HERE }`, the router's defect, beside `RINGED_FACE_ROUTED_HERE`.
- `UnresolvedWinding` at the join is untouched.

**Why the lane's existing rules did not reach this.** The chart lane's cone arm triangulates in the development, where three points of one ruling are collinear and no triangle takes them. That rule lives on the chart lane, and the face never went there: `lane_of` routed by carrier kinds and rings, and a split ruling is two iso carriers. The rectangle lane's `bare_sides` was its own admission test, but it was read after routing and answered by laying a lattice rather than by refusing.

## What moved

Digests of every patch: the whole `mesh` suite, `step-import`'s suite, and `sweep`'s 63 meshing modules, on the base (`c94b8144b5`, PR 7043 before its fix pass) and on this change over that same base, so the comparison isolates this diff. The table below was measured again on the final head, after merging 7043's fix pass, and its counts are the same. Each patch is keyed by test, body (δ and vertex points) and face. **Every moved patch is a face of the class, or a face sharing an edge with one in the same body** (a planar cap or a curved neighbour whose shared chords follow the class face's new chord count). Nothing else moved.

| suite | class faces moved | neighbours moved | other |
|---|---|---|---|
| mesh | 69 (53 cylinder, 16 cone) | 92 (76 plane, 8 cylinder, 8 cone) | 0 |
| sweep (meshing modules) | 30 cylinder, all in `a_box_along_a_fillets_line_arc_junction::a_box_along_an_inflection_ruling` | 60 plane | 0 |
| step-import | 0 | 0 | 0 |

`step-import`'s `split_iso_side` does not move: the import's join merges the two collinear `EDGE_CURVE`s into one edge before the body ships, so no split side reaches the mesh through that door.

The price is known and is the next unit's to remove: the chart lane lays rows these faces do not need, and sizes at `trimmed::UNPROVED_HEADROOM`.

| face | base | head |
|---|---|---|
| T1 cone face, coarse cells (meshed on base) | 81 / 340 / 1581 | 301 / 1215 / 6357 |
| sweep inflection-ruling fillet (cylinder) | 9–10 | 131–132 |
| `curved::tests` split frustum wedge, δ 0.1 (`a_split_then_placed_swept_face_is_not_refused`) | 28 | 40 |
| frustum, one ruling split, δ 0.1 / 0.01 / 0.001 (whole body) | 38 / 222 / 1842 | 74 / 454 / 4062 |
| full cylinder wall, split seam, δ 0.1 (whole body) | 74 | 108–130 |
| half cylinder wall, split ruling, δ 0.1 (whole body) | 42 | 58–72 |
| cone 45° / 80°, one split, δ 0.1 (whole body) | 28–32 / 48 | 68 / 96 |

Pins re-pinned:
- `reach_probes::a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta` becomes `…_meshes_at_every_delta`. All 18 cells mesh and pass `check_mesh` and `reach_probes::geometry` at the default ε, 10⁻⁶ and 10⁻¹²; the join has read every triangle's winding.
- `curved::tests::a_multiply_carried_iso_side_…`'s totals go from 246 meshed and 4 refused to 250 and 0. The four were the mirror-nappe diamond's split cylinder wall.
- `curved::tests::a_split_then_placed_swept_face_is_not_refused` goes from 28 to 40 triangles.
- `curved::tests::a_cone_patch_takes_no_rows_at_any_column_count` loses its fallback half.
