# BOXFINISH M10 — the cost row's tables re-taken on main after PR 7394, switch's box arm off (2026-11-24)

## What contradicts the row, first (all measured)
1. **The tour no longer refuses at the holes, and it got seven times dearer, not cheaper.** With PR 7384 the holes' `u_ref`
   refusal is gone: the box-arm-off tour certifies 211 leaves, 94.9069%, as the switch-on tour did at M4 (not re-run today). But each hole extrude
   now FINISHES in every leaf (520 ms average, was a 40 ms refusal at check 1), so summed leaf time is 1,304.9 s against M9's
   177.6 s. The time is on the decision path: `curve_face_pose_disc`, 8 decisions per hole finish, about 1.0 to 1.3 s of a
   traced finish, every one indeterminate after a ladder climb (12,000 to 28,000 forms, largest 173 terms).
2. **The two-parameter plate now refuses at check 7's quadrature, not check 1.** `m10_3`'s
   `the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it` and `a_band_parameter_certifies_normally_
   and_prices_nothing` refuse in all 31 replays and take 27 s each (switch on: 3.1 s and 4.9 s, pass):
   `VolumeUncomputable { face 7v1, Escalated { Indeterminate { Enclosure { 0.0, 9.1257 }, band (1e-9, 1e-8),
   "props_meridian_on_surface" }, check: Exact } }`, raised in the volume reading (`topo::props`), 925 ms per finish.
3. **The docm9 rows the row names are gone or renamed, and docm9 has new slow rows.** `a_certified_side_stops_at_the_seeds_
   edge` was replaced in `5c44ed4185` ("range: a resumable search ..."); its two siblings take 0.110 / 0.127 s off (0.039 /
   0.042 s on). The search rows that replaced them take 1.9 to 7.3 s with the box arm off and 0.2 to 2.2 s on (item 4).
4. **`a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width` runs out of
   memory with the box arm off**: killed by the kernel at 245 s in the suite and at 278 s run alone; 0.18 s with the switch on.
5. **PR 7458 removed M8's split waste on the tour**: 0 of 511 splits are on an axis the deciding node does not read (M8: 292),
   and the read-set tree is the drive (1,023 = 1,023; M8: 127 against 1,023). The deciding node is now node 91, the
   assertion, which reads all three axes (M8: node 29, two).

## 2. The row's tables, box arm off (OFF) and the same main with the switch on (ON)
Serial nextest times (`-j 1`) unless said; M4 = OFF-head 2026-11-15; M7 = S1 on the M7 tree (the state PR 7394 ships).
| row | M4 off | M7 (S1) | ON today | OFF today |
|---|---|---|---|---|
| slab `a_macroscopic_box_certifies_at_the_shipped_depth_budget` | 1 leaf, 89 ms, 816/971 | 1 leaf, 816/971, frozen 0 | pass | **pass, 1 leaf 33.8 ms, 792/971, frozen 0** |
| two-parameter plate (row above) | refused, check 1 `u_ref` | (not taken) | 3.10 s pass | **27.1 s FAIL**, 31 replays, 0 certified, check 7 |
| band-parameter plate | refused, check 1 `u_ref` | (not taken) | 4.93 s pass | **27.3 s FAIL**, same refusal |
| `the_certification_width_is_no_longer_bounded_by_epsilon` | settles 0.2152 | | 3.65 s pass | 9.27 s FAIL, settles 0.21516 (rays) |
| m10_8 `m10_8_the_a0_set_is_a0_alone` | 7.46 s | 1.61 s | 1.12 s | **1.72 s** (1.53 s median of 3 `--exact`) |
| m10_8 plain decisions (pin 444/496/0) | 2424/3151/192 | 2424/3151/24 | pass | 2376/3135/24, FAIL |
| m10_3 `driving_writes_nothing` | 0.44 s | 0.26 s | | 0.17 s |
| sweep revolved circle (`a_revolved_circles_latitude_over_a_centre_box_is_exact`) | 3.40 s | 1.33 s | 0.16 s | 2.30 s (1.01 s median of 3 `--exact`; the two runs disagree, unexplained) |
Per leaf, where the time goes (`BOXFINISH_M8=1`, one run each, probe printing subtracted; finish ms from
`BOXFINISH_TRACE=3`, before = building margins, inside = `sign_within`):
| leaf | leaf ms | node ops (finishes) | session teardown | finish breakdown |
|---|---|---|---|---|
| slab, 1 leaf | 38.5 | | 1.4 | 25 ms: pcurve/chart 12.5, rays 5.9, carrier (check 2) 3.1, dihedral 1.9; inside 4 |
| a0 plate, 1 leaf, 3 finishes (was 4) | 1,016 / 420 (two drives) | hole node op 450 ms each, finish 68 to 78 of it | 45 | hole finish 81 ms: carrier 34 (`ssi_hull_sup` margins 32), pcurve 18, quadrature 16, rays 8 |
| two-parameter plate, 31 leaves | 26,400 total (490 to 1,420 per leaf) | 23,600 (19,600) | 2,800 | refusing finish 998 ms: rays 655 (564 inside), `curve_face_pose_disc` 263 |
| revolved circle finish | | | | 838 ms: `curve_pair_span_end_chord` 438 (72 dec.), `bool_*` 294, pcurve 34 (M7: 1,011) |
Decisions per slab finish: 824, of which 732 repeat the build's (same node, M5's test); a0 drive 1,795 sym / 2,158 num.

