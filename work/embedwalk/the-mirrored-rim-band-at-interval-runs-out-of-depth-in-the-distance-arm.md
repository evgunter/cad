---
id: the-mirrored-rim-band-at-interval-runs-out-of-depth-in-the-distance-arm
kind: issue
title: A three-arc filleted disc mirrored across a generic plane holds at f64 and runs out of depth at Interval in the distance arm, on a wall cell short of the rim against a band cell past the foot parallel
status: open
opened: 2026-11-22
priority: P2
cost: M
---


## Finding

From the `embed/collinear-cover` lane's fix pass (2026-11-22), on
`crates/sweep/tests/self_intersection_proves_injectivity.rs`
`a_three_arc_discs_filleted_rim_holds_across_its_joints`, its mirrored
pose: the three-arc disc with a filleted rim (`filleted_arc_disc`),
mirrored by `topo::mirror_body` across the plane through
`(0.3, −0.2, 0.1)` with normal `(0.48, −0.6, 0.64)`.

With one line read by key (the lane's fix), the body **holds at `f64`**
at every ε. **At `Interval`**, at the default ε and 1e-6, it reads
`Refused(Budget(Depth { max_cell_depth: 40 }))`: a different verdict
class from its `f64` twin.

The refusal is not raised by a presentation. A print at the depth stop
of the sweep's distance arm (`clearance.rs`, where a pair that is not
touching is split and `walk.split` refuses, `EngineRefusal::Budget`)
fires 23 times, all on the wall against the band, cylinder `3v1`
against torus `9v1`, `below` open. The first:
cylinder cell `u ∈ (0, 3.07e-3)`, `v ∈ (0.89769, 0.89913)`, which
stops short of the rim at `v = 0.9`; torus cell
`u ∈ (2.09235, 2.09542)`, `v ∈ (6.25864, 2π)`, past the band's foot
parallel, beside the joint at `u = 2π/3`. The two cells share no cell
of the boundary, so they are read by distance, and the torus's carrier
past its foot parallel runs down inside the wall within `3e-5` of it.

**Not traced:** why the pair reaches depth 40 at cells this size (the
cells printed are about `3e-3` wide, twelve halvings of the window, so
something other than halving is spending the depth), and why the `f64`
twin does not meet it. At `f64` the mirrored rows hold the parallel at
`0` and `4.0e-16`; at `Interval` each is a bracket about `1e-13` wide,
so the band's window may reach below its parallel there. The prints of
`vacuous`, the clip merge and `sides_are_the_faces` at their `Local`
exits show no small cell at all in this run: the cover is not what
refuses.

The row pins the twin at that refusal, and holds at both scalars the
day it goes.

**Reproducer:** `sweep::all self_intersection_proves_injectivity::a_three_arc_discs_filleted_rim_holds_across_its_joints`
(the mirrored pose; about 10 s for the row).

## On the landing tree (2026-11-23)

Measured above on `c76b74b2c`, where the band's sliver still closed the
tangent seams. On the landing tree (`origin/embed/wave3-land`
`5764e4b84` merged, the sliver retired) the mirrored twin refuses
earlier, at the top plane against the band:
`Refused(Budget { stop: Pairs { max_cell_pairs: 65536 }, a: 1v1, b: 9v1 })`,
the tangent seam of
`the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam`,
so this row's distance-arm stop is not reached there and the suite row
no longer asks the mirrored twin. It is to be re-measured once a
tangent seam is read at `Interval`.
