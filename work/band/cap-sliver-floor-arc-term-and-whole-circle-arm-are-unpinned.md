---
id: cap-sliver-floor-arc-term-and-whole-circle-arm-are-unpinned
kind: issue
title: blend: two terms of the cap sliver meter are pinned by no assembly row (the arc's floor term; the whole-circle arm of an arc extreme)
status: closed
opened: 2026-09-26
closed: 2026-10-14
priority: P2
cost: M
---


## Finding

Two terms of the ruled cut-off's cap meter (`CapSliver`,
`crates/sweep/src/blend/open/ruled.rs`) survive an unsound mutant with
every cap row green (measured on PR 3271's branch, filters
`band_ruled review_band ring_clearance fillet_h7 review_fillet_h7
cap_ring ladder annulus`):

- **The arc's term of `floor`.** `CapSliver::removed_sliver` takes
  `floor = low_arc.min(low_a).min(low_b)`. The mutant
  `floor = low_a.min(low_b)` (the arc dropped) turns nothing red. The
  arc's lowest point along `toward` is below both rim pieces' only when
  a rim piece spans more than π of its circle, which no cap fixture
  builds.
- **The whole-circle arm of an arc extreme, for caps.** The mutant
  `CircleFrame::misses` → always `true` (every arc reads its ends'
  values, never the whole circle's) reds only
  `review_ring_clearance_r1_probes::r1_a_bored_cylinders_off_axis_ring_reaches_the_ladder_backstop_at_the_front_door`
  — a ladder host's outer boundary — and the unit row
  `a_pieces_extremes_are_its_ends_unless_it_holds_the_critical_point`.
  No cap row reaches it: no cap edge beside a cut-off holds the point
  of its circle nearest to, farthest from, or extreme along `toward`
  from the section circle's centre.

## What the taker owes

An assembly row for each: a convex ruled crease whose cap rim piece
from foot to old vertex spans more than π (so the arc's term binds),
and a cap edge — a ring arc or a cut-cycle arc — whose window holds a
critical point, each placed so the mutant passes a carve the meter
must refuse. If no front door can build either, say which door stops
it, and let the unit row stand as the pin.

## Closed by (`band/cap-sliver-unpinned-terms`)

Both mutants re-run on main before the change over `band_`,
`end_face`, `ring_clearance`, `cap_ring`, `blend/`, `overrun`,
`fillet_h7`, `ladder`, `annulus/` (383 rows): the arc's floor term
dropped turned nothing red; `CircleFrame::misses` → `true` reddened
nine rows, but scoped to the cap meter's own reads
(`CapSliver::clearance` and `CapSliver::of`) it turned nothing red.

- **The arc's floor term is dead, and is gone**, with the arc's
  `reach` term, which is dead for the same reason (`CapSliver`'s doc,
  `crates/sweep/src/blend/open/end_face.rs`). The end curve's ends are
  the feet, which end the rim pieces too; inside its window an arc of
  the convex section is lowest along `dₖ`, or farthest from `center`,
  only where the section's outward normal points the way the function
  runs on, and `S`, outside the section, runs on past it there. So
  `S`'s extremes lie on the rim pieces whatever they span, and
  `CapSliver::of` reads `reach` and the floors over them alone.
- **The whole-circle arm is pinned on a cap edge**, for the farthest
  point and the extreme along a floor's direction:
  `band_planar_consume::a_bore_across_the_arc_at_its_edges_extreme_crosses`,
  a round bore whose half's farthest point from `c` and a D-shaped
  bore whose arc's extreme towards `V` cross the arc while each edge's
  ends read clear. Each arm forced to its ends' values alone, inside
  the cap meter, lets the bore past the cap to a bounded
  `FaceClearance`.
- **The nearest-point arm has no cap row.** An edge read clear by its
  ends against `reach` has both ends past `V`'s distance from `c`, and
  on the square block no arc that enters `S` from there stays inside
  the rims (searched numerically over centres, radii and windows). The
  unit row `a_pieces_extremes_are_its_ends_unless_it_holds_the_critical_point`
  and the ladder row through `ring_clearance` stand as its pins.
