---
id: a-near-tangent-split-leaves-a-face-corner-that-runs-within-the-band
kind: issue
title: A near-tangent boolean leaves a face whose corner at the pierce point is 1e-8 to 1.6e-7 rad wide, so its two edges lie within the band for 0.06 to 1.0 from it; only the census's arithmetic keeps it from passing
status: open
opened: 2026-10-08
priority: P0
cost: H
design: true
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by this program's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

When a cube's face plane lies d rad off a prism corner's edge, it cuts the
faces at that edge along a line through `v` d rad off the edge. The
result keeps the piece between that line and the edge as a face. The
face's corner at `v` is 1e-8 to 1.6e-7 rad wide, so its two edges lie
within K·ε of each other for 0.06 to 1.0 from `v`.

- **At ε = 1e-9: 262 pairs**, all one vertex and one face, over the
  probe's 28 800 runs. They fall at d = ±1e-8 (233) and ±1e-7 (29), on
  every corner (asym 56, vee300 74, shallow200 58, notch307 28, w345 28,
  w60 18), and on both operand orders.
- 173 more such pairs at ±3e-8 (162 on one vertex), in a run without
  face keys.
- **Witness.** `notch307 nt e0 a3 d1e-8 pc S`. `EdgeKey(24v5)` runs on
  the prism's top edge from `v = (2, 1, 1)` to `(3.98308, 1.99154, 1)`.
  `EdgeKey(25v5)` runs from `v` to `(3.98308, 1.99154, 0.99999998)`.
  Both bound `FaceKey(11v3)`, and both are 2.217 long. sin θ is 3.6e-8,
  and they lie within K·ε for 0.28 of their length. The far vertices are
  8.9e-8 apart.

The body is the exact answer's (oracle volume; t2, certificate and
operand pass). But it holds two edges of one face within K·ε over a
stretch that no stage glued or refused.

Today tier 3′ escalates the pair only through the census's arithmetic:
`the-census-crossing-lane-misplaces-a-shared-points-crossing-on-a-near-collinear-pair`.
The exact crossing is at `v`, so a census with that fixed would pass the
pair silently, as it passes the two-vertex form of the same sliver
(`two-copies-of-a-pierce-carry-edges-that-run-within-the-band`).

**The stage.** The split: the section line cuts the face within the band
of the face's own edge over a stretch, and nothing reads that stretch. A
section line within the band of a boundary edge at one end and beyond it
at the other is one cut, so the question is whether the split may make
it.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row.

## The shape to give

A design question with several answers, to be weighed before a lane
builds it:
- glue the stretch: read the section line as the edge over the part
  within the band;
- refuse the pose typed;
- keep the sliver and certify it as a sliver: a census verdict that
  names it, rather than an escalation.

The answer governs this row, the two-copies row and the sliver lump
(`a-near-tangent-intersections-sliver-lump-reads-its-role-in-band-and-refuses`)
together.
