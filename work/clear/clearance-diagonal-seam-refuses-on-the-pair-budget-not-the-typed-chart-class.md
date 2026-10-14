---
id: clearance-diagonal-seam-refuses-on-the-pair-budget-not-the-typed-chart-class
kind: issue
title: At the axis-order basis's diagonal seam the clearance engine refuses Budget(Pairs{65536}) instead of the typed 'chart does not refine' class it names at the axis-meridian seam
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [2468]
---

## What was found (PR #2468's review, filed by LINALG's merge lane, 2026-10-01)

`Vec3::orthonormal_basis` (the axis-order construction) picks its
world axis by `|n.z| − max(|n.x|, |n.y|)/2`. Its seam therefore has two
parts: the meridians where `|n.x| = |n.y|` swap which axis wins the
`max`, and the elevation `atan(1/2)`. A plane whose normal enclosure
straddles the seam gets a hulled stored `u_ref`.

The clearance engine (`editor_core::clearance`) reads every carrier on
its STORED chart. Its `window` door refuses a chart that subdivision
cannot narrow as `Unsupported` ("a face whose interval chart does not
refine"). The reviewer measured two outcomes:

- On a plane at the axis-meridian seam, the engine refuses with that
  typed class.
- On a plane at the diagonal seam — azimuth 45°, tilt 19.4712° or
  70.5288° — it subdivides instead, and ends on
  `ClearanceRefusal::Budget(Pairs { 65536 })`.

`ClearanceRefusal`'s own doc says a class that refinement cannot settle
has "its own class so a reader is not sent looking for a bigger dial".
A pair budget sends the reader to exactly that dial.

## What is not established

Whether `refines` passes there because the hulled chart does narrow
under halving, just too slowly to reach the band within the pair
budget, or because the test samples a halving that misses the seam.
The reviewer's fixture was not committed; re-derive it from the
numbers above.

## Home

CLEAR (`crates/editor-core/src/clearance.rs`'s `window` door and
`refines`).