## 3. The tour (`demo-tour certified`, box arm off)
| | M9 (same probe, older main) | today |
|---|---|---|
| first drive | 1,023 replays, 0 certified, 512 refused | 1,023 replays: 812 bisect, **211 certified**, 301 refused (budget); 94.9069% certified |
| wall | 46 s (traced run) | **328 s** to the first drive's answer; the whole command 710 s, exit 101 |
| summed leaf time | 177.6 s | **1,304.9 s** (1.28 s per leaf; M9 0.17 s) |
| node ops (finishes) | 153.7 s (59.5 s) | 1,173.4 s (1,037.1 s) |
| session teardown | 23.0 s | 128.6 s |
| everything else | 0.9 s | 2.9 s |
| hole finishes | 2,046 refusing, 40 ms | 2,204 building, **520 ms** average, 1,146 s |
| plate finishes | 1,023, 22 ms | 1,102, 16.7 ms, 18.4 s |
- The command's exit: `src/chaintol.rs:297`: "the header says the Interval lane certifies the whole study at 1 link(s); it
  refused at: node 111e1e667832 — ... an edge's stored curve does not certify against its faces: whether it lies where its
  description says is undecided" (measured; whether the switch-on tour exits the same way: not run).
- Decisions: 1,836,285 symbolic of 4,378,201 (41.9%), 1,490 frozen.

