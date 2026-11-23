STAGE 2 REPORT, part 2 of 2 — the PR body, ready to paste (its "What still refuses" section changes once the fold cure is merged and the five meshings mesh).

=====
## The defect and the cure

The chart lane sized every analytic face (cylinder, cone, sphere, torus, spindle) and chorded every rim such a face shares at `UNPROVED_HEADROOM` = ½ of δ, because a triangle the constrained triangulation makes on a trim chord point can reach past one lattice cell and nothing bounds by how much. The factor bounded nothing either: faces refused on the certificate at the headroom, and it cost about twice the triangles on every such face and every rim beside one.

`crates/mesh/README.md` `[mesh.sizing]` now reads: the step is a first guess, the coarsest the chart's cell bound allows at δ; the certificate decides. This PR makes the code say that.

- **Every chart-lane face sizes at δ.** `UNPROVED_HEADROOM`, `trimmed::face_target` and the chord pass's per-edge target go; the lattice, the arm steps and every rim chord read δ.
- **The correction, on every row that triangulates.** A triangle whose certificate reads over δ (`cert::Verdict::Exceeded`) adds one point, and the face is triangulated again. The point is the middle of the triangle's side, among those that are no constraint, whose own chord stands farthest from the surface (`TrimmedFace::correction_point`, `curved::correction_point`); a triangle of three constraint sides takes its inside point. On a ruled chart the longest side can lie along a ruling, where a point moves no certificate, so the side is chosen by its sag, one surface evaluation a side. It runs on analytic rows, spline rows (which corrected at the centroid before) and the rectangle lane's triangulation (which did not correct).
- **Its own budget.** `trimmed::MAX_CORRECTION_ROUNDS` = 6, a termination constant beside `MAX_GRID_RETRIES`, which now counts only fold drops and shared-side splits. Past it the face refuses `CertificateExceeded`. It is a budget, not a proof; the most any face took in the suites below is 3.
- **No ε is read.** The correction reads the certificate's verdict and nothing else; the mesh stays a function of (body, δ) (`[tolerance.chordal]`, `[mesh.winding]`), checked by `d9_mesh_goldens` at three ε.

## What was measured before it was built

Over the `mesh` suite with its slow set, `sweep`'s 131 meshing modules and `step-import`, at ε `10⁻⁹`, `10⁻⁶` and `10⁻¹²` (the unit file's "Stage 1" section has the tables):

| point a triangle read over δ adds | faces lost on the certificate | most rounds (`mesh` / `sweep` / `step-import`) |
|---|---|---|
| centroid | 0 | 3 / 7 / 0 |
| middle of the longest free side | 0 | 3 / 7 / 0 |
| middle of the free side whose chord sags most (shipped) | 0 | 3 / 2 / 0 |

Work, release, one thread, nothing else running, two repeats, time in the face against the old δ/2 sizing:

| chart | faces | time | triangles |
|---|---|---|---|
| cone | 452 | 0.52 | 0.50 |
| cylinder | 1,819 | 0.74 | 0.71 |
| sphere | 858 | 0.56 | 0.53 |
| torus | 109 | 0.69 | 0.51 |

80 faces (2.4%) take a round; 53 of them cost more time than their old mesh and end coarser (median 1.19 times, worst 2.6). A first guess of 0.9 δ does not remove them (65 faces still take a round, mostly others), so no family is sized finer. Filed: `a-correction-round-triangulates-and-certifies-the-whole-face-again` (P3) and `the-torus-arm-at-a-finer-first-guess-is-measured-on-two-bodies` (P4).

## What moves, and why each new mesh is right

Every chart-lane analytic face and every face sharing a rim with one is meshed coarser, by the change of target from δ/2 to δ, with each triangle still certified within δ. Neighbours whose triangle count moved (`mesh` / `sweep`, ε `10⁻⁹`, count moved over faces, ratio over the moved):

| lane, chart | `mesh` | `sweep` |
|---|---|---|
| planar | 1,628 / 36,009, 0.72 | 1,378 / 22,923, 0.73 |
| rectangle, cylinder | 26 / 8,062, 0.75 | 34 / 3,130, 0.74 |
| rectangle, sphere | 26 / 3,101, 0.99 | 87 / 2,339, 0.76 |
| rectangle cone, torus, spindle; walked | none | none |

No rectangle-lane, planar or walked face newly refuses in any suite at any ε, and none takes a round.

Pins moved, each shown by a base run (the old value passes with the headroom at a half, the new value on this head):

