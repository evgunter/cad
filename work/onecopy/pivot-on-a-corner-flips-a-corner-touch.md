---
id: pivot-on-a-corner-flips-a-corner-touch
kind: issue
title: A plane pose's pivot one ulp off a wall's corner decides the corner-touch row: exactly on it, the verdict flips from Apart to Reaching
status: open
opened: 2026-11-03
priority: P2
cost: E
---


Found by the frame-readings unit (PR 6463), which repointed `SplineImplicitPose::pivot` (`crates/geom-brep/src/pair_section/spline_implicit.rs`) at `geom::Frame::foot` and then reverted that one site.

`pivot` takes the net ball's centre's foot on the partner plane as `c − n·((c − o)·n)`, with `n = normal/|normal|`. The plane-foot home (`Frame::foot`, i.e. `Line::square_foot`) computes `c − normal·((c − o)·normal / normal·normal)` instead. On the probe's "a plane touching one corner" pair (`sweep/rows.rs` `probe_pairs`: plane `x + z = 0` through the origin, normal `(1, 0, 1)`, against the unit wall `y = 0`, `x, z ∈ [0, 1]`), the ball's centre is `(0.5, 0, 0.5)`:

- the old formula leaves the pivot a rounding off the corner `(0, 0, 0)`;
- the home's divides by exactly `2` and lands the pivot on the corner exactly.

With the exact pivot, `the_probes_pairs_are_classified_in_c5s_order` reads `Reaching(ChartReach { sides: [[true, false], [true, false]] })` where it wants `Apart`, at all three ε rows. So a one-ulp move of the pivot decides that row. Either:

- the corner touch's `Apart` relies on the pivot's rounding, and the boundary pass's corner graze is undecided at an exact pivot; or
- `Reaching` both sides at the touching corner is the honest answer for an exact pivot.

That is the spline-implicit lane owner's call. Once it is settled, the pivot can read `Frame::foot` and the inline formula can go.
