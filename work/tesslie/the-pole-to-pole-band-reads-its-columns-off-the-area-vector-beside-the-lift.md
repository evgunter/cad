---
id: the-pole-to-pole-band-reads-its-columns-off-the-area-vector-beside-the-lift
kind: issue
title: the walk's pole-to-pole band places its columns off the loop's area vector beside the stored lift
status: dispatched
opened: 2026-11-20
priority: P1
cost: M
refs: [singular-turn-spells-the-chart-winding-rule-by-hand, mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers]
branch: tesslie/lift-turns
---

Found by `tesslie/wrap-strip-period`'s sweep (2026-11-20), which moved
`mesh::walk::wrap_strip`'s period onto the loop's stored lift.

**The site.** `mesh::walk::loop_polygon`, the `band_u` arm for a rimless
loop that is not the wrap strip (the `else if no_rim` branch beside
`loop_area`): it folds the loop's 3-D area vector, reads its azimuth
against the chart frame turned outward (`mid_az`), and puts each
meridian's column on the branch `Period::near(u_raw, mid_az)`.

**Why it is a sibling.** Which `2πk` branch each meridian of a
pole-to-pole band takes is the band's width round the poles, a whole
turn. The loop's lift (`topo::Body::loop_lift`) stores it: the joint at
each pole is the face's turn there (`topo::pcurves` `singular_turn`,
the distinct-edges arm), and the lifted rows sit on the branches the
face occupies. The walk reads the same quantity a second time, off a
direction of a floating-point sum, which Ev's ruling on
`the-period-jump-has-no-ruling` (PR 4975) says a whole turn is never
read off. Nothing proves the two agree; `wrap_strip`'s two derivations
did agree on every mesh witness when measured, which is evidence of
the kind wanted here, not a proof for this arm.

**What closing it takes.** Measure first: instrument the arm to compare
each column's branch against the lifted row's (`LiftedRow::lift`) on
every rimless band in the mesh and sweep suites, and report any
disagreement as a live bug. Then read the columns off the lift, retiring
`loop_area` and the outward-frame read, as `wrap_strip` now does. A loop
with no lift then refuses `DomainUnread` as the strip and the trimmed
lane do.



## Re-banded P1 (2026-11-20, PR 7202 review)

A second derivation of a whole-turn branch the stored lift already holds, the reason `singular-turn-spells-the-chart-winding-rule-by-hand` stayed P1, and against the ruling on `the-period-jump-has-no-ruling` (PR 4975) that a whole turn is never read off numbers.
