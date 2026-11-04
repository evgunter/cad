---
id: a-plane-at-the-bands-edge-reads-by-where-its-origin-is-stored
kind: issue
title: A plane one ε off a dome's top reads Unaccounted with its origin stored at the foot, a touch with it stored far along
status: open
priority: P3
cost: M
opened: 2026-11-03
---


Found by PR 6479's dual review (R1), while it built the row that pins
the pivot's padded band
(`crates/geom-brep/src/pair_section/spline_implicit/sweep/rows.rs`,
`a_plane_stored_far_along_itself_reads_the_touch_its_pivot_pads`).

## What happens

The fixture:
- the dome `z = −(x−½)² − (y−½)²` (3×3 biquadratic);
- the plane of normal `(0.1, 0.06, 1)`, normalised, one ε above the
  dome's tangent point `(0.55, 0.53, −0.0034)`;
- ε 1e-9.

With the plane's origin stored 1e5, 1e6 or 1e7 m along the plane,
`pair_section(.., Some(SsiLane::marched()), band)` reads `Counted`, one
`Point` (the touch). Unpadded, the band reads `Unaccounted`. With the
origin stored at the foot, the same plane reads `Unaccounted`, and it
did on the base too.

So "where its origin is stored changes nothing" (the module docs of
`crates/geom-brep/src/pair_section/spline_implicit.rs`, before PR 6479
corrected the sentence) is false at the band's edge. The pivot's shift
pads the band: a far origin has a larger shift, which widens the band
enough to decide the touch, and the near origin's band does not.

## What closes it

A reading at the band's edge that does not depend on where the origin is
stored: the touch decided on the pivot plane whatever the shift, or
refused alike at every offset. Add a row at the foot and at the far
offsets.
