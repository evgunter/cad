# A centre-held conic read by chords at `f64` and from its centre at `Interval` and `Sym`

Measured 2026-11-21 on `oneround/anchored-conic` at `c3fb679d1` with a
temporary switch, reverted (`switch.patch`; the probes are
`probes.patch`). Facts only.

## The switch

In `crates/geom/src/curves/conic.rs`, for a conic built from a centre
(`Born::Centred`; one born anchored is untouched):

- `CV_CHORD=f64`: `Conic::point_at` and `Conic::running_point` read
  `fl(centre + u_ref·a) + u_ref·a·(cos θ − 1) + v_ref·b·sin θ`, the first
  offset as `−2·sin²(θ/2)`, at every scalar but `interval` and `symbolic`
  (so at `f64`, `dual` and the telemetry scalar). `Interval` and `Sym`
  keep `centre + size·direction`.
- `CV_CHORD=all`: the chord form at every scalar.
- The seam vertex is summed on each read, not stored. `running_point`
  treats it as exact.
- Nothing else changes: the stored centre, `frame()`, `center()`,
  `azimuth`, the derivatives, and every reader outside `conic.rs` that
  sums a point from a conic's frame itself.

## (a) Rows that hold an `Interval` enclosure against the `f64` point

- The row named,
  `m10_9_r2_probes_interval::r2_circle_at_is_the_old_arm_at_f64_and_contains_it_at_interval`,
  fails under the switch, on its **`f64`** clause: it holds `circle_at`'s
  bits equal to a centre-sum chain written out in the row, and one
  coordinate at `t = 0.7` is one ulp off. Its `Interval` clause compares
  the `Interval` `circle_at` with the `Interval` reading of the same
  chain; it reads no `f64` point and was not reached.
- No other row of the fast set that names an enclosure of the `f64` lane
  moved (the 26 that moved are listed under (b); among the rows found by
  name, `generic_replay::the_corpus_replays_at_interval_and_encloses_the_f64_lane`,
  `pole_slit_window::the_interval_walk_encloses_the_f64_window_on_every_slit`,
  `edge_nurbs::…on_carrier_domain_at_interval_brackets_the_f64_curve`,
  `offset_mint::…interval_mint_encloses_the_f64_mint`,
  `m10_2_measure::a_measure_at_interval_contains_the_f64_value` and the
  others pass at both ε). The search was by test name; a row asserting it
  under another name was found only if it moved.

Measured directly (`interval-contains-f64.txt`): 3000 random conics a
size, the arc through a point of the unit box and the centre `size` away,
the conic lifted to point enclosures, the `f64` point's three coordinates
tested against the `Interval` point's.

- **On the unit as it stands (centre sum at both), the `f64` point is
  not always inside**: over `|t| < 1e-3`, 32 to 52 of 3000 lie outside
  at every size from 1e2 m up, by up to 6e-15 m at 1e2, 6e-13 m at 1e4,
  6e-10 m at 1e7 (enclosures 4e-13, 4e-11, 4e-8 m wide); over the whole
  turn 0 to 2 of 3000.
- **With the chord at `f64`**: over `|t| < 1e-3`, 35 to 53 of 3000, by
  the same amounts; over the whole turn 0 to 6 of 3000, by up to 2.9e-11
  m at 1e5, 1.2e-10 m at 1e6, 9.3e-10 m at 1e7.
- With the chord at every scalar
  (`interval-contains-f64-chord-at-every-scalar.txt`) it is 137 to 150
  of 3000 at 1e4 over `|t| < 1e-3`, by up to 6.8e-13 m.

So the `Interval` point does not contain the `f64` point's bits to the
bit under any of the three readings; the switch adds whole-turn misses of
up to 1e-9 m at 1e7 and leaves the small-angle ones as they were.

## (b) The suite on the `f64` lane, `CV_CHORD=f64`

`cargo nextest run --workspace --profile ci --no-fail-fast`, debug build.

| ε | run | failed | failed on the unit without the switch | moved |
|---|---|---|---|---|
| 1e-9 | 17,467 | 27 | 1 (`pcurve_villarceau::an_in_band_move_is_never_read_off_the_torus`) | 26 |
| 1e-12 | 17,467 | 28 | 2 (that, and `reach_probes::a_ringless_cone_face_near_its_apex_meshes_sound`) | 26 |

