# Where the wedge reach's two bounds disagree

This is a measurement, not a conversion: no behaviour in it is pushed. The
instrumentation is `shadow.patch` beside this file, applied to the branch at
e17a7033.

## How it was read

- **The consumers.** Each composite consumer was run at the bound from below, as
  it is today. Where the door had read the bound from above for that lever, it
  was run again there, detached from the escalation log.
  - Wrapped at the consumer: `must_carry_over_edge` (MC: `must_carry_description`,
    `reask_pair`, `reask_chart`, tier 3's re-ask), `edge_wedge_over_as` (WV) and
    `classify_seam`.
  - Wrapped at the site: `classify_shared_rim` (`route_rim`), the singular turn's
    `Period::decide`, `bool_rim_face_opposed` (`stands_on`),
    `bool_touch_sheets_part` (`sheets_part`), `bool_rider_wall_side` (`walls_inside`),
    `edge_join::joinable` (dihedral, then second order) and
    `splitting::rules::edge_wedge` (dihedral, then `enters_material`).
- **The comparison.** A call reads EQ when its two verdicts' Debug is identical,
  PAYLOAD when they differ only past the variant names, and ACTION when the
  variants differ.
- **The decisions.** A decision is read off the verdict variant. A WV
  `Crease→InBand` is the class pass or the sign, an MC `JetDeterminate→InBand` is
  the class pass or the bend, and the turn's `Ok(k)→Undecided` is its own
  decision.
- **Coverage.**
  - ε = 1e-9: topo 4335, sweep 3831, step-import 464 and editor-core 3543 tests
    (all four crates).
  - ε = 1e-12: topo 4336 and step-import 461 tests; sweep its first 339 of 3831,
    and editor-core not run. Instrumented, the 1e-12 sweep runs at about 10 tests
    a minute.
  - Lanes: f64, `Interval`, Dual and Sym, wherever the suites run a consumer.
- **Not compared.** A lever not from a door (UNREG: MC 650, WV 14, seam 33 at
  1e-9) has no bound from above to compare.
- **Test failures.** The 28 failures at 1e-9 are the instrumentation's own: the
  site-wrapped boolean predicates are decided once more outside the detached log,
  and the symbolic ledgers count the extra forms. Neither moves a verdict compared
  here.

## Counts

| Consumer | ε | EQ | PAYLOAD | ACTION | ACTION with the bound from above poison |
|---|---|---|---|---|---|
| `edge_wedge_over_as` | 1e-9 | 9,771,947 | 179,231 | 22 | 22 |
| | 1e-12 | 1,927,256 | 34,114 | 20 | 20 |
| `must_carry_over_edge` | 1e-9 | 194,777 | 0 | 12 | 12 |
| | 1e-12 | 8,201 | 0 | 0 | — |
| `classify_seam` | 1e-9 | 1,168,518 | 0 | 0 | — |
| | 1e-12 | 224,261 | 0 | 0 | — |
| `pcurve_singular_turn` | 1e-9 | 2,685,893 | 0 | 47,527 | 47,527 |
| | 1e-12 | 1,207,400 | 0 | 13,514 | 13,514 |
| `classify_shared_rim` | 1e-9 / 1e-12 | 304 / 2 | 0 | 0 | — |
| `edge_join_joinable` | 1e-9 / 1e-12 | 38,148 / 2,487 | 0 | 0 | — |
| `rules_edge_wedge` | 1e-9 / 1e-12 | 3,848 / 2,642 | 0 | 0 | — |
| `bool_rider_wall_side` | 1e-9 / 1e-12 | 4,878 / 3,242 | 0 | 0 | — |
| `bool_rim_face_opposed` | 1e-9 / 1e-12 | 128 / 128 | 0 | 0 | — |
| `bool_touch_sheets_part` | 1e-9 / 1e-12 | 260 / 242 | 0 | 0 | — |

**At both ε, no action differs where the bound from above is finite.** Every
ACTION row has a poison bound from above:
- WV `Crease→InBand` (16 + 18) and `Seam→InBand` (4);
- MC `JetDeterminate→InBand` (12, all in
  `contact_edge_must_carry::every_contact_edge_on_the_corpus_is_jet_determinate`);
- the turn, `Ok(0|−1|1)→Undecided` (47,513) and `OnMark→Undecided` (14).

The PAYLOAD rows are WV only: `Crease→Crease` and `Seam→Seam`, whose margins
differ. The by-lane counts are in the per-run summaries; Interval, Dual and Sym
reads are in the EQ and PAYLOAD counts above.

## Where the poison comes from

A rerun of a sample of the ACTION cases logged each poison bound's source. There
are two:
- **NURBS faces whose carrier has no cover.** `Unknown::PoisonedNet`: the control
  net carries poison in some channel (`geom::cover::readings::net`). These are
  the bulk of the door's poison:
  - the placeholder-NURBS fixtures (`review_m3_pr4`,
    `geometric_cube::without_the_top_cap…`, `tier3_tests::check_1…`);
  - the contact-edge corpus;
  - step-import's snapped cylinders (`a_relation_the_file_band_reads_zero_is_snapped::*`);
  - `blend::reach::tests::a_pips_clip…`.
  A few more are a plane face with no bounding loop (`Unbounded`). Six reads had
  both faces covered and the bound from above still poison; not chased.
- **The site mint** (`pcurves::site_rows` → `edge_reach`), whose bound from above
  is poison by construction.

Both sources appear among the turn's cases. They were not attributed call by
call.

## The largest cases by ratio

The twenty largest below, all of them finite-ratio and all of them PAYLOAD
(`edge_wedge_over_as`, the same action at both bounds). Every ACTION case has an
infinite ratio (poison above) and is grouped above.

| ratio | below | above | body (test) |
|---|---|---|---|
| 2376 | 4.47e-4 | 1.0625 | `split_seam_vanishing_tilt::a_split_through_a_seam_vertex_returns_no_side_tier_three_refuses` (and two `an_aimed_ray_from_far_reads_a_split_side_right` rows) |
| 2255 | 4.71e-4 | 1.0625 | `a_vertex_on_a_grazing_plane::a_reading_through_a_vanishing_sine_offers_no_lever` |
| 1414 | 2.00e-3 | 2.83 | `a_tube_ending_on_a_ball::the_probe_family_ships_no_wrong_body` |
| 819 | 4.88e-3 | 4.00 | `a_tiny_lune_on_a_wide_sphere_reads_its_own_extent::…levers_every_edge_at_the_bodys_own_extent` |
| 434 | 2.45e-3 | 1.0625 | `a_rigid_turn_keeps_a_slivers_volume::a_turned_sliver_validates_and_measures_its_own_volume` |
| 238 | 4.47e-3 | 1.0625 | `a_vertex_on_a_grazing_plane::an_unresolved_section_reading_keeps_the_offsets_verdict` |
| 200 | 1.00e-2 | 2.00 | `section_end_poses::*` (three rows) |
| 23.8 | 4.47e-2 | 1.06 | `an_aimed_ray_from_far_reads_a_split_side_right::the_side_readers_reach_bounds_a_section_arc_not_its_ellipse` |
| 22.4 | 8.94e-2 | 2.00 | `snowman::a_cut_ends_at_the_nearest_of_several_boundary_hits` |
| 19.9 | 0.100 | 2.00 | `germ_plane_torus_spiric::the_fat_corner_bars_lens_is_the_true_body` |
| 14.0 | 6.92 | 96.7 | `reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt` |
| 13.4 | 0.300 | 4.01 | `review_arm_vs_reach_probes::review_curved` |
| 11.8 | 0.118 | 1.40 | `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_wider_than_the_wall` |
| 11.6 | 0.173 | 2.00 | `snowman::a_plane_through_a_caps_conventional_vertex_cuts_as_elsewhere` |
| 9.31 | 0.172 | 1.60 | `spheres_crossing_off_every_edge::a_ball_crossing_both_faces_of_a_lens_builds` |
| 9.24 | 0.306 | 2.83 | `a_tube_ending_on_a_ball::a_tilted_tube_builds_at_its_closed_form` |

Why ratios this large leave the action unmoved was not checked. Two
candidates: `folded_lever_arm` mins the reach with the faces' curvature arms,
which on curved faces can sit below both bounds; and a crease's `sin θ` can be
far past the band at either arm.
