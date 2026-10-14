---
id: cap-sliver-floor-arc-term-and-whole-circle-arm-are-unpinned
kind: issue
title: blend: two terms of the cap sliver meter are pinned by no assembly row (the arc's floor term; the whole-circle arm of an arc extreme)
status: open
opened: 2026-09-26
priority: P2
cost: D
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
