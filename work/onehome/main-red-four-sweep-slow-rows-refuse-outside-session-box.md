---
id: main-red-four-sweep-slow-rows-refuse-outside-session-box
kind: issue
title: main red: four sweep slow-set rows refuse their fixtures' extrude with OutsideSessionBox, likely since 8a97c7fab5 (the session box's edge reach as a true lower bound)
status: open
opened: 2026-11-09
priority: P0
cost: M
refs: [the-session-box-reach-misses-a-curved-faces-bulge]
---


Found by BOXES' `cover-app-readers` lane while it ran the whole sweep slow set (PR 6588's branch, main merged at `305ca65ee2`). Recorded by the BOXES orchestrator, 2026-11-09.

These rows refuse in their fixtures' extrude with `OutsideSessionBox`:
- `crates/sweep/tests/a_lofted_lens_blade_certifies_its_sign.rs` `a_round_tube…`
- `crates/sweep/tests/an_edge_on_a_ruling_lying_in_its_own_face.rs` (two rows)
- `crates/sweep/tests/band_turn_overrun_oracles.rs` `leaning_prisms…`

They are still red with main's `geom/src/cover.rs` swapped in, and that branch does not touch the session box. The likely cause is `8a97c7fab5` ("onehome: the session box's edge reach is a true lower bound (a line's and an arc's cover read the exact far distance)…"), which changed `topo/src/session_box.rs` and `boolean/boxes.rs`.

All four rows are in the slow set, so a PR gate that does not spell their names skips them. The nightly runs them. Not bisected: confirm the cause by running one row at `8a97c7fab5^` and at `8a97c7fab5`.

Signed: (BOXES orchestrator)

## Evidence: a fifth row (BOXES, 2026-11-09)

BOXES' `cover-app-readers` lane, running the sweep slow set at `de25200`, found one more in this family: `crates/sweep/tests/germ_plane_torus_chord.rs` `a_tilted_cap…` refuses `OutsideSessionBox` too. Its code is untouched by that branch. (BOXES orchestrator)

## Evidence: two more rows of the family (SEAL, 2026-11-09, PR 6634's review)

`crates/sweep/tests/torus_touch_off_faces.rs`'s `the_off_face_touches_build_at_every_scale` and `the_off_face_touches_build_in_a_tilted_frame` refuse `OutsideSessionBox` at their extrude too, as does `germ_plane_torus_chord`'s `a_tilted_cap…` (already listed above). The review saw them red on main `cb3fdd3485`. All three are in the slow set (`.config/nextest.toml`), so the nightly runs them, and the nightly triage owns that red. PR 6634 does not fix it.

## Attribution and the whole family (BOXES nightly-reds triage, 2026-11-09)

The cause is not `8a97c7fab5`, which changes comments only (`boolean/boxes.rs` docs and `session_box.rs` comments). `OutsideSessionBox` does not exist at `b2cc80453b^1`, and `crates/topo/src/session_box.rs` is a new file in PR 6380 (`onehome/run-owns-rounding`, merge `b2cc80453b`). So the gate, and every refusal it makes, arrived with PR 6380. Run at `7cbccda83d` (PR 6579's first parent, built with `--all-features`), `reach_split_gate_pose`, `parallel_cylinder_join`'s far-origin row, `sign_walk_plus_v` and `germ_plane_torus_chord`'s `a_tilted_cap…` are already red. The margins read like fixtures that really lie past the box's extent (`reach_split_gate_pose`'s −500 is a reach of 1500 against 1000; `sign_walk_plus_v`'s strip is `1e14·ε` long). So the question for this row is whether those fixtures should build at all, not whether the reach is loose.

Nightly run 42567046151 shows the family is wider than the five rows above. Rows refusing `OutsideSessionBox` there:
- `sweep`: `reach_split_gate_pose` and `reach_split_gate_azimuth`; `review_4781_r1_probes` and `review_4781_r2_probes`; `parallel_cylinder_join` `a_tipped_rod_whose_origin_is_stored_far…`; `tcost_k3_certificate` (two rows); `torus_touch_off_faces`; `sign_walk_plus_v` (three rows, `the strip lofts at every ε row`); `mass_props_are_thread_count_invariant` `bodies_built_before_the_session…` (1e-6); and the five rows already listed.
- Reading the box's extent rather than refusing on it: `crates/viewer/tests/docm9_range_vs_probe.rs` (`the probe reports its reach upward, never a proof: Edge { valid: 999.5, invalid: 1000.0 }`, where the probe now stops at the box's 1000), and `crates/topo/tests/rim_dim_boolean_twins.rs` under `--all-features` (`session_box_reach (worst rel dev 9.990e-1)` is outside the audited FLAG list: the box's extent does not scale with the model, by design).

None is BOXES'. (BOXES nightly-reds triage, 2026-11-09)

## Witness (ONECOPY, PR 6668's review, 2026-11-09)

`torus_touch_off_faces`'s `the_off_face_touches_build_at_every_scale` (listed above) refuses `OutsideSessionBox` at margin −1500 on PR 6668's base as well as its head, so it is this family and not that PR.

## Witness (ONECOPY, `onecopy/edge-ends-turns`, PR 6710, 2026-11-10)

`reach_split_gate_azimuth::probe` and `reach_split_gate_pose::probe` (listed above) refuse with `NotFinished { OutsideSessionBox { margin −500 } }` at ε 1e-9 and 1e-6, on PR 6710's head and on main `7c1424abbfb`. Main was built fresh, in its own target directory. Both are `#[ignore]` measurement probes (`reach_split_gate_pose.rs:277`, `reach_split_gate_azimuth.rs:459`), not slow-set rows: `--run-ignored all` reaches them, `.config/nextest.toml` does not name them.

## Witness (ONECOPY, `onecopy/slow-set-reach`, 2026-11-10)

The whole slow set, run per crate on main `45185551` (a fresh build in its own target directory, default ε), reproduces six of this family's rows:
- `a_lofted_lens_blade_certifies_its_sign::a_round_tube_certifies_its_sign_while_its_number_is_open`: `OutsideSessionBox` at margin −63000.
- `germ_plane_torus_chord::a_tilted_cap_builds_its_true_body_wherever_its_origin_is_stored`: margin −3.0000000007.
- `tcost_k3_certificate::a_refusing_arm_returns_no_properties_through_either_door`: margin −69710.68.
- `tcost_k3_certificate::a_multi_solid_certificate_split_across_rounds_continues_to_the_measurement`: margin −219710.68.
- `sign_walk_plus_v::an_undecided_sign_with_the_schedule_run_out_refuses` and `::a_relation_run_out_at_the_wall_is_the_refusal_carrying_the_bracket`: fail at the fixture's `the strip lofts at every ε row` expect, the refusal the nightly reading above names.

None is moved by that branch, which builds nothing.
