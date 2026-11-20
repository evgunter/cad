---
id: a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve
kind: issue
title: A Boolean section circle through a sphere face's pole mints no pcurve (NoGeneralLane, an azimuth gap), with or without a wrap tip at the pole
status: dispatched
opened: 2026-11-01
priority: P1
cost: H
branch: charttail/pole-section
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