`payloads-eps-1e-9.txt`, `payloads-eps-1e-12.txt` hold each row's panic
line and the lines after it. 25 rows move at both ε; one more at each
(the two `section_end_poses` rows). 25 of the 26 at ε = 1e-9 are among
step B's 45 (`../step-b-one-form/`), with the same payloads but where
noted; the one that is not is the first below.

### Roots past ε of the stored centre's circle

| row | what it reads |
|---|---|
| `topo in_band_sweep::a_circle_of_radius_1500_places_its_roots_within_the_zero_band` (not among step B's) | 50 answers in one run, 30 in the other (the row draws its poses afresh): at its ε = 1e-9 band, certified roots 1.1e-9 to 2.0e-9 m along the arc from the exact roots, at depths 2.2e-8 to 4.3e-7 m; at its ε = 1e-12 band, 2.4e-12 to 1.5e-10 m, at depths 2.7e-7 to 0.039 m. Every one has the right count of roots. |

Three of its poses taken apart (`probes.patch`, `probe2_radius_1500_pose`):

| depth (m) | without the switch | with it | the two exact circles' roots apart |
|---|---|---|---|
| −2.2e-8 | refused, `Meter { bool_conic_quadric_sub_root_slack }` | 2 roots, 1.22e-9 m from the stored centre's | 1.22e-9 m |
| +4.3e-7 | refused, the same | 4 roots, up to 1.99e-9 m from the stored centre's | 1.1e-11 to 2.0e-9 m |
| −3.4e-8 | refused, the same | 2 roots, 1.20e-9 m from the stored centre's | 1.20e-9 m |

"The two exact circles" are the one about the stored centre and the one
through `fl(centre + u_ref·a)` (the oracle run on an anchored conic at
that point, which reads its derived centre). With the switch the door's
roots on the centre-held conic equal the anchored conic's own (to 1e-22
rad) and lie within 4e-14 m of that second circle's exact roots. The two
circles differ by the rounding of the seam vertex, `ulp(1500)` = 2.3e-13
m a coordinate.

### A refusal becomes a definite answer

| row | what it reads |
|---|---|
| `topo in_band_sweep::a_root_whose_slack_reads_in_the_gap_is_refused` | certifies two roots, ±9.1396e-7, where a refusal is pinned |
| `topo cone_rows::a_root_the_slack_meter_cannot_place_refuses` | d = 1e-5: certifies four roots where a refusal is pinned |
| `geom-brep ellipse_torus::torus_rows::the_slack_meter_charges_every_term` | "the top speed" certifies two roots where a refusal is pinned (checked on the one-chain branch: 0.000 and 0.013 zero bands from the true ones) |
| `geom-brep touch_spread_rows::the_subdivisions_touch_whose_spread_exceeds_the_band_escalates` | one meeting at (1.0000000000000098, 2, 0) where an escalation is pinned |

### A build becomes a refusal, or one refusal another

