# Step B, one stored form for every conic: what it moved

Measured 2026-11-21 on `oneround/anchored-conic-one-form` at `e04a87227`,
a work-in-progress commit on top of step A (`3193e7c97`, the phase).
Facts only.

## What that commit builds

- `Born` is gone from `crates/geom/src/curves/conic.rs`: nothing records
  how a conic was made.
- `Circle::new(frame, r)` and `Ellipse::new(frame, a, b)` store the anchor
  `frame.origin + frame.u_ref·a`, summed once, with phase `(1, 0)`.
  `Circle::anchored` and `Ellipse::anchored` are unchanged.
- Every conic evaluates from its anchor:
  `anchor + u·a·(cos θ − cos θ₀) + v·b·(sin θ − sin θ₀)`, the first offset
  read as `−(2·sin²(θ/2) − (1 − cos θ₀))`.
- `Conic::frame()` and `Conic::center()` derive the centre on each read,
  `anchor − u·(a·cos θ₀) − v·(b·sin θ₀)`. No centre is stored or cached.
- `Conic::anchor()` returns a point. `Curve3::data()` walks `Anchor` for a
  conic and `Phase` for an ellipse; the mesh memo key's and
  `step-import`'s `carrier_sig`'s anchor marks are removed.
- No test expectation, golden or bound is changed, apart from the few
  tests that asked how a conic was born and no longer compile that way.

## What ran

`cargo nextest run --workspace --profile ci --no-fail-fast`, default ε
(1e-9), debug build: 17,465 run, 17,420 passed, 45 failed. One of the 45
is red on the base `onehome/carrier-frames` at `ec8749552` without this
work (`pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`).
On step A's head the same command passes 17,465 of 17,466.

`rows.txt` lists the 45; `b_payloads.txt` has each one's panic line and
the lines after it. The ε = 1e-6 and 1e-12 rows and the slow set were not
run on this commit.

To reproduce one row: check out `e04a87227` and run
`cargo nextest run -p <crate> -E 'test(=<the name after the binary id>)'`,
where `<crate>` is the binary id up to `::` (`sweep::all` is package
`sweep`). For example
`cargo nextest run -p sweep -E 'test(=band_open_mixed_chain::a_teardrops_rim_fillets_whole_at_interval)'`.

## The 45 rows

### 1. At `Interval` (and the symbolic scalar), a conic built from a centre is wider

The door computes `anchor = c + u·a`; a reader of the centre gets
`(c + u·a) − u·a`.

| row | what it reads now |
|---|---|
| `sweep::all band_open_mixed_chain::a_teardrops_rim_fillets_whole_at_interval` | refuses: `Transversality`, enclosure `[0, 1.688e-9]`, zero band 1e-9 |
| `sweep::all band_open_mixed_chain_shapes::a_turned_l_plates_run_builds_at_interval` | refuses: same check, `[0, 1.520e-9]` |
| `sweep::all a_sweep_keeps_the_frame_it_is_handed::a_revolved_circles_latitude_over_a_centre_box_is_exact` | refuses: rung 3 `Stalled`, `ssi_hull_sup` `[0, 1.969e-9]` |
| `editor-core::all intent_s6_c_trace::the_traced_wall_moves_with_the_radius_at_dual_and_interval` | refuses: profile validation undecided, `[-1.419e-9, 1.419e-9]` |
| `geom-brep pcurve_cache::circle_lane_rows::a_needle_cones_far_section_is_covered_at_its_50_digit_distance` | Interval envelope `[1.07e-14, 1.175e-10]` against the 50-digit distance 2.723e-12 |
| `geom-brep::all bool5r1_probes::probe_the_coincident_plane_ladder_at_interval` | relative width 1.206e-13 at k = 20 |
| `topo splitting::classify::tests::a_graze_side_decided_zero_keeps_its_margin` | `None` where `Some(Zero)` (ε = 1e-6, centre 1·ε, normal 1·z) |
| `sweep swept::tests::the_placed_rim_chains_through_the_sketch_rim_to_the_radius` | symbolic scalar: `None` where `Some(Zero)` |
| `geom-brep pcurve_cache::limb2_memo_rows::a_hit_is_the_fresh_compute_and_a_changed_input_misses` | "Sym reads Interval's rounds" fails |
| `geom-brep pcurve_cache::limb2_quick_rows::a_span_hit_is_the_fresh_compute_and_a_window_shares_its_spans` | Sym reads 14 spans, Interval 12 |

### 2. At `f64`, a reader of the centre and the evaluator are a rounding of the semi-axis apart

The derived centre is `fl(fl(c + u·a) − u·a)`, not `c`.

| row | what it reads now |
|---|---|
| `topo splitting::containment::arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size` (a = 1e7) | the arc's point at −0.00045 lies 0.5062501916684451 from the cover's middle, past its radius 0.5062501912455183 (4.2e-10 m out) |
| `geom-brep::all edge_extent_bounds::each_kinds_bounds_bracket_the_diameter_and_stay_tight` | circle, closed: sup 4 is short of the evaluated diameter 4.000000000000001 |
| `sweep::all review_contact_edge_must_carry_r2_probes::r2_a_sphere_support_toward_osculation_never_reaches_the_rule` | a distance pinned 0.5 reads 0.49999999999999994 |
| `sweep::all revert_plane_charts::carried_cavity_certificates_equal_a_fresh_recertification` | stored 8.040613248383182e-17, fresh 6.123233995736766e-17 |
| `sweep::all snowman::the_fallback_assembly_carries_the_kept_operands_certificates` | stored 2.220446049250313e-16, fresh 1.2246467991473532e-16 |
| `step-import::all freecad::cross_dialect_fixed_point` | `box_fillet_corner`: the second round trip's export is not byte-identical to the first's |
| `topo ruled_region::ruled_region_rows::a_cones_circle_legs_read_no_chart_frame_at_the_escalation` | 36 queries a frame answers, 35 pinned |
| `topo torus_region::tests::a_torus_faces_circle_legs_read_no_chart_frame_at_the_escalation` | 51, 33 pinned |

