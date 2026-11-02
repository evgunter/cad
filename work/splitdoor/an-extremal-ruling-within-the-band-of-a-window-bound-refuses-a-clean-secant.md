---
id: an-extremal-ruling-within-the-band-of-a-window-bound-refuses-a-clean-secant
kind: issue
title: an apex sector whose extremal ruling lies within the band of its bound refuses sector_apex_window on a clean secant
status: open
opened: 2026-10-29
priority: P3
cost: M
---


## What

`sector_shape::apex_shape` decides whether each extremal ruling of the
plane's side `s(φ)` lies strictly inside an apex sector's window, using
`sector_apex_window`: the chord from each bound, levered at the arm. When
the extremum stands within ε–10ε of a bound, that reading escalates, and
the split refuses `SliverSector`, even on a clean secant whose bounds and
extremum all read definite sides.

The delta review of PR 5956 measured this with the ignored row
`review_5956_r1_probes::r3_an_extremum_at_a_window_bound_never_misleads`:
at ε 1e-9, 1772 of 5040 poses refuse, including at tilt 0.4. None of them
lands wrong.

## Where to look

The window reading only matters where the subdivision could change a run.
If the plane's side at the extremum (`split_bisector_side` on the
extremal ruling) is definite and agrees with the nearer bound, then
whether the extremum is inside or outside the window changes no run.
Either reading is sound, so the escalation is avoidable there.
