---
id: tier-3-reads-no-face-through-an-edges-wedge-within-a-solid
kind: issue
title: Tier 3's check never tests its own clause that no face passes through an edge's material wedge against a face the edge does not bound; only the census does, and the Boolean door runs no census
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [the-parting-reads-no-edge-lying-in-a-face-no-record-names, boolean-door-runs-the-census-over-its-result]
---

Found by a PARTING designer weighing
`the-parting-reads-no-edge-lying-in-a-face-no-record-names` (2026-11-01),
read from the code: the Boolean's `ops::gate` runs `validate_geometric`
and the line-wedges check, and `FaceThroughEdgeWedge` is raised only by
the census (`census.rs`), which the Boolean door does not run
(`work/reachhold/boolean-door-runs-the-census-over-its-result.md`,
parked P0, blocked on the cross-solid curved lane). So a parting that
misses an edge lying in a face ships silently: the measured instance is
a slab less a revolved triangle whose ridge circle lies in the slab's
top face (one closed edge, `validate_pseudomanifold` Ok).

**Fix.** Within each solid, test every edge against the faces its box
meets that it does not bound, with the census's existing wedge reading,
at the tier-3 gate. The within-solid half does not hit the cross-solid
curved blocker the door-census row waits on. Measure the cost and the
refusals it adds on main (a guard: P3, not the band of what it guards).