| row | what it reads |
|---|---|
| `sweep::all a_slit_tip_on_a_curved_face_reads_its_full_turn::the_slit_block_meshes` | `SelfTouchingTrimLoop { face: FaceKey(2v5) }` where it meshed |
| `topo arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size` (a = 1e7) | the arc's point at −0.00045 lies 4.2e-10 m outside the cover |
| `mesh::all the_one_face_ball::a_slit_between_the_tips_is_not_the_strip` | `SingleColumnCurvedFace` on the slit face where `PinchWedge` on the ball face |
| `sweep::all section_end_poses::a_torus_section_read_square_refuses_as_the_inputs_and_its_lever_builds` (ε = 1e-9 only) | `KernelLimit`, bound 1.0000000000000034e-9, where the typed `SquareToAxis` escalation, in all six operations |
| `sweep::all section_end_poses::a_section_ends_own_rounding_is_a_kernel_limitation` (ε = 1e-12 only; not among step B's, which was not run at 1e-12) | `SectionUnresolved { Undecided { EndpointEnd } }` at margin 1.02e-12 in the band (1e-12, 1e-11) where a cut is pinned; a second pose `EndpointStart` at 7.3e-12 |

### Bits, digests and counts

| row | what it reads |
|---|---|
| `editor-core::all m10_9_r2_probes_interval::r2_circle_at_is_the_old_arm_at_f64_and_contains_it_at_interval` | one ulp in one coordinate at t = 0.7 |
| `geom conic::tests::the_conic_readings_are_the_carriers_own` | the row writes the centre chain out by hand |
| `geom-brep carrier_resolution::tests::the_resolution_bounds_each_section_tables_carrier` | sphere: a miss at 0.106 of the bound (a tightness floor) |
| `geom-brep::all edge_extent_bounds::each_kinds_bounds_bracket_the_diameter_and_stay_tight` | circle, closed: sup 4 is short of the evaluated diameter 4.000000000000001 |
| `geom-brep::all r2_probes::r2_probe_sphere_seam_bit_move` | 2.500002041387553e-10 m where 2.5000014215237624e-10 |
| `sweep::all review_contact_edge_must_carry_r2_probes::r2_a_sphere_support_toward_osculation_never_reaches_the_rule` | a distance pinned 0.5 reads 0.49999999999999994 |
| `sweep::all revert_plane_charts::carried_cavity_certificates_equal_a_fresh_recertification` | stored 8.040613248383182e-17, fresh 6.123233995736766e-17 |
| `sweep::all snowman::the_fallback_assembly_carries_the_kept_operands_certificates` | stored 2.220446049250313e-16, fresh 1.2246467991473532e-16 |
| `step-import::all freecad::cross_dialect_fixed_point` | `box_fillet_corner`: the second round trip's export is not byte-identical to the first's |
| `topo ruled_region_rows::a_cones_circle_legs_read_no_chart_frame_at_the_escalation` | 36 queries a frame answers, 35 pinned |
| `topo torus_region::tests::a_torus_faces_circle_legs_read_no_chart_frame_at_the_escalation` | 32, 33 pinned (step B read 51) |
| `mesh::all d9_mesh_goldens::every_corpus_body_meshes_to_its_committed_digest`, `…the_corpus_digests_the_same_bytes_at_one_and_four_threads` | 19 of 48 body/δ pairs move |
| `mesh::all route_rows::a_body_that_meshes_records_mains_k_stream`, `a_section_tilted_within_the_band_meshes_to_mains_bytes`, `an_obliquely_placed_rectangle_meshes_on_the_rectangle_lane_to_mains_bytes` | mesh bytes move |
| `mesh::all the_one_face_ball::a_wrap_tip_cap_meshes_off_its_rim` | the hemisphere cap's mesh bytes move |

### Step B's rows that do not move here

Nineteen of step B's 45: its ten at `Interval` and the symbolic scalar
((c) below), `chord_join::seam_rows::an_arc_near_a_minor_vertex_resolves_at_its_own_scale`,
`ellipse_torus::torus_rows::a_graze_pair_at_the_seam_is_two_crossings`,
`foot_and_phase_rows::a_circle_foot_is_its_angle_on_the_turn_at_from`,
`circle_torus::shallow_sweep::the_exact_oracle_agrees_with_mpmath_on_the_pinned_poses`,
`m5_s12_curved_ops::revert_uses_the_plane_normal_for_planes_and_the_sense_bit_for_charts`,
`curves_same_content::a_twin_one_ulp_off_in_any_stated_value_is_another_carrier`,
the two key-walk rows and the datum-name row.

## (c) The ten `Interval` and symbolic rows step B lost

All ten pass with `CV_CHORD=f64` at ε = 1e-9 and at ε = 1e-12 (the first
table of `../step-b-one-form/README.md` names them).

## (d) The chord at every scalar, those ten rows only

`CV_CHORD=all`, ε = 1e-9: all ten pass. The switch leaves the stored
centre and every reader of it as they are, where step B derived the
centre from the anchor.

## Not measured

- The slow set, ε = 1e-6, and the suite under `CV_CHORD=all`.
- Any reader outside `conic.rs` that sums a conic's point from its frame
  rather than calling `point_at`: it still reads the centre sum, so the
  26 include rows where two `f64` readings of one conic now disagree by
  a rounding (the certificate rows, the cover row).
