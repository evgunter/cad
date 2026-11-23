STAGE 2 REPORT, part 1 of 2 — tesslane/certificate-decides, head 42d61a3806 (confirmed with ls-remote). Whole bar the folds. No PR opened.

WHAT CHANGED, by commit (this branch's own, oldest first after stage 1)
- d5e18bc7ad: `trimmed.rs` UNPROVED_HEADROOM and face_target deleted, lattice and arm steps at δ; `chords.rs` edge_target and adjacent_faces deleted, every rim chorded at δ; the retry loop on two counters (fold + shared side against MAX_GRID_RETRIES, correction against new MAX_CORRECTION_ROUNDS = 6); `TrimmedFace::correction_point` (side-sag) on every row, spline included; `curved.rs` the same correction around its triangulation; `cd_probe.rs` and every hook removed.
- 2b70ea5cd5: chart-lane row `an_over_delta_first_guess_is_corrected_and_refuses_typed_without_the_correction`; test seams CORRECTION_ROUNDS / CORRECTIONS_USED; the clearance row moves to δ 0.02.
- 9a210ca359, 1bc8e467a9, e6093bb2f4, e09cbbacbb: re-pins (each stated with its old value in the message) and the rectangle-lane row `an_over_delta_grid_is_corrected_and_refuses_typed_without_the_correction` (sphere band, grid steps doubled, STEP_SCALE seam in `grid_counts`).
- 5fa05642ba: prose: trimmed's module docs gain "The correction"; sizing, chart_table, types, nurbs_cert, split_rulings, d9 doc lose the headroom; README other-constants list gains MAX_CORRECTION_ROUNDS (the only README edit).
- 216e87f9ae: test seams moved inside trimmed's tests module (the ε inventory cuts a file at its first top-level `#[cfg(test)]`).
- cf6c4e30f7: work files per your rulings (unit file's stage-1 section; three rows closed not owed with both caveats; clearance row opened; P3 cheap-round unit and P4 torus row filed; spur cap row gains the four measurements).
- 0133e9ec74: merge of main. PR 7498 rewrote the rectangle lane's triangulation (CDT in the lattice metric); I took main's `curved.rs` and re-applied the correction onto it (an affine metric keeps a side's chart middle its middle). `chart_table` takes main's cell_metric/trimmed_metric split, keeping unit 4b's rowless cylinder. PR 7517's two new face_target readers are spline rows and read δ.
- 42d61a3806: viewer re-pins (below).

RUN LOCALLY (dev profile unless said; chain on a separate worktree)
- `mesh` whole suite on the merged head at ε 1e-9, 1e-6, 1e-12: 563/564 each; the red is `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (the fold).
- `sweep` 131 meshing modules (934 tests) at 1e-9 on cf6c4e (pre-merge): reds `tilted_sphere_pair::a_circle_near_the_pole…` (fold) and main's listed `bisector_nodes::the_near_node_poses_at_the_witness_band`. `sweep`'s 50 chart-lane tests on the merged head at 1e-6 and 1e-12: the fold, and at 1e-12 main's listed `a_cylinder_wall_through_a_prism_corner…`.
- `step-import` (cf6c4e): only main's three listed nist_ftc_09 reds.
- `editor-core` (cf6c4e): 3566/3594; 28 reds in 15 files (corpus_finish_count, docm9_range, revalue_corpus ×10, m10_* interval rows ×10, seat4/7/8 lowering, split_pinch_keeps, sym11_exact_channel_rows, sym_9_retry_interval ×2). None is this unit's: editor-core reads the mesher only in `resolve/pick`, and no failing file tessellates or picks (grep; the only hits are the word "picks" in prose). Not re-run on main (a full rebuild of another tree). Report them onward as you see fit.
- the tour (`ci-tour` cargo profile, merged head): 107/108; the red `wall_jets::the_klein_loops_walls_hold_against_themselves` panics in `sweep`'s placement at Interval and never meshes.
- the viewer's slow rows on the merged head: both moved with this unit and are re-pinned (42d61a3806, verified passing): `display_budget` only `cut_cylinder` moved; `review_pick_r2` tally (424,728, 138,718, 11,004, 5,130) → (422,568, 137,416, 10,236, 4,818). Caveat: the old value is this branch's pre-unit pin (unit 4b's), which conflicted with main's own re-pin of the same row (main 451,512/147,844/…); I did not build a main+4b-without-this-unit tree for a clean base. If the two other open PRs re-pin these first, they move again at the next merge.
- mutants: correction off (caught by both lane rows, `m5_pr11_trimmed::a_wide_tilted_cylinder…`, `rowless_cylinders::a_notched_or_holed_wall…`); budget shared again (caught by the chart-lane row's own-budget assertion); centroid (caught by `sweep` `snowman::two_cut_ins…`, refuses at δ 1e-3, 1.05 δ); longest-side midpoint (caught by `sweep` `bisector_nodes::slanted_bores…`, refuses at δ 2e-3, 1.34 δ). The rule mutants are caught by existing rows, so no new row pins rounds per face.
- clippy -D warnings and fmt on mesh; viewer fmt; work.py lint ok.
LEFT TO CI: everything else, release profile, the nightly ε rows and render lanes.

NOT DONE / OPEN
- The folds block landing, per your ruling 5. I have not merged `tesslane/join-split`; tell me when its fold-cure commit exists and I will merge it and re-measure the five meshings.
- Unit 4b's fix pass (the two-hole row) is not on its branch yet (its head is still 553d4d6310, merged here); merge pending.
- Commit e6093bb2f4 pushed a needle-funnel pin that failed at the default ε; e09cbbacbb fixed it. History, noted.

SLOWED ME DOWN: dev-profile `sweep` and `editor-core` runs (43 and 83 minutes; several boolean rows take minutes each); a merge with main mid-run that rewrote the code my rectangle correction wraps; I once ran a stray `git checkout <old sha> -- crates/mesh/src` that overwrote the working tree with the probe-era source; recovered from a stash I had just taken, nothing lost or pushed.