### 3. An arc away from the anchor's vertex on a long ellipse

| row | what it reads now |
|---|---|
| `topo chord_join::seam_rows::an_arc_near_a_minor_vertex_resolves_at_its_own_scale` | an end misses its vertex by 6.48e-13; the row holds 1e-14 and cites 2e-12 for the un-re-seamed centre form |

### Moved, not checked against an oracle

| row | what it reads now |
|---|---|
| `sweep::all a_slit_tip_on_a_curved_face_reads_its_full_turn::the_slit_block_meshes` | `SelfTouchingTrimLoop { face: FaceKey(2v5) }` where it meshed |
| `geom-brep curve_face::ellipse_torus::torus_rows::a_graze_pair_at_the_seam_is_two_crossings` | seam 10.39044468670754: the exact oracle reads six roots (payload) |
| `topo boolean::conic_quadric::cone_rows::a_root_the_slack_meter_cannot_place_refuses` | d = 1e-5: certifies four roots where a refusal is pinned |
| `topo boolean::conic_quadric::in_band_sweep::a_root_whose_slack_reads_in_the_gap_is_refused` | certifies two roots, ±9.1396e-7, where a refusal is pinned |
| `geom-brep::all foot_and_phase_rows::a_circle_foot_is_its_angle_on_the_turn_at_from` | a foot on the axis from −7 reads −6.283185307179586 where −7 is pinned |
| `geom-brep curve_face::circle_torus::shallow_sweep::the_exact_oracle_agrees_with_mpmath_on_the_pinned_poses` | the exact oracle's 2.6859224115623483 is 1.467e-10 m from mpmath's 2.6859224114229985 |

### Checked earlier on the one-chain branch (`../phase2-one-chain/`), same payloads here

| row | reading |
|---|---|
| `geom-brep curve_face::ellipse_torus::torus_rows::the_slack_meter_charges_every_term` | "the top speed" certifies two roots; against the exact oracle they are 0.000 and 0.013 zero bands from the true ones |
| `geom-brep curve_pair::touch_spread_rows::the_subdivisions_touch_whose_spread_exceeds_the_band_escalates` | one meeting at (1.0000000000000098, 2, 0); twice the spread 1.20e-13 under a zero band of 2e-13 (5.13e-13 with the centre chain) |
| `mesh::all the_one_face_ball::a_slit_between_the_tips_is_not_the_strip` | `SingleColumnCurvedFace` on the slit face where `PinchWedge` on the ball face: the same body refused by another check |
| `sweep::all section_end_poses::a_torus_section_read_square_refuses_as_the_inputs_and_its_lever_builds` | `KernelLimit`, bound 1.0000000000000034e-9, where the typed `SquareToAxis` escalation |

### Bits, digests, counts and datum names

| row | reading |
|---|---|
| `mesh::all d9_mesh_goldens::every_corpus_body_meshes_to_its_committed_digest` | 19 of 48 body/δ pairs move |
| `mesh::all d9_mesh_goldens::the_corpus_digests_the_same_bytes_at_one_and_four_threads` | compares with the same committed digests |
| `mesh::all route_rows::a_body_that_meshes_records_mains_k_stream`, `a_section_tilted_within_the_band_meshes_to_mains_bytes`, `an_obliquely_placed_rectangle_meshes_on_the_rectangle_lane_to_mains_bytes` | mesh bytes move |
| `mesh::all the_one_face_ball::a_wrap_tip_cap_meshes_off_its_rim` | the hemisphere cap's mesh bytes move |
| `editor-core::all m10_9_r2_probes_interval::r2_circle_at_is_the_old_arm_at_f64_and_contains_it_at_interval` | one ulp in one coordinate at t = 0.7 |
| `sweep::all m5_s12_curved_ops::revert_uses_the_plane_normal_for_planes_and_the_sense_bit_for_charts` | a volume one ulp off its negation |
| `geom-brep::all r2_probes::r2_probe_sphere_seam_bit_move` | 2.500002041387553e-10 m where 2.5000014215237624e-10 |
| `geom curves::conic::tests::the_conic_readings_are_the_carriers_own` | the row writes the centre chain out by hand |
| `geom-brep carrier_resolution::tests::the_resolution_bounds_each_section_tables_carrier` | sphere: a miss at 0.106 of the bound (a tightness floor) |
| `geom::all curves_same_content::a_twin_one_ulp_off_in_any_stated_value_is_another_carrier` | "circle: a twin with centre[0] one ulp off" |
| `mesh memo::tests::the_curve_key_reads_every_scalar_the_walk_yields`, `topo tier3_tests::the_carrier_poison_read_reads_every_scalar` | the walk's first scalar is the anchor's, not the builder's centre's |
| `topo tier3_tests::the_carrier_datum_read_names_every_datum_that_describes_no_curve` | `Poisoned([Anchor])` where `Poisoned([Center])` |

### Red on the base without this work

`geom-brep::all pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`
(`MapResidual` margin 1.2503e-9 in the band).
