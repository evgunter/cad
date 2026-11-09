---
id: side-at-infinity-aimed-rays-disagree
kind: issue
title: side_at_infinity's aimed rays read opposite sides of one solid: production takes the first ray's, so the side depends on face order
status: open
opened: 2026-11-06
priority: P1
cost: M
---


Found by `a-ladder-that-checks-its-answers-agree-has-no-home`
(`onecopy/refusal-standing`), whose first form wired a debug check of
the `Answering::Any` premise (`climb_any`: read every rung after the
answer and assert each agrees) into the `Any` ladders. That wiring is
held while the design fork on `Any`'s premise goes to Ev (`Any` kept for
theorems of the rule, an `Alike` kind for "every rung reads one fact of
one input", audited by a probe-gated test), so the check is not on
main.

## What

`side_reader::cast_aims` (`crates/topo/src/side_reader.rs:606`) climbs
`Answering::Any` over the selection's faces, one aimed ray per face. Its
premise, stated at `side_reader.rs:570-579` (`FAMILY`'s doc) and at the
climb: every ray from outside the body reads the one side, so the first
ray's side is the answer. **On the fixtures below that premise is false:
two rays decide opposite sides of the same solid.**

**Production takes the first answer** (`climb(Answering::Any, ..)`
stops at the first rung that answers), so the side `side_at_infinity`
returns depends on the order of the selection's faces, not on the
geometry. A reordering of the faces would flip a solid's side at
infinity, and every consumer of it (`point_in_faces`'s `missed_side`,
`census::sweep_cross_solid_backstop`, the shell-role reads) with it.

The site stays on plain `climb` with a pointer to this row
(`side_reader.rs`, above `let read = climb(`).

## The disagreement shape

Rungs are faces, each an aimed ray (`aim_at`, then `aimed_ray`). Under
the debug check the run printed 48 disagreements over the topo `ci`
suite: **`Ok(Out)` then `Ok(In)` 44 times, `Ok(In)` then `Ok(Out)` 4
times.** The backtrace runs `side_at_infinity` ← `missed_side` ←
`point_in_faces` ← `census::sweep_cross_solid_backstop`.

A probe (face, aim as `census::witness`, side) on two fixtures:

- `census::two_carrier_records::line_through_faces::a_plain_plate_is_confirmed`:
  faces 1-6 aim at `(0.382, ±0.236, 1|0)`, `(2.472, -1, 0.5)`,
  `(4, 0.236, 0.618)`, `(1.528, 1, 0.5)`, `(0, -0.236, 0.618)`, all
  `Out`; then **face 7 aimed at `(2, 0, 1)` reads `Out` and face 8
  aimed at the same point `(2, 0, 1)` reads `In`**. Two faces aimed at
  one point (an edge or corner the two share) read opposite sides: the
  aim on a shared edge looks like the lead.
- `split_operand_gate::an_inside_out_shell_under_one_solid_does_not_finish_at_f64`:
  faces 1-6 of a unit-ish box read `Out`; **face 7 aimed at
  `(4.9994, 0.6277, 1.0)` reads `In`**. Here the selection holds an
  inside-out shell, so `In` from its face may be the right reading and
  the premise (one side for every ray) the thing that does not hold
  for a selection that is not one oriented solid.

The ray direction is `aimed_ray`'s (`AIM_INCIDENCE` to the face normal,
turned by `aim_turn`); the probe did not print it.

## The failing tests

With the debug check at `cast_aims` (debug, topo `ci` profile), these
panicked with `an Any ladder's answers differ`:

- bool4r2_probes::the_per_solid_door_reads_the_container_s_own_sign_at_infinity
- census::records_by_carrier::a_face_pair_on_two_carriers_is_confirmed_along_its_tangent_line
- census::two_carrier_records::a_two_carrier_record_whose_extents_meet_at_one_point_is_stale
- census::two_carrier_records::a_two_carrier_record_whose_line_misses_a_notch_at_its_middle_is_confirmed
- census::two_carrier_records::an_interpenetrating_internal_tangency_is_not_confirmed
- census::two_carrier_records::line_through_faces::a_line_through_both_faces_with_its_sides_wrong_is_contradicted
- census::two_carrier_records::line_through_faces::a_plain_plate_is_confirmed
- census::two_carrier_records::line_through_faces::a_short_overlap_is_confirmed
- census::two_carrier_records::line_through_faces::a_tab_between_schedule_stations_is_confirmed
- census::two_carrier_records::line_through_faces::a_tab_on_a_schedule_station_is_confirmed
- census::two_carrier_records::line_through_faces::a_tab_width_reads_in_closed_form
- census::two_carrier_records::line_through_faces::an_edge_offset_from_the_line_reads_in_closed_form
- census::two_carrier_records::line_through_faces::an_extent_end_offset_reads_in_closed_form
- census::two_carrier_records::line_through_faces::review_5006_delta::a_record_through_a_spike_beside_an_apex_in_band_is_not_stale
- census::two_carrier_records::line_through_faces::the_line_along_the_plates_edge_is_stale
- census::two_carrier_records::line_through_faces::the_line_grazing_a_vertex_is_stale
- census::two_carrier_records::line_through_faces::the_notch_is_confirmed_in_every_row
- contact5_gate_and_beam::a_solid_whose_cavity_crosses_its_wall_blocks_its_pair
- shell_winding::a_void_outside_every_outer_refuses
- split_operand_gate::an_inside_out_shell_under_one_solid_does_not_finish_at_f64
- split_operand_gate::an_inside_out_shell_under_one_solid_refuses_at_a_dual
- the_fin_crown_winds_as_a_solid::the_fin_crown_is_a_solid_where_the_old_crown_crossed_itself
- tier3_tests::check_10_reads_every_shell_s_role_or_refuses_the_solid
- tier3_tests::the_census_gate_finds_a_far_slab_nested_in_another_solid

To reproduce: swap the site's `climb(Answering::Any, &geos, ..)` for
`geom_brep::ladder::climb_agreeing(&geos, .., same)`, where `same` holds
for two `Ok` answers that are equal, have its `Err(Disagree { .. })`
panic, and run `cargo nextest run -p topo --profile ci`.

## Done when

Either the rays agree on every fixture (a ray that reads the wrong side
is fixed or set aside, e.g. an aim on an edge two selected faces share),
and the site passes the every-rung audit; or the premise is retired and the
reader climbs every ray, refusing where two disagree
(`ladder::climb_agreeing`), with the consumers taking the refusal.
