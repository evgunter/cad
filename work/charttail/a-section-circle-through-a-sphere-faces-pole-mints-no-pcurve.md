---
id: a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve
kind: issue
title: A Boolean section circle through a sphere face's pole mints no pcurve (NoGeneralLane, an azimuth gap), with or without a wrap tip at the pole
status: closed
opened: 2026-11-01
priority: P1
cost: H
branch: charttail/pole-section
pr: 7180
closed: 2026-11-20
---

Found by `splitdoor/wrap-tip-boolean` while pinning the wrap tip in the
Boolean, measured on main `9036ae9e` at ε 1e-9.

## What

The unit hemisphere `y ≥ 0` (a full revolve of the quarter disc about
y), intersected with a box whose bottom face is a plane through the
pole `(0, 1, 0)` tilted off the axis (normals `(1, 1, 0)`, `(0, 1, 1)`,
`(1, 2, 0)`), refuses before any result is built:

```
Pcurves { source: Certify { half_edge: …, error: NoGeneralLane { chart: Sphere,
  carrier: Circle, gap: Pose { piece: 0, channel: Azimuth } } } }
```

The same at every normal, on the hemisphere as the revolve leaves it
(two meridians at the pole) and on the one whose pole is a wrap tip
(`crates/sweep/tests/common/wrap_tip.rs`). The section is a circle on
the sphere through its pole, so its azimuth in the sphere's chart jumps
by a half-turn there: the pose lane has no piece for it. The answer is a
cap of height `1 − d` (`d` the plane's distance from the centre), a
closed form.

## Where to look

The sphere chart's circle lane (`crate::pcurves`' certify step): whether
a circle through a pole is read as two pieces meeting at the pole's
reset, as a meridian through the pole is.

## Re-measured: the pole is already a vertex; the arc ends there (2026-11-20)

Measured on `charttail/pole-section` at origin/main `d44f46d328`, with a
dump of the finished body just before the boolean's mint
(`boolean::ops`, the `mint_pcurves` call after `finish_output`), at
ε 1e-9.

- **Plane × hemisphere**, the box's floor through `(0, 1, 0)` at normals
  `(1, 2, 0)`, `(1, 1, 0)` and `(0, 1, 1)`, plain and wrap-tipped
  hemisphere alike: the cap the mint is handed is one sphere face and one
  plane face sharing ONE closed circle edge whose vertex is the pole
  (both half-edges run pole → pole over the whole circle). The merge took
  the meridian off the cap and the join closed the circle at its far site;
  it left the pole, as `edge_join::singular_at` requires.
- **The two-ball radical circle** (`tilted_sphere_pair`, δ = 0): the
  section arc that refuses runs FROM A's north pole, a vertex of four
  edges, to its other site.

So no section is left unsplit at the pole. The join already makes the
pole a vertex, as DESIGN's chart-singularity clause asks. What refuses is
the projected (general) lane: it cannot image a tilted circle arc that
ENDS on the sphere chart's singular set. The arc's first piece holds
the pole. Its radial hull reaches the axis, so `projected::piece_sector`
cannot place it in an open half-plane (`SectorRefused { piece: 0,
channel: Azimuth }`, relabelled `Pose` by
`pcurve_cache::general_lane_refusal`). The meridian class reads a pole
start already: the azimuth comes off the next-order direction, `d̂ = b̂_r`
where `a_r` is zero, in `pcurve_cache`'s closed-image derivation. The
general lane has no such reading.

At ε 1e-12 the `(1, 2, 0)` and `(1, 1, 0)` caps escalate instead
(`Escalated { GeneralLane, sample 0, margin ≈ −2e-12 / −3e-12 }`): the
hull's lower end sits on zero, read in band. This is the shape of the
1e-6 GAUGE red on the two-ball row
(`work/gauge/the-in-arc-pairing-turns-the-pole-arcs-lane-gap-into-an-indeterminate-at-eps-1e-6.md`).
At 1e-6 all six caps refuse `Pose`, as at 1e-9.

The fix therefore belongs to the general lane's image of an arc with an
end on the singular set, not to the join. Several answers are viable, so
the row carries a design question.

## Built: the pole end reads the reduced radial (2026-11-20)

The orchestrator chose the first answer above: a pole-ended piece in the
projected lane, with the end read from the vertex's incidence and never
from the carrier's end point. The fix is on `charttail/pole-section`.

- `topo::pcurves::singular_ends` decides each end of a half-edge's edge
  on the chart's singular points from its vertex, by
  `chart_singularity::vertex_at_singular_point`, the reading the walk's
  joints use. The mint's derivations, the lift's, and the site mint's
  (an existing half's vertices, a new edge's carrier ends, a split
  child's inherited end) hand it to `geom_brep::chart_pcurve_over_ends`.
- On a sphere, a conic's projected image with an end on the pole stores
  that end (`PoleEnd`). It reads its azimuth off
  `g = ±(b_r cos m − a_r sin m)`, `m = (t + t_p)/2`. This is
  `r(t) − r(t_p)` divided by `2 sin((t − t_p)/2)`, nonzero at the pole.
  The read covers its point, its piece boxes (so sector, branch, window
  and sweep) and its azimuth speed bound. Check 4 adds the pole term
  `2·L·(|r(t_p)| + e·sup|g|)` to fidelity.

The six caps (plain and tipped hemisphere at normals `(1, 2, 0)`,
`(1, 1, 0)` and `(0, 1, 1)`) build at `π h² (3 − h)/3` at ε 1e-6, 1e-9
and 1e-12, with tiers 3 and 3′ and a point lattice. The remainder builds
at `(1, 2, 0)`. At the two normals where the circle touches the base rim,
the remainder's face has a zero-angle corner and its measure refuses
(`work/nearpoint/a-sphere-face-with-a-zero-angle-corner-refuses-its-measure.md`).
The two-ball radical circle builds at the lens closed form.

## Closed (2026-11-20)

By PR 7180: an arc that ends on a sphere chart's pole images through the projected lane's pole-ended piece (its azimuth read off the reduced radial part `g`, its end decided from the vertex's incidence, the envelope charged a pole term). The plane-through-pole cap, its subtract at (1,2,0) and the two-ball radical circle build at their closed forms at ε 1e-6, 1e-9 and 1e-12. The section was already split at the pole; the gap was the lane's.
