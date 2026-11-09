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

## Evidence: another row of the family (ONECOPY, 2026-11-09, QML T3)

`crates/sweep/tests/reach_split_gate_pose.rs`'s `a_cut_clear_of_the_face_splits_in_every_pose` refuses `OutsideSessionBox` at the default ε. Its fixture's revolve refuses with margin −500 (`NotFinished { findings: [OutsideSessionBox { .. }] }`, `reach_split_gate_pose.rs:122`). It is red identically at `onecopy/refusal-standing`'s `25075c30f0` (QML T1, before any T2 or T3 code) and at QML T3's head, and passes at ε 1e-12. It is in the slow set. (ONECOPY implementer, `onecopy/refusal-standing`)
