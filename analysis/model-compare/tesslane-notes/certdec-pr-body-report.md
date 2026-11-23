certificate-decides: the PR body for head 1cfa09180c (confirmed with ls-remote). The list is running detached on that head; I push nothing until you say, and send its result separately. Two notes outside the body: the four mutants were run before the last three merges (on the rowless cylinder's earlier head), and the first-pass `mesh` count is 562 of 563 because the suite gained and lost rows across the merges.

=====
## The defect and the cure

The chart lane sized every analytic face (cylinder, cone, sphere, torus, spindle) and chorded every rim such a face shares at `UNPROVED_HEADROOM` = ½ of δ, because a triangle the constrained triangulation makes on a trim chord point can reach past one lattice cell and nothing bounds by how much. The factor bounded nothing either: faces refused on the certificate at the headroom, and it cost about twice the triangles on every such face and every rim beside one.

`crates/mesh/README.md` `[mesh.sizing]` now reads: the step is a first guess, the coarsest the chart's cell bound allows at δ; the certificate decides. This PR makes the code say that.

- **Every chart-lane face sizes at δ.** `UNPROVED_HEADROOM`, `trimmed::face_target` and the chord pass's per-edge target go; the lattice, the arm steps and every rim chord read δ.
- **The correction, on every row that triangulates.** A triangle whose certificate reads over δ (`cert::Verdict::Exceeded`) adds one point, and the face is triangulated again. The point is the middle of the triangle's side, among those that are no constraint, whose own chord stands farthest from the surface (`TrimmedFace::correction_point`, `curved::correction_point`); a triangle of three constraint sides takes its inside point. On a ruled chart the longest side can lie along a ruling, where a point moves no certificate, so the side is chosen by its sag, one surface evaluation a side. It runs on analytic rows, spline rows (which corrected at the centroid before) and the rectangle lane's triangulation (which did not correct).
- **Its own budget.** `trimmed::MAX_CORRECTION_ROUNDS` = 6, a termination constant beside `MAX_GRID_RETRIES`, which now counts only fold drops and shared-side splits. Past it the face refuses `CertificateExceeded`. It is a budget, not a proof; the most any face took in the suites below is 3.
- **A side's middle is read through `geom::mid_param`**, the repo's one spelling of a bracket's middle (the `bracket-mid-by-hand` gate).
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

80 faces (2.4%) take a round; 53 of them cost more time than their old mesh and end coarser (median 1.19 times, worst 2.6). A first guess of 0.9 δ does not remove them (65 faces still take a round, mostly others), so no family is sized finer. The owner's reading of the clause on these faces (PR 7486, 2026-11-23): "this still looks good to merge--the 'do extra work' is more centrally about the work of writing the code rather than of running it". Filed, as running-time improvements the clause does not owe: `a-correction-round-triangulates-and-certifies-the-whole-face-again` (P3) and `the-torus-arm-at-a-finer-first-guess-is-measured-on-two-bodies` (P4).

## What moves, and why each new mesh is right

Every chart-lane analytic face and every face sharing a rim with one is meshed coarser, by the change of target from δ/2 to δ, with each triangle still certified within δ. Neighbours whose triangle count moved (`mesh` / `sweep`, ε `10⁻⁹`, count moved over faces, ratio over the moved):

| lane, chart | `mesh` | `sweep` |
|---|---|---|
| planar | 1,628 / 36,009, 0.72 | 1,378 / 22,923, 0.73 |
| rectangle, cylinder | 26 / 8,062, 0.75 | 34 / 3,130, 0.74 |
| rectangle, sphere | 26 / 3,101, 0.99 | 87 / 2,339, 0.76 |
| rectangle cone, torus, spindle; walked | none | none |

No rectangle-lane, planar or walked face newly refuses in any suite at any ε, and none takes a round.

Pins moved, each shown by a base run (the old value passes without this unit, the new value on this head):

- `d9_mesh_goldens`: `tilted_above`, `tilted_below`, `notched_wall` and `holed_wall` at δ 0.1 and 0.01 (the corpus's chart-lane cylinder walls), the same digests at ε `10⁻⁹`, `10⁻⁶`, `10⁻¹²` and at one and four threads. No other corpus body moved.
- `route_rows`: the sphere faces under a tilted slab (eight digests) and the pipe cut on a tilt.
- `rowless_cylinders`: every cylinder face still holds Euler's count `n − 2 + 2h + 2p`. `a_cut_wall_is_its_boundary_points_alone` held `p = 0` on every wall at δ/2; at δ six cells take the correction and are pinned by wall and δ (two notches at 10⁻¹: 2 points; the comb of six: 10; the bars of half-width 0.6 and 0.9: 10 and 8; the notch beside the seam and two notches at 10⁻³: 1 each), every other cell `p = 0`. `two_holes_across_a_column_and_a_half_mesh_by_the_correction`: at δ/2 its eight cells 1.5–1.6 columns apart took 4 points in one round under a centroid; at δ those cells mesh at the first attempt, and the cells that refuse with the correction off are the ones 2.5 columns apart and the first `r = 10` cell, pinned at 4, 6, 5, 4 and 4 points, one round each (a centroid inserts 8 on the same cells, which the count would show). The notch-across-the-seam digests move.
- `winding_rows::the_cut_frustum…`: thin triangles 226 → 160, least height 5.6·10⁻¹⁰ → 7.9·10⁻¹⁰ m.
- `reach_probes::crossing_rods…` holds only Euler's count now and passes unchanged; its prose's measured cost is restated at δ (worst `n = 22` at δ 0.5, 76 triangles).
- `reach_probes::a_needle_funnel…`: the half above refuses at every δ either way; at δ 2·10⁻⁴ by `FoldedTriangle` (was `UnclearedSharedSide`), at 3·10⁻⁵ by `UnclearedSharedSide` (was `FoldedTriangle`).
- `trimmed::tests::a_lattice_laid_without_clearance…` runs at δ 0.02, whose lattice is the one δ 0.04 laid at the headroom.
- `viewer` `display_budget`: `cut_cylinder`'s largest probe 656 → 456 at δ 10⁻⁴ and 2,912 → 2,056 at 5·10⁻⁶ (triangles 712 → 512, 3,624 → 2,584); no other document's answer moved.
- `viewer` `review_pick_r2`: the corpus tally (rays, grazes, refused candidates, rays with a refusal) (424,728, 138,712, 11,004, 5,130) → (422,568, 137,416, 10,236, 4,818); `genuine_refused` stays empty.

Base runs for the rows the rowless cylinder (PR 7526) pinned: on its head b448200999 its own pins pass (`rowless_cylinders`, `d9_mesh_goldens`, `crossing_rods…`, and the viewer's two rows at (424,728, 138,712, 11,004, 5,130) and `cut_cylinder` 656 / 2,912); on this head the values above.

The neighbours' table and the work table were measured before the rowless cylinder's final head and main were last merged; the pins above are read on this head.

## The rows that pin it

- `trimmed::tests::an_over_delta_first_guess_is_corrected_and_refuses_typed_without_the_correction` (the tilted pair of balls, sphere arm steps doubled): corrected within 1..=6 rounds and the mesh checks and encloses the body; with no round it refuses `CertificateExceeded`; with the fold and shared-side budget at nothing it still corrects (its own budget); at the arm's own steps no round runs and the mesh holds under 0.6 of the triangles of the mesh at δ/2.
- `curved::tests::an_over_delta_grid_is_corrected_and_refuses_typed_without_the_correction` (a sphere band, grid steps doubled, δ 10⁻² and 10⁻³): the same three relations on the rectangle lane.
- `loops_the_meridian_guard_admits::a_spur_on_a_rim_only_cap…` keeps refusing at both δ and its pin stays; its doc gains the four measurements (the shipped rule needs more than 40 rounds at δ 0.1 and 8 at 0.01).

## Mutants

| mutant | caught by |
|---|---|
| the correction off (both lanes) | both rows above, `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, `rowless_cylinders::two_holes_across_a_column_and_a_half_mesh_by_the_correction` |
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

- Five meshings that mesh at δ/2 fold at δ, each a tilted circle near a sphere's pole: `sweep` `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta` (the second body at δ 4·10⁻², its four sphere faces) and `mesh` `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (the near-pole tube on the ball, its ringed sphere face at δ 5·10⁻³). No triangle reads over δ there (0.934 δ and 0.956 δ), so the correction does not reach them. Their cure is the fold's own, PR 7573's, which is NOT merged here: both rows are red on this head and stay red until it lands under this.
- Every refusal that refused before, by the same cause, but for the needle funnel's two cells above.

## Open, owed at the integration with PR 7573

Measured on a scratch tree: PR 7573's fold cure (commit 4922edde4a) with this unit ported onto it by hand, without the rowless cylinder under it, dev profile, ε `10⁻⁹`. All five folded meshings mesh there (the near-pole ringed face in 5 fold rounds at δ 2·10⁻², 2 at 5·10⁻³, and one correction round at 10⁻³; the tilted pair's four faces at δ 4·10⁻² in 2 fold-or-shared-side rounds each). Three things from that tree are open:

1. `sweep` `bisector_nodes::an_oblique_seam_near_a_node_builds` and `bisector_nodes::equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms` fail their mesh-volume oracle there: the mesh, returned, certified and watertight at δ 2·10⁻³, encloses 0.76449 against 0.76980, 0.69% where the row allows half a percent. Both pass on this branch. On the integrated tree either they pass, or the inscribed-polygon deficit at that δ and radius is measured and the row's number re-derived from it.
2. The correction rate there is 51% of faces (1,051 of 2,047 take one round, 12 take two) against about 2% on this branch. It is re-measured on the integrated tree; if it does not fall back, the work table above does not hold.
3. Six sphere faces of `sweep` `spheres_crossing_off_every_edge::a_trimmed_face_cut_beside_its_pole_builds` refuse `FoldedTriangle` at δ 5·10⁻³ after six fold rounds, as they do on main.

The crossing rods' cell (`reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make`, said to read over δ once the shared-side insert is retired at b197d12c7e) is checked at that integration, by name.

## What ran, on which tree

The branch holds main at a497926d13 (PR 7526, the rowless cylinder, is in it) and `tesslane/ev-sizing` at 10397beb6e. The fold cure (PR 7573) is another unit's and is NOT merged in.

On 73a4fdd625 (this unit on the rowless cylinder's final head and the main before a497926d13), dev profile, all of it complete:

- green: `cargo fmt --all --check` and the benches'; `work.py lint`; `scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings`; the same on `viewer`; `cargo clippy -p mesh --all-targets --features budget -- -D warnings`; the viewer's `display_budget::the_budget_commits_the_delta_it_always_has` and `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus`.
- `mesh` whole suite at ε `10⁻⁹`, `10⁻⁶`, `10⁻¹²`: 562 of 563 each. The one red, each time: `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (the near-pole fold at δ 5·10⁻³).
- `cargo test -p mesh --features budget --no-fail-fast`: the lib 253 of 253, `tests/all.rs` 316 of 317, the same one red.
- `sweep`'s 131 meshing modules, 934 tests, ε `10⁻⁹`: 932 pass. Red: `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta` (the fold) and `bisector_nodes::the_near_node_poses_at_the_witness_band` (main's, red there before this unit).

On 1cfa09180c (the same with main at a497926d13 merged; 80 files under `crates/` moved, mostly `geom-brep`): `cargo check -p mesh --tests` passes; the whole list above is running again and is NOT yet in.

Earlier, before the last three merges: `step-import` (red only on main's three `nist_ftc_09` rows), the tour (red only on `wall_jets::the_klein_loops_walls_hold_against_themselves`, which fails on main's own tree with the same panic), `editor-core` (28 reds, 24 on main's tracked rows and the other four red on main's own tree with the same messages), `sweep`'s chart-lane tests at ε `10⁻⁶` and `10⁻¹²` (the fold; at `10⁻¹²` main's `a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`).

## Closed and filed

- `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`, `the-trimmed-sphere-arm-has-no-worst-triangle-theorem` and `one-containment-lemma-for-an-oblique-boundary` close as not owed: the bound each asked for was owed only so the step alone could hold the triangle beside a trim chord, and the certificate holds it now. Said where they close: the correction's termination is a budget, not a proof; `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` is a walked face and is not touched.
- `the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound` opens as a measurement.
- Filed: the two rows above (P3, P4); the near-pole fold's evidence at δ on `a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm`.
- `crates/mesh/README.md`'s other constants gain `MAX_CORRECTION_ROUNDS`; the `[mesh.sizing]` clause is the owner's text from PR 7486.
=====
