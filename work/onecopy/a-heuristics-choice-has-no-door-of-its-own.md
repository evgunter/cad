---
id: a-heuristics-choice-has-no-door-of-its-own
kind: issue
title: A heuristic's choice decided on the least band takes the logged door unless someone argues it out (F31, F32): one door for such choices would make the next one land there
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by the design fork on `a-foots-parameter-is-read-without-its-certificate`.

Ledger rows F31 (`curve_foot_branch`, `angle_near`'s half-turn flip) and F32 (`curve_foot_polish`, the polish's tie-break) are one class. Each is a heuristic's choice between candidates, decided on `Band::least()` so it takes the sign alone, and each was first written through the logged `decide`. The logged door is the default, so each one escalated at `Interval` and gave the driver (`editor-core` `drive::log_read`) a cue no box answers, until a lane argued it out with a ledger row.

Other sites take the same shape on the logged door, `decide(.., Band::least())`. One is `geom::curves::on_carrier`'s `curve_carrier_speed`, which reads whether a speed bound is positive. A sweep for `Band::least()` across the kernel finds the rest. Some are verdicts rather than choices, so each needs its disposition before any moves.

Fix: give such choices their own door, for example `k_stats::choose(name, margin, ledger_row)`. It would be `check_unlogged` on the least band with the row required, and the least band would come off the logged `decide`'s callers. Then the next heuristic lands there by construction, and `flagged_census` checks its row.
