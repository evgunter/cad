---
id: the-cylinder-sphere-frame-pad-table-drifted-past-its-guard-on-main
kind: issue
title: cylinder_sphere_frame's MEASURED_PAD drifted on main up to 3.7x its entries, so a 4x guard sat at 0.91x
status: open
opened: 2026-11-12
priority: P3
cost: M
---

Filed by PCERT's lane on PR 6864 (its review's MINOR-3), whose own
share of the drift that PR re-measures.

## What

`sweep/tests/cylinder_sphere_frame.rs` guards each pose's widest volume
pad at four times its `MEASURED_PAD` entry. The table was written by
spiricarm's PR 6509 fix pass (`795dc76fab`, 2026-11-10). It was
measured in debug at ε 1e-9.

By main `9550b98e39`, several poses had drifted. Release, ε 1e-9, widest
pad over the pose's building runs, against its entry:

| pose | entry | widest pad on main | ratio |
|---|---|---|---|
| 0, centre on the wall | 1.4e-11 | 5.12e-11 | 3.66× |
| 1, centre inside | 6.9e-11 | 1.15e-10 | 1.66× |
| 2, centre outside | 6.6e-12 | 1.21e-11 | 1.83× |
| 3, turned chart | 1.0e-10 | 2.88e-10 | 2.88× |

The ratios are the review's. The pads are this lane's, measured with
check 4 forced to eight parts, which is main's reading.

So pose 0 sat at 0.91× its guard, and any further widening reddened it.
PR 6864's check-4 change widened it by 1.77×, and that PR re-measured
the whole table from its own build.

## Why it matters

A guard that has drifted to within 10% of its trigger is not measuring
what it says it measures. Nobody has said which change moved these
pads, or whether the move is right.

## What a fix could be

Bisect the drift between `795dc76fab` and `9550b98e39`. Decide whether
each move is right, and say in the table's doc which build each entry
was measured on.