## 4. The docm9 rows
| row | ON | OFF |
|---|---|---|
| `a_parameter_field_boxes_directly` | 0.042 s | 0.127 s |
| `the_query_does_not_touch_the_input_document` | 0.039 s | 0.110 s |
| `a_side_with_nothing_to_find_reaches_the_seeds_end` (nearest to the removed seed's-edge row) | 0.037 s | 0.104 s |
| `one_side_does_not_starve_the_other` | 2.15 s | 7.26 s |
| `the_search_finishes_soon_after_its_answer_stops_moving` | 1.30 s | 4.72 s |
| `every_value_inside_the_proof_decides_as_the_witness_did` | 1.48 s | 4.36 s |
| `the_first_boundary_stops_the_proof_and_a_search_on_the_island_proves_it` | 1.12 s | 3.51 s |
| `a_gap_between_two_lines_is_proven_different_below_the_crossing` | 0.59 s | 1.98 s |
| `a_boundary_past_undecided_ground_is_proven_different` | 0.23 s | 1.87 s |
| `the_directed_search_proves_ground_on_the_plate_the_level_drive_cannot` | 22.1 s FAIL | 21.9 s FAIL (main's own) |
No docm9 finish refuses with the box arm off (812 box finishes, 29 ms average); the multiple is finish cost per replay.

## 5. What refuses now that did not at M9, and what stopped (full suite, geom-core + profile + topo + sweep + editor-core,
default eps, box arm off: 14,097 run, 14,046 pass, 51 fail; against M7's S1-on run on the M9 tree, 58 fail; each of
today's 51 then run on the switch-on control of the same main commit)
- **New since M9, red only with the box arm off (4):** `a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_
  rectangle_certify_over_a_boxed_width` (out of memory, above); `m10_sym_profile_interval::the_plates_largest_form_is_under_
  the_growth_guard` ("the cap apex's lines freeze nothing on the plate: [Early/Instrument 470, Door/Instrument 470]");
  `sym11_far_placement_rows::sym11_the_far_placement_is_a_rounded_contradiction_at_sym_f64` (M4's `OutsideSessionBox` at the
  door, renamed row); `topo live::tests::production_reads_an_edges_curve_only_through_the_view` (the probe's own trace reads
  the curve arena raw in `validate`: a probe artefact).
- **New since M9, red on main with the switch on too (21):** the bisected pair (`bisector_nodes`, `section_end_poses`,
  `revalue_boss_union_apart_to_its_parting`); nine more `revalue_corpus` rows ("a decision's DAG re-valued at p0 is not the
  p0 build's own bits": `boss_union`, `cut_cylinder`, `declared_tangency`, `die_chamfer`, `heat_sink_fins`, `measured_web`,
  `plate_param` and its parting, `a_chart_on_a_cap_tilted_at_zero`); `coincidence_door::a_boss_typed_tangent_to_a_cylinder_
  glues_one_unproven_tangency`, `self_intersection_proves_injectivity::a_notched_rod_..`, `verbs_germarms_interval::the_ring_
  lane_builds_at_the_certified_scalar` (all `Coincidence(Section, Moot)`, `section_site_foot_bend` or `MarginDiag(Invalid)`);
  `docm9 the_directed_search_proves_ground..`, `g_family_review_rows::a_familys_carry..` (`SlotVarKind`), `m10_di_dual_corpus
  ::at_rest_census_verdicts_match_f64_at_interval` (`loft_prism`), `m10_p_lift::at_sym_a_fixed_box_is_the_build`, `trim_3_
  windows_interval::a_holed_cylinder_wall_drops_the_cells_in_its_hole`, `sym_rule_f_rows::the_copysign_mint_sites..`.
- **Red at M9 and still, box arm off only (21):** the three `m10_3` rows of item 2, the m10_8 pin, `m10_10_pins` ×3,
  `m10_bulge` ×3, `m10_9_what_the_registry_buys`, `sym_9_the_drive_writes_the_ladders_receipt`, `sym11_the_exact_channel_
  never_contradicts_past_the_ceiling`, `m10_sym_profile ..plains_ledger..`, `decide_3_no_predicate_loses_a_decision`,
  `flagged_census::every_shipped_site_cites..`, sweep `a_lofted_arc_wall_is_a_function_of_the_sections_bulge`, the two
  frames rows (`..latitude_identity_is_symbolic..`, `..exact_caps_and_wide_walls`), `sym11_..never_contradicts_at_sym_
  interval`, topo `live::tests::the_public_surface_door..` (probe). Red at M9 and on main too: 5 (`corpus_finish_count`,
  `bounds_census`, `germ_tilted_rod`, `square_search` (fuzz), `recourse_roster`).
- **Stopped refusing:** 18 rows now pass (among them `m10_9_no_registrant_lies..`, `m10_9_the_pad_at_both_rule_f_dials`,
  `sym_9_the_kept_atom_ladder..`, `gap_reads_a_cell_against_a_round_face..`, `cylinder_sphere_frame::a_ball_across_..`, both
  `offer_rows`, two `a_corner_reads_only_its_own_sectors` rows); 15 rows no longer exist on main (the seven
  `sym11_witness_kind_rows` counter rows, the three `mass_props_are_thread_count_invariant` rows, `r1_two_registrations..`,
  two `m10_sym_profile` pins, `flagged_census::shipped_decide..`, `sym11_..counted_dispute_at_sym_f64`).
- **By first refusal in the box studies** (traced): the `u_ref` length (M4's kind A) is gone from every study and the tour
  (0 `SurfaceDatumUndecided`); new: `VolumeUncomputable` on `props_meridian_on_surface` (60 finishes, two m10_3 rows) and
  `RingContactEscalated` on `ring_outer_locus_gap` (2); still: kind B, every ray set aside on a depth-boxed slab past
  ±0.215 (32 in m10_3, 2 in m10_8); gone: kind C, the turning wall's `side_at_infinity_denom` (M4: 4); docm9: 0 refusing
  finishes (M4: 7).

## 6. One line each
- **A body finished again though its box does not move it:** on the tour, the plate (one content key in 1,023 replays) is
  24.3 s of 1,304.9 s, 1.9%; counting every node whose key repeats (the holes' keys repeat across splits of the axis each
  does not read) it is 967 s, 74% (finishes 858.7 s, build 108.4 s on keys already seen).
- **The repeated edge-door decisions:** 732 of 824 decisions of a slab finish, 5.0 ms of its 25 ms (20%); 35.7 of 81 ms of
  an a0 hole finish (44%, nearly all `ssi_hull_sup`'s margins); 3.7 ms of a ~1.1 s tour hole finish (0.3%).

## Method
- Probe: `boxfinish/probe-switch-off`, merge `a0cba9afe1` = probe 5dc671ed99 + `origin/main` 82fd6a46df (after PR 7394).
  Conflicts: `editor-core/src/drive.rs` (main's `witness_build(..)`; the probe's tally dump kept after it), `geom-core/src/
  sym.rs` ×3 (main's `discharge_in` returning the rung, main's definite-sign arm, main's `discharge_checked`; the S1 and S2
  substitutions are gone with the code they replaced; the M8/M9 helpers kept). Compile repairs, probe side only: one trace in
  `topo/src/solid_contain.rs` (`UnitVec3::get`), `LeafVerdict::Bisect { .. }`, the M9 counts delta reduced to main's four
  columns, and M3's ignored print rows gated out (`editor-core/tests/all.rs`). Switch-off unchanged (`AtRestPolicy for
  Interval` and `Sym<T>`: `Leaf = AtRestBody`, `TAKE_FINISHES = false`).
- Control: a worktree of 82fd6a46df, unmodified, its own target dir, built for editor-core `all`, sweep `all`, geom-core
  `all` and topo's lib tests. Build as before: ci.yml's env, the opt-0 wrapper, `CARGO_INCREMENTAL=0`, one heavy job at a time.
- Traces as in M4, M5, M8, M9 (`BOXFINISH_TRACE=1`/`3`, `BOXFINISH_M8=1`), on the same binary.

## Not measured, and why
- The switch-on tour on today's main (not built: disk; the tour panic at `chaintol.rs:297` is unattributed).
- `topo::all` and the profile/geom-core lib rows on the control (not built); so `recourse_roster` is placed by M9, not today.
- Why the revolved circle's row reads 1.01 s by `--exact` and 2.30 s under nextest on the same binary.
- The a0 hole node's ~375 ms of build outside its finish, by predicate.
