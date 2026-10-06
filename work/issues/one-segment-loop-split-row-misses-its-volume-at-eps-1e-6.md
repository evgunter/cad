---
id: one-segment-loop-split-row-misses-its-volume-at-eps-1e-6
kind: issue
title: sweep: one_segment_loop's split-through-the-seam row misses its closed-form volume at eps 1e-6 on main
status: open
opened: 2026-10-06
priority: P1
---


(Found by the transversality-lever lane, 2026-10-06, running the `ci`
profile at ε 1e-6; reproduced on `origin/main` at e5e8a9c3df with no
branch change.)

## What

`sweep::all one_segment_loop::a_split_through_the_seam_builds_as_the_two_arc_form_does`
(`crates/sweep/tests/one_segment_loop.rs`, `close` ~79) fails at
`CAD_TOLERANCE_EPS=1e-6`:

> Along, phase 0, sweep 6.283185307179586: through the bottom seam
> vertex, one segment: volume 3.1415853098901643, closed form
> 3.141592653589793

The relative miss is 2.3e-6, against the row's fixed `1e-9` relative
bound in `close`. The row passes at the default ε. Either the volume's
accuracy is ε-scaled and the bound should be too, or the split body's
volume is genuinely short at a coarse ε; which one is undecided.

Filed in `issues/`: the path is `tcost` and `tint` territory by
`work.py territory`, and the feature row is
`work/paths/a-one-segment-loops-revolve-and-loft-wall-wraps-a-period-with-no-seam.md`.