- `d9_mesh_goldens`: `tilted_above` and `tilted_below` at δ 0.1 and 0.01 (the corpus's two chart-lane cylinder walls), the same digests at ε `10⁻⁹`, `10⁻⁶`, `10⁻¹²` and at one and four threads. No other corpus body moved.
- `route_rows`: the sphere faces under a tilted slab (eight digests) and the pipe cut on a tilt.
- `rowless_cylinders`: the notch-across-the-seam digests; the notched-wall relation measures against the plain wall at δ and counts two triangles per added point, its shares 1.1 → 1.15 and 1.4 → 1.5 (measured 1.014–1.146 and 1.278–1.500; a notch's own boundary points are a larger share of the coarser strips).
- `winding_rows::the_cut_frustum…`: thin triangles 226 → 160, least height 5.6·10⁻¹⁰ → 7.9·10⁻¹⁰ m.
- `reach_probes::crossing_rods…`: the shared-side cure's cost 3.25n → 3.5n (worst n = 22 at δ 0.5, 76 triangles).
- `reach_probes::a_needle_funnel…`: the half above refuses at every δ either way; at δ 2·10⁻⁴ by `FoldedTriangle` (was `UnclearedSharedSide`), at 3·10⁻⁵ by `UnclearedSharedSide` (was `FoldedTriangle`).
- `trimmed::tests::a_lattice_laid_without_clearance…` runs at δ 0.02, whose lattice is the one δ 0.04 laid at the headroom.
- `viewer` `display_budget`: `cut_cylinder`'s largest probe 656 → 456 at δ 10⁻⁴ and 2,912 → 2,056 at 5·10⁻⁶ (triangles 712 → 512, 3,624 → 2,584); no other document's answer moved.
- `viewer` `review_pick_r2`: the corpus tally (rays, grazes, refused candidates, rays with a refusal) (424,728, 138,718, 11,004, 5,130) → (422,568, 137,416, 10,236, 4,818); `genuine_refused` stays empty. The old value is the rowless cylinder's re-pin, taken before main's own re-pin of this row; the new value is measured on this head, which carries both.

The neighbours' table and the work table were measured before main was last merged; the pins above are read on this head.

## The rows that pin it

- `trimmed::tests::an_over_delta_first_guess_is_corrected_and_refuses_typed_without_the_correction` (the tilted pair of balls, sphere arm steps doubled): corrected within 1..=6 rounds and the mesh checks and encloses the body; with no round it refuses `CertificateExceeded`; with the fold and shared-side budget at nothing it still corrects (its own budget); at the arm's own steps no round runs and the mesh holds under 0.6 of the triangles of the mesh at δ/2.
- `curved::tests::an_over_delta_grid_is_corrected_and_refuses_typed_without_the_correction` (a sphere band, grid steps doubled, δ 10⁻² and 10⁻³): the same three relations on the rectangle lane.
- `loops_the_meridian_guard_admits::a_spur_on_a_rim_only_cap…` keeps refusing at both δ and its pin stays; its doc gains the four measurements (the shipped rule needs more than 40 rounds at δ 0.1 and 8 at 0.01).

## Mutants

| mutant | caught by |
|---|---|
| the correction off (both lanes) | both rows above, `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, `rowless_cylinders::a_notched_or_holed_wall…` |
| the correction's rounds counted against `MAX_GRID_RETRIES` | the chart-lane row's own-budget assertion |
| the point at the centroid | `sweep` `snowman::two_cut_ins_on_one_face_build_in_either_order` (refuses at δ 10⁻³, 1.05 δ) |
| the point at the middle of the longest free side | `sweep` `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself` (refuses at δ 2·10⁻³, 1.34 δ) |

## §5 sweep

Shape 1, a lane sizing below δ by a typed share (`delta *`, `* 0.5`, `_SHARE`, "share of δ" over `crates/mesh/src`):

- `trimmed::UNPROVED_HEADROOM` and `face_target`: removed here.
- `nurbs_cert::LATTICE_SHARE` (½): the spline lattice's share, solved with its sliver theorem so the sliver lands at δ; a derived split, with its own open row (`the-nurbs-lattice-share-is-not-chosen-by-measurement`). Not this unit.
- `cert::ALLOWANCE_SHARE`: rounding, not sizing. Not this unit.
- `chart_table`'s `0.5 * cone_chord_sag`: a derived radius below which a cone row moves no bound. Not a factor.

What that pattern cannot see: a share hidden inside a step function, or a lane that triangulates without correcting. Shape 2, every site that reads a certificate verdict (`FaceVerdict::new`, `.refusal(`):

- `trimmed::TrimmedFace::attempt`: corrects (this PR).
- `curved::tessellate_curved`: corrects (this PR).
- `trimmed::walk_face` (the walked lane): no triangulation; every triangle is held at δ by the walk's theorems (`sizing::sagitta_step`, `cone_grid_step`). Not this unit.
- The planar lane reads no certificate: its triangles lie on their plane.

The step functions in `sizing.rs` (`sagitta_step`, `curvature_step`, `ellipse_step`, `spiric_step`, `cone_grid_step`, `sphere_grid_steps`, `torus_grid_steps`) each take the target as given, and every caller now hands them δ.

## What still refuses

- Five meshings that mesh at δ/2 fold at δ, each a tilted circle near a sphere's pole: `sweep` `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta` (the second body at δ 4·10⁻², its four sphere faces) and `mesh` `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (the near-pole tube on the ball, its ringed sphere face at δ 5·10⁻³). No triangle reads over δ there (0.934 δ and 0.956 δ), so the correction does not reach them; their cure is the fold's own, from `tesslane/join-split`, merged before this lands.
- Every refusal that refused before, by the same cause, but for the needle funnel's two cells above.

## Closed and filed

- `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`, `the-trimmed-sphere-arm-has-no-worst-triangle-theorem` and `one-containment-lemma-for-an-oblique-boundary` close as not owed: the bound each asked for was owed only so the step alone could hold the triangle beside a trim chord, and the certificate holds it now. Said where they close: the correction's termination is a budget, not a proof; `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` is a walked face and is not touched.
- `the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound` opens as a measurement.
- Filed: the two rows above (P3, P4); the near-pole fold's evidence at δ on `a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm`.
- `crates/mesh/README.md`'s other constants gain `MAX_CORRECTION_ROUNDS`; the `[mesh.sizing]` clause is the owner's text from PR 7486.
=====
