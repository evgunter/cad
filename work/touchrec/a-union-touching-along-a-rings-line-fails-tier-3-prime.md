---
id: a-union-touching-along-a-rings-line-fails-tier-3-prime
kind: issue
title: A result whose two solids touch along a slit's or ring's line, where one holds an edge, fails tier 3′ CensusUndecidable
status: open
opened: 2026-10-25
priority: P2
cost: M
refs: [boolean-door-runs-the-census-over-its-result]
---


Found by the claims review of PR 5544
(`cleave/boolean-closes-a-slits-face`); the same on main before it.
First noted on the closed row `a-boolean-places-two-germs-along-a-slit-line`.

## What

Two solids of a Boolean result can touch along an operand's slit or
zero-area ring line where the other holds an edge there. The result is
right by material, volume and tier 3, and tier 3′ refuses it
`CensusUndecidable` (the touch "along an edge and again", which the
census does not read). The Boolean door gates at tier 3 only
(`work/reachhold/boolean-door-runs-the-census-over-its-result.md`), so
the door returns these results `Ok`, uncertified at 3′.

Witnesses:

- the review's probe: the plate less the lying void
  (`a_ring_crossed_along_its_line_places_its_germs_apart.rs`) united
  with a pyramid standing on the top, its edge along the ring, in either
  order, at rest and turned: two solids, v16 e25 f14;
- `a_vertex_read_by_two_sector_passes.rs`, the cells it admits as
  `SLIT_TOUCH`.

## Fix

Read the touch along a line of coincident edges in the census, each
edge's material sector against the other solid's, as tier 3′ reads a
touch along a single edge. Then drop `SLIT_TOUCH` and pin the
standing-pyramid union passing 3′.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TOUCHREC holds a result's records of its own touches and the glue's kept copy; id and body unchanged. (CLEAVE orchestrator)
