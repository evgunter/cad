---
id: main-red-three-editor-core-rows-red-at-1e-12-since-before-11-19
kind: issue
title: Main red (ε = 1e-12): editor-core's e2e_channel_slider_over_an_epsilon_box and two revalue_corpus apart rows, red since before 2026-11-19
status: open
opened: 2026-11-21
priority: P1
---


Found by ONEROUND's lane (2026-11-21) on main `43080b1f`: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p editor-core --all-features --ignore-default-filter`.

## What

`m10_5_r1_probes_interval::e2e_channel_slider_over_an_epsilon_box`: the measured receipt is `(10, 1264, 1254, 532)`, the default row's pin, where the 1e-12 row pins `(10, 1258, 1248, 526)`. "verdict holds"; `deepest=13`, `regions decided=168`.

`revalue_corpus::revalue_boss_union_apart_to_its_parting`: "boss_union: 1403 margins moved and re-valued before the parting, under the floor of 1462 (Found { compared: 2360, moved: 1403, drifted: 0, opaque: 36, escalated: 0, … flips: [], mode: Some(Apart) …".

`revalue_corpus::revalue_plate_param_apart_to_its_parting`: "plate_param: 1803 margins moved and re-valued before the parting, under the floor of 1993 (Found { compared: 3246, moved: 1803, drifted: 0, opaque: 72, escalated: 0, … flips: [(3246, "point_in_loop_arm", 0.0, 2.6487939376878975e-10)], mode: Some(Apart) …".

## When

All three fail with these same numbers on `1360b9eb` (2026-11-19 19:50), `46e89240`, `f2c38057`, `48d98d55`, `6d58cebd`, `d3012f9f`, `ca6dd709`, `0ad2e0b5`, `02804869` and `43080b1f`. So they predate every ONEROUND merge of 2026-11-20 (PR 7145, 7146, 7143), and none of those moves a number. Not bisected past `1360b9eb`; the merge that moved them is older.

`m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials` is red at this ε too, with the message `work/scalar/the-race-rows-leaves-need-two-amounts-where-they-needed-six.md` holds for the default ε.

## Owed

A bisect before 2026-11-19 to name the merge, then whether each new reading is right (the e2e row's verdict holds; the revalue rows read fewer moved margins than their floors).

## Measured (GREEN U3, `green/editor-1e-12`, 2026-11-21)

From the scheduled nightlies' junit artifacts (every leg, read whole). Run 44927630239 (`dbc608ff`) is a `workflow_dispatch` on PR 7264's branch (`profile/pen-rectangle-over-a-box`), not main, and its readings carry that PR's moves (`plate_param`'s floor reads 1989 there).

| nightly | main | e2e slider | `boss_union` apart | `plate_param` apart |
|---|---|---|---|---|
| 44023697569 | `cdee7461` (11-17 05:22Z) | green at every ε | green at every ε | green at every ε |
| 44243918484 | `db86602a` (11-18) | no junit | no junit | no junit |
| 44680731754 | `8039ba06` (11-20) | red at 1e-6, 1e-12 | red at 1e-6, 1e-12 | red at 1e-6, 1e-12 |
| 44870651337 | `f5ef8f2b` (11-21) | red at 1e-6, 1e-12 | red at 1e-6, 1e-12 | red at 1e-6, 1e-12 |

So all three are red at 1e-6 too, and the move lands between `cdee7461` and `1360b9eb`. At 1e-6 the e2e row also reads `(10, 1264, 1254, 532)`, the default row's pin, against the 1e-6 pin `(10, 1282, 1272, 572)`: the receipt reads the same at every ε again.

The revalue floors were last set by PR 7114 (`doctail/verdict-log-tells-no-validation-reading-from-a-building-decision`, merged `e7d1a14b` 11-18 12:56Z, inside that range): 2539 → 1993 and 1619 → 1462, when the floor came to count the choices' margins only. Its body lists the revalue rows run locally at the default ε only. To be confirmed at its first parent and its merge.

### Bisected on hosted runners (2026-11-22)

Each probe is a throwaway branch `green/editor-1e-12-bisect-<sha8>`: the merge plus a workflow that builds editor-core (`--all-features`, opt-level 1, the gate's profile) and runs these rows with `--ignore-default-filter` at the default ε, 1e-6 and 1e-12.

- **The two revalue rows: PR 7114** (`doctail/verdict-log-tells-no-validation-reading-from-a-building-decision`, merge `e7d1a14b`). Green at every ε on its first parent `4ef175f0`; on the merge, green at the default ε and red at 1e-6 and 1e-12: `boss_union` moves 1414 (1e-6) and 1402 (1e-12) margins under the floor 1462, `plate_param` 1955 and 1802 under 1993. On main `1aa4a931` the same: `boss_union` 1415 / 1403, `plate_param` 1956 / 1803 (compared 2360 and 3246).
- **The e2e slider: after PR 7114** (green at every ε on `e7d1a14b`), red at 1e-6 and 1e-12 on `dbab861f` (PR 7139), reading the default row's `(10, 1264, 1254, 532)` at both. Narrowing among the eight source-touching merges between them.
