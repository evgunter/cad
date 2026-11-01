---
id: a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve
kind: issue
title: A Boolean section circle through a sphere face's pole mints no pcurve (NoGeneralLane, an azimuth gap), with or without a wrap tip at the pole
status: open
opened: 2026-11-01
priority: P1
cost: M
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
