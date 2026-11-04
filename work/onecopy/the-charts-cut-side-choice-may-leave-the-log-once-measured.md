---
id: the-charts-cut-side-choice-may-leave-the-log-once-measured
kind: issue
title: Whether the chart's cut-side choice (pcurve_chart_cut_side) is a renaming that may leave the log is unmeasured: sym11, m10_10, r2_rounded_pad, revalue and the census goldens may read it by name
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand]
---


From the anchored-angle decision (`an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand`, "Decided"). All three designers hold that the chart read's formula choice is a renaming in principle. But PR 5225's revalue rows and possibly the census goldens read `pcurve_chart_cut_side` by name. Measure what rests on the logged choice. If nothing does, move it under F31 like `Azimuth::near`, and drop `principal`/`principal_in`'s `band` argument. If something does, the ledger says what.
