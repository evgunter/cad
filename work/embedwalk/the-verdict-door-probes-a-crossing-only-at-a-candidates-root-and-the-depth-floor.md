---
id: the-verdict-door-probes-a-crossing-only-at-a-candidates-root-and-the-depth-floor
kind: issue
title: The verdict door offers a straddling cell pair to its verifier only at a candidate's root and at the depth floor, so two footballs through one another read Crosses only at 64 times the pinned budget, and then on their seam
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [the-point-door-places-no-point-on-a-spindle-sheet-face]
---


## Finding

Found by lane `inside-spindle-sheet` (2026-11-25), giving the curved
point door its spindle sheet arm (`topo::contain`'s
`curved_interior_placement`, through `topo::torus_region`). The row
that asked for it expected that, once a sheet station is placed inside
its face,
`sweep::all self_intersection_proves_injectivity::footballs_through_one_another_never_read_embedded`
could ask `Crosses` of every case. It cannot at its budget, and the
placement is not what holds it back.

Measured with `topo::clearance::embedded` at f64 on
`footballs_through(R, R, by)` (that test's fixture), default depth,
`max_cell_pairs` varied:

| case | pairs | main | with the sheet arm |
|---|---:|---|---|
| lemons `R = −0.5` moved 0.3 | 1,024 | `Budget`, outside 0 | the same receipt to the digit |
| | 8,192 | `Budget`, outside 0 | `Budget`, outside 1,005 |
| | 65,536 | `Budget` (25 s), outside 0 | `Crosses` (18 s), outside 1,299 |
| apples `R = 0.4` moved 1.2 | 8,192 | `Budget`, outside 0 | `Budget`, outside 582 |
| | 65,536 | `Budget` (26 s), outside 0 | `Crosses` (16 s), outside 670 |

So the arm's `Outside` cells (vacuous discharges) are what lets the walk
reach a crossing at 64× the budget. The witness is still a pair of
boundary stations, both on the seam meridian `u = 0`
(`stations: [(-5e-324, 0.1506), (-5e-324, -0.1506)]` for the lemons):
there the two copies' seam meridians cross exactly, by the fixture's
symmetry. No interior station has been the witness in any run.

## Why

`Bound::pair` in `topo::clearance` offers an indeterminate pair to the
verifier only where `probe_here` holds: `pair.fresh` (a candidate's
root, or the first pair below a touching one) or the depth floor. The
strict question's separation is the norm of a difference, so it never
reads `Answers` short of both cells narrower than the band; an interior
crossing between two faces that share nothing is probed at the two
windows' root and then not again until the depth floor. At that root the
verifier's lattice closest pair is apex to apex, `0.3` apart, and its
Gauss–Newton step (`coincidence`) answers `None` from there, so no
interior point is ever offered to the point door. Instrumented at 8,192
pairs: four offers in the whole walk, all at the roots, each with nine
lattice stations a side and no coincidence.

The cell pairs near the crossing circle straddle zero at every depth the
budget reaches; none of them is a probe site.

## What is not known

Whether probing at other depths (every k levels, or where a pair's
enclosure first straddles zero after a clear parent) costs less than it
finds, and whether the coincidence step should start from a pair other
than the lattice's closest when that pair is two chart singularities.
