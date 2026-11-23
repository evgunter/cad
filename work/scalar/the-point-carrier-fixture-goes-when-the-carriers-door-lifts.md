---
id: the-point-carrier-fixture-goes-when-the-carriers-door-lifts
kind: issue
title: The f64-body-held-as-points fixture is deleted, and its rows and the pinned refusals go back onto the production loft, when a carrier holds enclosure knots and weights
status: open
opened: 2026-11-22
priority: P2
cost: E
refs: [f64-built-data-is-built-at-t]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-22. Triggered by `work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md` S5z (the carrier's door admits an enclosure knot and weight).

A loft at an enclosure scalar is built in `T`. Its arc weights and chord-length parameters are enclosures there, and until a carrier holds one the loft refuses (`CarrierWeightNotAPoint`, `CarrierKnotNotAPoint`). Two kinds of row stood on main's lifted `f64` skin and were sorted in PR 7292. Both are interims, and this row is their end.

## Delete the fixture, and move its rows onto the production loft

`sweep::test_support::f64_body_held_as_points` and `f64_body_held_as_points_construction` (`crates/sweep/src/loft.rs`, `test_support.rs`) build the `f64` body and enter each carrier value at `T` as a point. They exist so a certifier has some spline-walled body at `Interval`. When the door lifts, delete them (and the `fixture` row for `crates/sweep/src/loft.rs` in `scripts/gates/from-f64-sites.tsv`), and build these rows from `loft_body` / `sweep_body` at their scalar:

- `crates/sweep/tests/common/semicircle_wall.rs` `semicircle_prism`, and through it `a_wall_row_read_in_band_takes_the_lane_a_miss_takes` (`a_seam_in_the_band_takes_its_general_image_at_interval`, `a_rim_in_the_band_takes_its_general_image_at_interval`) and every other user of that helper.
- `crates/sweep/tests/self_intersection_proves_injectivity.rs`: `lens_walled` (`lens_walls_hold_against_themselves_moved_off_the_origin_at_both_scalars`, `a_lens_walled_cap_holds_against_its_walls_at_its_tip_at_both_scalars`; both renamed from `a_lens_loft…`, and can take the loft's name back), `slab_wall_pair` (`a_gap_over_a_metre_is_read_in_metres`, `a_wall_pair_closer_than_the_bound_is_violated_not_separated` and the helper's other rows), `arc_spine` (`an_arc_spine_past_a_full_turn_reads_violated`, `an_arc_spine_short_of_a_full_turn_holds`, `arc_spine_walls_with_knots_ulps_off_their_splits_hold_against_themselves`). (The two sweep rows, the ribbon and the S duct, are back on `sweep_body`: a path sweep's knots are constants and it builds at `Interval` at every station count.)
- `crates/sweep/tests/r1_area_gauge_probes.rs` `r1_e2e_gauge_is_silent_at_the_interval_lane`.

The bodies are wider then (a few ulps per value), so a row that pins a count or a decision at an exact edge is re-read, not restored.

## Turn the pinned refusals back into what they ask

Each of these pins the door's refusal and says in its message what to ask once the loft builds. They go red when the door lifts; that is the signal.

- `an_arc_loft_seams_its_rational_wall_at_every_spacing::stacked_arc_sections_refuse_at_the_carriers_door_at_interval` (was `eight_stacked_sections_loft_at_interval` and the slow `many_stacked_sections_loft_at_interval`; the slow one goes back in `.config/nextest.toml`).
- `wire_loft_end_profile_lift::interval::the_cone_loft_refuses_at_the_carriers_door_at_interval` (was `each_seam_strut_at_interval_encloses_the_f64_world_points_exactly`; the struts then hold the `f64` points, no longer as point enclosures).
- `bool6_r2_probes::r2_middle_slab_band_edges_at_interval` and `bool6r1_probes_interval::an_interior_slab_at_the_band_edges_decides_the_same_at_interval`: the step the stacking door passes.
- `a_swaying_loft_corner_meters::a_swaying_loft_builds_closed`: the v-degree 2 bodies at `Interval`.
- `one_segment_loop::a_lofted_one_segment_section_is_one_spline_wall_with_a_wrap_strut`: the `Interval` leg (tiers 1 and 2, and tier 3 reading the one-segment wall as the two-arc walls).
- `a_loft_computes_its_walls_in_t`: `what_a_carrier_cannot_hold_refuses_typed_at_interval` and the two refusals in `point_inputs_loft_in_t_at_interval`.
- `demos/tour/tests/wall_jets.rs`: the lily blade and the Klein loop at `Interval` (pinned before this unit).
