---
id: main-red-three-editor-core-rows-red-at-1e-12-since-before-11-19
kind: issue
title: Main red (ε = 1e-12): editor-core's e2e_channel_slider_over_an_epsilon_box and two revalue_corpus apart rows, red since before 2026-11-19
status: dispatched
opened: 2026-11-21
priority: P1
cost: M
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
