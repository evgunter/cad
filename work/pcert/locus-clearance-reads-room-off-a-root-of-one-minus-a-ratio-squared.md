---
id: locus-clearance-reads-room-off-a-root-of-one-minus-a-ratio-squared
kind: issue
title: locus clearance reads room = sqrt(1 - (s/r)^2), which floors at sqrt(2u) near s -> r, where sqrt((r - s)(r + s))/r has no cancellation
status: open
opened: 2026-10-13
priority: P3
cost: E
---

Found by the review of PR 5004 (`pcert/cone-section-interval-beta`),
which fixed a `√(1 − x²)` read near `x → 1` in `chart_pcurve`'s cone
arm. Not measured here: this is the arithmetic, and whether a verdict
moves is the owner's to say.

## What happens

`clearance` (`geom-brep/src/locus.rs:592`) reads
`room(s, r) = √(1 − (s/r)²)`: the cosine of a wall point's angle from
the locus, for a point standing `s` across on a wall of radius `r`.

- Near `s → r` the root carries about `√(2u)`, so `h − r·room` (the
  plane-cylinder arm) and `k` (the cylinders arm) carry about
  `r·√(2u)`, 2e-8 at `r = 1`.
- `√((r − s)(r + s))/r` reads the same value without cancelling.
  `r − s` is exact for close `s` and `r` (Sterbenz), and the clamp
  `max(0)` becomes a `max(0)` on `r − s`.

## What closes it

Read `room` as `√((r − s)(r + s))/r`, with a row at `s` within
`r·√u` of `r`, or show that the clearance's use is conservative in
the direction the rounding errs.
