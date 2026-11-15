---
id: nightly-1115-residue-the-reds-no-lane-holds
kind: unit
title: Nightly 11-15: the reds no program's lane holds (two censuses, a moved pin, two 1e-12 rows, the probe rows, the render lanes)
status: open
opened: 2026-11-15
priority: P1
cost: M
---


From nightly 43680833019 (main `dbb21b4a`, 2026-11-15), each full-suite leg's junit read whole, with the six units of 2026-11-14 on main. Failures fell from 22 / 35 / 38 (11-13) to 10 / 11 / 15, every ε row reported all 19126 listed tests, and the three 1e-12 shards ran 42, 31 and 29 minutes. GREEN's rule holds (`work/green/program.md`): a red is greened here, not fixed.

**Difficulty letter:** M. **Review tier:** single review, STYLE — a pin or a census line can be read against what main produces; the render-lane change is small and its job's own run is the check. Both logged before any draw (2026-11-15).

## Not this unit's (owned, in flight)

BOXFINISH holds, at P0, the five `revalue_corpus` rows (PR 6963) and the two iso-image rows (PR 6959): `work/boxfinish/main-red-boxfinishs-merges-left-slow-rows-red-through-the-gates-slow-set-gap`. Leave them. `assemble_one_local_battery`'s gather row is ONEHOME's open row.

## What is red

Tests:
- `profile::all raw_door_census::no_production_source_writes_a_vertex_table` (all rows): `crates/mesh/src/trimmed.rs:2496, :3017, :3056, :3677`. Row on this slate: `main-red-the-profile-raw-door-census-names-two-lines-of-mesh-trimmed-rs`.
- `test-utils::all reader_census::every_site_that_reads_rust_source_is_in_the_ledger` (all rows; red on 11-14 too): owes ledger lines for `sweep/tests/a_blend_reads_the_wedge_through_the_one_fold.rs`, `topo/src/pcurves/lift_agreement_rows.rs`, `topo/src/pcurves/staleness_posture.rs`, and the line for `topo/src/boxes.rs` goes.
- `editor-core::all sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling` (all rows): four documents' receipts moved again after PR 6956 pinned them (two_hole_plate, r2_link, r2_filleted_bracket, r2_rounded_pad).
- `sweep::all mass_props_are_thread_count_invariant`, both the four-thread and the one-thread row, 1e-12 only: one line, `arc_loft_1e11eps`' pads. BOXFINISH's row names PR 6946 "to confirm".
- `editor-core::all coincidence_door::the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows`, 1e-12 only: `shaft_and_bearing_ball: the union certifies at Interval, escalating only where pinned`. PR 6967 re-stated this row yesterday and it passed locally at 1e-12 for the lane and the reviewer.
- Three fuzz rows red on this night's seed: geom-brep `curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance` at 1e-6 (seed 0x296907b8296cc745), topo `boolean::conic_quadric::cone_rows::certified_answers_hold_against_the_quadric_form` at 1e-12 (seed 0x5a2a16bda0f5438c; this slate's `cone-rows-certified-answers-fail-at-1e-12-on-one-seed` has an earlier seed), topo `boolean::section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section` at 1e-12 (seed 0x96ec838c8a0d073e).
- Probe feature (`suite under --all-features`, and the dev-probe sweep's plain half): `sweep review_chamfer_r1_probes::the_chamfers_probe_rows_are_exactly_its_own_questions` (`fillet3_convexity_sign` where the row lists `fillet3_chain_arm`).

Jobs:
- `k-lint (dev-probe)`: the sweep now runs every plain suite past its reds, then dies in the dump half at `k-probe sweep @ eps=1e-6 (demo scenes)`: demos `src/walls.rs` wall 8 "still refuses, but NOT with the refusal it pins" (`NoMarch { scalar: "telemetry probe" }`). So the 1e-9 and 1e-12 dumps, the marker and both lints did not run. Row on this slate: `the-tours-k-probe-reads-lily-wall-8-at-the-probe-scalar-where-it-refuses-nomarch`.
- `render lanes / scene inputs + uv sheet + wild montage` and `render lanes / freecad montages`: `RecursionError: maximum recursion depth exceeded` in matplotlib's `Path.__deepcopy__` under Python 3.14 (`demos/compose_montage.py` `fig.tight_layout`), red on 11-14 at the same steps too. `work/ciw/tool-versions-outside-the-env-block-have-no-source-of-truth` may be the class.
- `PR CI latency (alarm)`: median 20.8 minutes over 128 runs against the 20-minute alarm. Report only: say how much of the 24 hours' load was GREEN's own six lanes at tier `all`.
