---
id: the-curve-face-pose-classes-the-carrier-not-the-span-a-trace-reads
kind: issue
title: The curve × face pose classes a trace piece's whole carrier, so a graze past the crossing a piece takes refuses every prefix of its run
status: open
opened: 2026-11-22
priority: P2
cost: M
---

Found by DOCTAIL, building Ev's ruling on [ev] #7353 in PR 7151. `docs/DESIGN.md` `[placement]`: "A trace uses the turn only at the crossing a piece takes … and decides nothing past that crossing."

## What

`topo::boolean::trace::ray_crossings_to` and `circle_crossings_to` read a piece's run for where its crossings are. Past a refusal they read the longest prefix that holds the crossing the piece takes, and the final read runs to halfway to the next crossing. That releases a refusal only where the refusal is local to the span.

The curve × face pose (`geom_brep::curve_face::pose`, C5's curve × face paragraph, "decided once, here, for every door") classes the piece's whole carrier. So a graze or an in-band pose rung past the taken crossing refuses every prefix of the run:

- **A line and a torus.** The line runs along `x` at `y = 1.5` in the middle plane of the torus with major radius 2 and minor radius 0.5 (frame at the origin, axis `z`). It enters the tube at `x = −2`, then touches the inner equator at `x = 0`. `ray_crossings` returns `TraceFault::Grazing` over every run tried (1, 2.5, 3, 3.9 and 4.5 from `x = −4`), well short of the touch. `Pose::Tangent` is "one nondegenerate double root", so the transverse entry is never listed. The same holds at offsets of 4e-16 to 1e-10 below the tangent line.
- **A circle and a cylinder.** The cylinder has radius 1 along `y` through `x = 1`, so its section in `z = 0` is the lines `x = 0` and `x = 2`. The circle has radius 2 about the origin, in `z = 0`, from `(−2, 0, 0)`; it crosses `x = 0` twice and touches `x = 2`. Every prefix of the turn escalates at `conic_quadric_sub_side`.

## Owed

A pose decided over the span a piece reads: a double root or an in-band rung that the span stands clear of leaves the span's own roots to its method, as C5 already says of "a double root past the span whose reach stands clear of it". A trace step then refuses nothing past the crossing it takes.

`topo::boolean::trace`'s `a_graze_past_the_taken_crossing_refuses_nothing` (the torus repro) is pinned red against this row. It goes green, and its pin red, when this lands.

