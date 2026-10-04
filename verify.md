# Verify 2, PR 3985 (`reach/arc-from-pairing`): the main merge and the walk filter's removal

Head `b171d906a4`. It merges current main `c860806e8`, so main's tip is also the merge base. The previous verifier checked `f69ec37c57`, whose main was `a93cbcf5e`.

**Verdict: VERIFIED.** No blocking point. There is one non-blocking precision note on check 1 (N1).

## Method

- Worktrees for the head and for `origin/main` `c860806e8`, each with its own target dir, on a 4-core box.
- **Instrumentation** (local, never pushed). On both trees, `chord_join::chord_spec` is renamed `chord_spec_inner` and wrapped by a function with the same signature. When `VERIFY_CHORD_DUMP` is set, the wrapper appends one line per minted spec (`Ok(Some(_))`): the test thread's name, both end points, `carrier`, `param_start` and `param_end`, all `{:?}`-printed f64, so they round-trip exactly. With the variable unset it changes nothing. It is the same probe the first verifier used.
- **Touched-row set:** 224 rows. It covers:
  - every test in the 13 touched sweep test files;
  - JOIN's `pocket_ring_steep_ellipse` and `pocket_wall_crossing_a_side_face`;
  - `editor-core` `refusal_concision_chains`, mesh `r2_bool_door`, step-import `poleguard` and topo `m6_2_fitted_at_rest`;
  - topo's unit tests under `chord_join`, `boolean::{join, reduce, solid_contain, offer_rows}` and `splitting::{join, classify, rules}`.

## 1. The removal is clean. Holds.

I compared the PR's delta against its main before and after the merge, file by file: `diff(a93cbcf5e, f69ec37c57)` against `diff(c860806e8, b171d906)` over `crates/`. Only four files' deltas changed. Everything else the PR carries is line-for-line what was verified at `f69ec37c57` (`all.rs` only re-orders under rustfmt).

- **`boolean/join.rs`.** The walk filter is gone whole: the `if let Some((center, _)) = frame` loop over `open` and `fn walk_passes`, with its two decide sites `bool_join_walk_site` and `bool_join_walk_order`.
  - `git grep 'walk_passes|bool_join_walk'` on the head finds no hits.
  - The only other delta change is the merge resolution. The PR now also deletes main's `wall_region` and its two call sites, which fed `face_azimuth_window` in the `PlaneWall` and `WallPlane` arms.
  - **`join.rs` is main's except for the datum. Confirmed.** `git diff c860806e8 b171d906 -- join.rs` contains:
    - the `Leave`/`Datum` import;
    - the `leave: Leave<T>` argument threaded through `curve`, `split_curve` and `bool_planar_curve`;
    - `bool_planar_curve` losing its `window`;
    - `germ_at` / `leave_a` / `leave_b` with the `JoinDesync` guard ("a segment's curve halves are not its matched germs' own");
    - `wall_region` and the window lookups removed;
    - the self-check row's `segment_curve` call given a `Leave`;
    - comment re-wordings of the retired window.

    Nothing else differs.
- **`boolean/offer_rows.rs`.** The only change is that the `("join.rs", "walk_passes", "Coincide::Join", 1)` row is gone. The audit doc's row went in `4bdc8f67c`, outside `crates/`.
- **`chord_join.rs`.** The only change is that `ChordJoiner::fragments()` is deleted. The other hunks are main's `from_driver` and `lineage` changes, and one hunk whose diff merely re-aligned (`wall_section` call, unchanged text). `lineage` stays, because `rest.rs:977` still uses it.
- **`four_crossings_on_one_section_circle.rs`.** Doc and assertion edits in `c9a91f58c`, `4bdc8f67c` and `1d331e3b9`: the row now holds the arc rather than the walk filter. It is test-only.

**N1, non-blocking.** The brief asks me to confirm that nothing calls `wall_region` / `ChordJoiner::fragments` on main. Strictly, main does call them: `origin/main:crates/topo/src/boolean/join.rs:761` and `:790`, which are `wall_region(.., sb.joiner.fragments(), ..)` in the `PlaneWall` and `WallPlane` arms.

- Both calls exist only to pick the region whose azimuth window `face_azimuth_window` reads. That window is exactly what this PR retires in favour of the datum.
- So the two functions are dead on the head, not on main. They have no other caller on main: `fragments()` is used only by those two lines, and `wall_region` only there.
- Deleting them is a consequence of the PR's window removal, not dead code on main. That is how the lane's "dead after the merge" should be read. It is not a behaviour change beyond the window retirement already verified.

## 2. The chord differential. Holds.

The run covered topo, sweep, mesh, editor-core and step-import with `--all-features` at ε 1e-9 (full default profile, slow set included), plus the demo tour (`cargo test --release`).

| | main `c860806e8` | head `b171d906` |
|---|---|---|
| suite tests run / passed | 7739 / 7735 | 7736 / 7732 |
| chord-minting tests (thread names) | 485 | 497 |
| chords minted | 26,499 | 27,886 |
| tour tests minting / chords | 29 / 10,830 | 30 / 10,854 |
| tour result | all green | all green |

- **Chords main mints that the head does not: 0.**
  - Every distinct spec in main's dump is minted somewhere on the head.
  - For every test name present on both trees, main's chord multiset is contained in the head's.
  - In the tour, all 29 of main's tests are identical.
