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

## Second route: through a ring of the cap (the keyhole)

The keyhole through-hole — block `[−1, 1]² × [0, 1]`, ring = disc
R = 0.5 plus slot `[√0.21, 0.8] × [−0.2, 0.2]`
(`review_band_ruled_ring_probes::keyhole_block`) — has two CONVEX
creases at the disc/slot junctions, ending in the caps' RINGS. With a
third profile loop `profile::circle((0.4623, 0.204), 0.001)` (inside the
material the r = 0.1 band removes at the upper junction), both creases
carve at r = 0.1, `validate_geometric` returns `Ok(())`, and
`ΔV` = −2·A exactly (`keyhole_cut(0.1)`) — off from the true body by
the bore's π·1e-6, which `mass_properties` still subtracts. The cut-off
ran in the cap's keyhole RING (not its outer cycle), and the bore's
ring now lies inside that ring's region. Buildable since
`band/ruled-d-hole-ring-crease` made ring cut-offs carve; before it the
keyhole refused.

## Why

`RuledPlan::plan` does not read the CAP's rings (its support-gate
comment in `crates/sweep/src/blend/open/ruled.rs` points here).
`ring_clearance_pass` (`crates/sweep/src/blend/surgery.rs`) meters the
rings of SUPPORT faces against the trimlines and has no cap arm. The
`mef` in `ruled_phase` step (1) leaves every other cycle on the cap by
construction. So a cap ring inside the removed sliver is carried into a
region the cap no longer covers. On the concave side (the sunk rod, the
D hole) the sliver is void, so no ring can be there; the gap is the
convex side's, through the cap's outer cycle (the D-rod) or a ring (the
keyhole).

Tier 3 misses both routes for different reasons. Check 9
(`RingOutsideOuter`, `crates/topo/src/validate.rs`) does test ring-in-
outer, but is silent on arc-bearing outer loops like the D-rod cap's —
filed as `work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk`.
The keyhole route leaves the bore inside ANOTHER RING, which no check
tests — filed as `work/atrest/check-9-does-not-check-a-ring-nested-inside-another-ring`.

## What the taker owes

Before any `mef`, meter every cap ring against the band's section
circle / the sliver region in closed form (the cut-off arc is a circle
of radius `r` about `CapEnd::center`, and a ring edge's carrier is
stored). Refuse `RingClearance` where the margin is not definite, with a
row at each fixture (the D-rod bore and the keyhole bore).

## Closed by (`band/cap-ring-in-sliver`)

Both routes measured red as stated, plus a third the item did not
name: a bore STRADDLING the cut-off arc also carved and tier 3 accepted
it. Every cycle a convex cut-off leaves on its cap — the cap's rings,
and its outer cycle where the cut runs in a ring — is now metered
before any mutation by the ring carry-through pass (arm (c) of
`ring_clearance_pass`, `crates/sweep/src/blend/surgery.rs`) under
`fillet3_ring_clearance`, against the annulus that encloses the sliver
(`CapSliver`, `rim_reach` in `crates/sweep/src/blend/open/ruled.rs`).
Rows: `band_ruled_cap_ring` (the D-rod: inside, straddling, two clear)
and `review_band_ruled_ring_probes` (the keyhole: inside, clear).

Deviations, each filed: the meter is an enclosing annulus by whole
carriers, so a definite negative reads "meets the annulus", not "meets
the sliver", and the bore wholly inside the sliver refuses rather than
vanishing
(`ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver`); the
cut cycle's own non-rim edges are not metered
(`ruled-cut-off-does-not-meter-the-cut-cycles-own-other-edges`).
