---
id: ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver
kind: issue
title: blend: the ruled cut-off leaves a cap ring that lies in the removed sliver on the cap, and tier 3 accepts the body
status: open
opened: 2026-09-25
priority: P0
cost: D
---


## Measured (2026-09-25, `band/ruled-d-hole-ring-crease`)

The D-rod at flat 0.3 (`rod_chord_at(0.3)`, extruded `ROD_L`) with a
second profile loop: a bore of radius 0.002 centred at `(0.297, 0.395)`.
That point is inside the rod (0.4967 from the axis), 0.003 from the flat
and 0.0033 from the wall, and 0.1087 from the upper crease's ball centre
`(0.2, 0.3464)`. So it lies in the corner region the r = 0.1 band
REMOVES: between the cut-off arc and the old vertex. The source is
tier-3 valid. `fillet_edges(&body, &rod_creases(&body), ROD_FILLET, tol)`
**carves**, `validate_geometric` on the result returns `Ok(())`, and both
caps still carry a ring. The bore's cycle is left on the cap face while
lying outside that face's new outer boundary.

## Why

`RuledPlan::plan` deliberately does not check the CAP's rings (its
support-gate comment in `crates/sweep/src/blend/open/ruled.rs`).
`ring_clearance_pass` (`crates/sweep/src/blend/surgery.rs`) meters the
rings of SUPPORT faces against the trimlines and has no cap arm. The
`mef` in `ruled_phase` step (1) leaves every other cycle on the old face
by construction. So a cap ring inside the sliver is carried into a region
the cap no longer covers. On the concave side (the sunk rod, the D hole
since this branch) the sliver is void, so no ring can be there. The gap
is the convex side's.

That tier 3 accepts the result is a second fact, on `topo`'s ground
(`validate_geometric` apparently does not check that a ring lies inside
its face's outer boundary). The taker should confirm it and file it
there.

## What the taker owes

Before any `mef`, meter every cap ring against the band's section
circle / the sliver region in closed form (the cut-off arc is a circle
of radius `r` about `CapEnd::center`, and a ring edge's carrier is
stored). Refuse `RingClearance` where the margin is not definite, with a
row at this fixture.