- **Identical:** 473 tests.
- **Differ only by added chords (7):**
  - `join1_r1_rows::a_pole_struts_halves_face_their_own_meridians` +45
  - `conic_edge_curved_face::a_ball_through_the_cut_face_clears_the_rim_and_stops_downstream` +12
  - `review_d2_adv_probes::d2_no_input_reaches_a_panic` +12
  - `review_d2_adv_probes::d2_reached_variants` +12
  - `run_walls_built::revolved_runs_build_one_wall_each` +8
  - `r2_bool_door::r2_bool_door_near_pole` +6
  - `review_ring_clearance_r1_probes::r1_diag_cylinder_pierces` +4
- **Renamed or retired (5 main names absent on the head).** Each one's specs are all minted under its successor.

  | main name | head successor |
  |---|---|
  | `snowman::a_bar_through_a_ball_crosses_the_sphere` (8 distinct specs) | `snowman::a_bar_through_a_ball_builds_under_every_boolean` |
  | `tilted_sphere_pair::a_tilted_section_stops_at_the_pierce_ring_and_the_planar_side` (1) | `tilted_sphere_pair::a_pierce_off_the_seam_plane_builds_under_every_boolean` |
  | `tilted_sphere_pair::a_tilted_split_of_a_sphere_body_refuses_before_either_arc_rule` (4) | `tilted_sphere_pair::a_tilted_split_of_a_sphere_body_refuses_at_the_reduce` (also minted by two other tilted rows) |
  | `chord_join::tests::arc_side_definite_ccw_and_cw` (2) | `chord_join::tests::the_datum_orients_the_chord` |
  | `chord_join::tests::window_rule_is_seam_placement_independent` (1) | `chord_join::tests::the_datum_orients_the_chord` |

- **New chord-minting names on the head (17)** are the new or renamed rows. Among them are the two four-crossing rows, the two wedge rows, the snowman and tilted successors, and probes that refused on main and now build. Examples of the last group are `m5_s13_review_probes::probe_belly_pierce_…` and `review_pr12_probes::probe_i_door_a_full_tool`.
  - The tour adds `snowman::tests::the_head_moved_out_of_the_seam_plane_builds_through_its_ring` (24 chords).
- **Caveat.** Specs minted on unnamed worker threads are pooled under one `?` bucket: 8,232 on the head. That bucket is multiset-identical on the two trees, so it hides no loss, but it is not attributed per test.
- **Reds in this run, identical on both trees:** four torn-body rows under `per-op-postcondition`.
  - `euler::tests::a_fan_split_past_a_paired_tear_leaves_no_minted_key_in_an_orbit_error`
  - `row_walk_proofs::{a_diversion_paired_with_a_parent_loop_tear_passes_the_proof, row_walks_on_seeds_that_divert_the_sheet, row_walks_on_a_few_torn_bodies}`

  They are main's, as the lane reports.

## 3. Mutants. Every one is killed.

Each mutant was applied alone over the 224 touched rows at ε 1e-9, then rebuilt, run and reverted. The control had 0 of 224 rows red.

| mutant | smallest edit (head) | rows red | four-crossing build row | JOIN pocket build rows (both) |
|---|---|---|---|---|
| datum negated | `arc_leaving`: `along = -(leave.dir·tangent)/…` | 43 | red | red |
| always ccw | `arc_leaving`: `Sign::Negative => Ok(true)` | 44 | red | red |
| germ dir flipped | `germ_at`: `(g.he, -g.dir)` | 31 | red | red |
| partner dir | `germ_at`: each half paired with the other site's `dir` | 11 | red | red |

- The four-crossing row is `four_crossings_on_one_section_circle::a_slab_crossing_one_section_circle_four_times_builds_under_every_boolean`. It is red under all four, as the brief requires.
- The JOIN pocket rows are `pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed` and `pocket_wall_crossing_a_side_face::a_pocket_whose_wall_crosses_a_side_face_builds_at_its_closed_form_volume`. Both are red under all four.
- Under partner dir, the reds are:
  - the four-crossing row;
  - both pocket rows;
  - `join1_r1_rows::a_pole_struts_halves_face_their_own_meridians`;
  - `germ_coplanar_conic::every_op_refuses_or_answers_its_closed_form`;
  - `snowman::{a_bar_through_a_ball…, a_spun_snowman…}`;
  - `tilted_sphere_pair::a_pierce_off_the_seam_plane…`;
  - `verbs_ga_r2_probes::r2_an_off_centre_bar_unions…`;
  - both wedge rows.
- For reference, the first verifier counted 43, 44, 31 and 11 at `f69ec37c57` on its 218 rows, without the pocket files. Here the two pocket build rows account for 2 of each count. The row sets differ, so the counts are not compared row by row.

## 4. Battery. Holds: every red also fails on main.

The head's full suites ran for topo, sweep, mesh, editor-core and step-import with default features and the default profile (slow set included). geom-core's delta is one comment, so I left it out.

| ε | head | red | on main `c860806e8` |
|---|---|---|---|
| 1e-9 | 7690/7690 | — | — |
| 1e-6 | 7689/7690 | `pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed` | **red**, with the same "11 runs miss" and the identical 11 miss lines (byte-equal) |
| 1e-12 | 7689/7690 | `nurbs_import::arc_loft_natively_computes_its_rational_volume` | **red**, with the same panic at `nurbs_import.rs:345` and the identical message |
| 1e-9, `--all-features` (from check 2) | 7732/7736 | the four torn-body rows above | **red**, the same four |

This matches the lane's report exactly: four torn-body rows under `per-op-postcondition`, `steep_ellipse_poses` at 1e-6 and `arc_loft` at 1e-12. All of them are main's.

## Blocking points

None.
