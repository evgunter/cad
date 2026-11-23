---
id: the-charts-cut-side-choice-may-leave-the-log-once-measured
kind: issue
title: Whether the chart's cut-side choice (pcurve_chart_cut_side) is a renaming that may leave the log is unmeasured: sym11, m10_10, r2_rounded_pad, revalue and the census goldens may read it by name
status: closed
opened: 2026-11-03
priority: P2
cost: M
refs: [an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand]
closed: 2026-11-06
pr: 6559
---


From the anchored-angle decision (`an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand`, "Decided"). All three designers hold that the chart read's formula choice is a renaming in principle. But PR 5225's revalue rows and possibly the census goldens read `pcurve_chart_cut_side` by name. Measure what rests on the logged choice. If nothing does, move it under F31 like `Azimuth::near`, and drop `principal`/`principal_in`'s `band` argument. If something does, the ledger says what.

## Measured (PR 6559)

The choice stays logged. Decided at `Band::least()` under F31, across the workspace `ci` set at 1e-9 and 1e-12, exactly one row moves: `pcurve_cache::placement_rows::a_direction_on_the_windows_cut_reads_its_closed_end_at_every_scalar`, where a direction on the cut reads the window's open end instead of the closed end that placement's tie-break takes. Unlogged at the caller's band, no named reader moves. The audit row `pcurve_chart_cut_side` says what rests on the choice.
