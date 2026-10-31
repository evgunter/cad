---
id: the-pierce-span-clearance-gives-up-on-a-face-with-a-curved-edge
kind: issue
title: The edge-face pierce span's boundary clearance gives up on a face with a curved edge, and a wide crossing enclosure there escalates
status: open
opened: 2026-10-27
priority: P1
cost: M
---

Found by the fix pass on `restread/curved-edge-in-face` (the cleared
disc, `link::Sector::cleared`), whose sweep for planar faces refused for
a curved edge listed it as not that unit's.

## What

`census.rs` `pierce_inside` places a line edge's crossing of a planar
face from both ends of the crossing's enclosure `S`. Where both ends read
`In` (or both `Out`) and `S` is longer than `2ε`, the ends answer for the
crossing only if no boundary of the face meets `S`, which `span_clear`
reads. `span_clear` answers `Some(false)` outright where the face is not
a polygon (`if !f.polygon { return Some(false); }`), so on a planar face
with any curved edge, however far that edge stands from `S`, the
crossing falls through to the straddle reading and pushes
`CensusEscalated` (`pm_census_ef_pierce_straddle`) on a valid body.

Its argument (the pierce-height reach `g·(1 + σ)`, then
`interiors_meet` and `on_edge_interior_reading` against each boundary
edge) is written for line boundary edges. A curved edge far from `S`
could be cleared by the same lower bound the cleared disc reads,
`link::clearance` (a line's segment, a curve's whole carrier through
`geom::Curve3::clearance`): an edge whose clearance from both ends of `S`
passes `L + g·(1 + σ)` past the band cannot meet `S`.

## Evidence

No measured body: no topo or sweep test is known to reach the arm with
a non-polygon face. A witness is an edge piercing a planar face with an
arc boundary, at a shallow angle so that the enclosure is wider than
`2ε`, far from the arc.
