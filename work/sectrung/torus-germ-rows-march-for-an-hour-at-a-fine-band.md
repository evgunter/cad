---
id: torus-germ-rows-march-for-an-hour-at-a-fine-band
kind: issue
title: The torus germ rows' marches run past an hour in debug at ε 1e-12
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Filed by SECT's `sect-components` lane (branch `sect/components-by-foot`,
2026-10-29).

## Measured

Since the germ mint carries a torus pair's section (a site's component
is its foot), every union whose germs ride a cylinder × torus or torus ×
torus pair marches the pair at the mint. In debug at ε 1e-12, run
locally on the branch:

- `crates/sweep/tests/germ_torus_components.rs`
  `a_drum_against_a_moved_donut_passes_the_germ_mint` (12 unions) did
  not finish in 3 260 s; `two_donuts_in_general_pose_pass_the_germ_mint`
  (6) took 815 s and `a_ball_round_the_donuts_axis_passes_the_germ_mint`
  (2) 329 s. At the compiled ε the drum row takes 467 s.
- `crates/sweep/tests/germ_tilted_rod.rs`
  `other_spins_and_steeper_tilts_refuse_at_their_germs` (32 runs) did not
  finish in 3 420 s; on main it stopped at the mint without a march. At
  the compiled ε, after the torus ring path landed on main, it takes
  1 560 s, every run now reaching the result gate's volume.

At the nightly's settings (opt-level 2), the dual review of PR 6011
measured at 1e-12: the tilted-rod spins row 1 015–1 770 s, the drum row
400–700 s, the two donuts 520–900 s, against the `full-suite` job's 180
minutes.

The rows are in the slow set, so this is the nightly's cost. The PR 5972
log measured the door rows at 230–865 s each at 1e-12, so most of the
cost is likely the march itself (`ssi::torus_torus_ssi`,
`ssi::cylinder_torus_ssi`) at a fine band. A pair the mint marches is
marched once per operation (`SectionTable::carried`): the germ mint now
marches each held pair it mints a germ on, and the table is built per
operation at the glue door, so no march is shared across operations.
Four ops in two member orders march one pair eight times.

## Where the time goes (2026-10-29, `sect/march-once`)

At opt-level 2 on a shared 4-core box, at 1e-12 (1e-9 in brackets):
the spins row 1 170 s (195 s), the drum row 460 s (80 s), the two
donuts 609 s (109 s), the radial hole 170 s (26 s). Each op marches
its one pair once; the march is 156 s, 30 s, 30 s and 15 s of those
(3–16 %), the seats 49 s, 15 s, 19 s and 4 s. A gdb sample of the
drum's placement-0 ∪ at 1e-12 (43 s, 32 samples) found no sample in the
march: the pcurve mint after the build 11, the result gate's geometric
certificate 7, the join's chord-edge certificate 10 (inclusive:
`pcurve_cache::twin_span_hulls` 16, `ssi::certify::certify_branch` 10).
So most of the cost is the fitted chord edges' certificates, read at
the edge mint, the pcurves and the gate, not the march; the numbers and
the cross-operation question are on
`work/sect/a-surface-pairs-march-is-repeated-per-operation-and-per-germ.md`.

## What a fix has to supply

A measurement of where the march spends its time at 1e-12, and either a
cheaper march or rows sized for the nightly's budget.

## Rows past 300 s at 1e-12 (EPSROWS, 2026-10-31)

EPSROWS (CLEAR) ran the sweep rows that pin a refusal arm at
`CAD_TOLERANCE_EPS=1e-12`, on main `45f6be75` (4 cores, 300 s per row).
These ran past 300 s with no verdict. They are slow, not red: the
nightly has no 300 s cap.

- `ellipse_torus::a_torus_just_clear_of_the_rim_is_decided_by_its_roots`
- `germ_interior_oval::a_nurbs_graze_behind_crossings_is_refused_on_every_op`
- `germ_radial_hole::a_radial_hole_through_the_tube_stops_at_the_volume`
- `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
- `germ_torus_components::a_ball_round_the_donuts_axis_passes_the_germ_mint`
- `germ_torus_components::a_drum_against_a_moved_donut_passes_the_germ_mint`
- `germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint`
- `the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names`
- `the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names_against_the_rod`
- `the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names_against_the_stub`
- `the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names_the_tilted_ball_against_the_turned_rod`
- `the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names_the_unit_ball_against_the_turned_rod`
- `torus_doors_join::a_rod_across_the_inner_bound_stops_at_the_join`
- `torus_doors_join::a_rod_across_the_tube_stops_at_the_conic_trim`
- `torus_doors_join::a_rod_inside_the_tubes_shadow_stops_at_the_volume`
- `torus_doors_join::a_rod_off_the_axis_line_stops_at_the_join`
- `torus_doors_join::a_square_rod_on_the_outer_equator_stops_at_the_join`
- `verbs_cylsph_opening::a_transversal_pose_reaches_the_join_lane_in_both_poses`
