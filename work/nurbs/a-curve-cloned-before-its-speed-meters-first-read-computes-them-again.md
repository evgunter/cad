---
id: a-curve-cloned-before-its-speed-meters-first-read-computes-them-again
kind: issue
title: A spline cloned before its speed meter's first read computes the meter again in every clone
status: open
opened: 2026-11-20
priority: P3
cost: C
---


Found by SECT's `sect/ball-cylinder-cost` lane (2026-11-20).

## Measured

`NurbsCurve3`'s meters (`speed`, `speed_sup`, `accel`, `accel_spans`,
`crates/geom/src/curves/nurbs.rs`) are `OnceLock`s held by value, so a
clone taken before the first read carries an empty cell and each clone
computes the meter on its own; the field docs' "a clone carries the
memo of the same value" holds only for a clone taken after it.

In the unit ball ∪ the rod of
`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`
(default ε, opt-level 2, assertions off, 2.43 s), `speed_upper_bound`
computed 18 times, on 10 distinct carriers (the section's 501- and
997-point carriers, at `f64` and at `Interval`): 8 recomputes of a
value an earlier clone had already read, about 17 ms each, under
`certify::ReadSpec::of` → `decide_turn` → `carrier_speed::speed_range`.
About 0.13 s, 5 % of the operation.

## What closing it takes

Clones share one cell (`Arc<OnceLock<_>>`: a clone is the same value,
and every door that changes the fields builds a new curve with fresh
cells), with the session rule of `memo::session_memo` unchanged.
