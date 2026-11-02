---
id: offset-circles-centres-are-placed-by-differences-of-squares
kind: issue
title: offset_circles places a circle × circle fillet centre at along = (d² + ρ₁² − ρ₂²)/2d and √(ρ₁² − along²), the textbook radical form, not Circles' placement
status: open
opened: 2026-10-29
priority: P2
cost: M
---

Found by the sweep of `sect/circle-crossing-once`
(`circle-pair-crossing-is-spelled-three-times`), 2026-10-29. Not
reproduced at this site; the algebra is the one measured below.

## The site

`crates/profile/src/sugar.rs` `ArcCarrier::offset_circles`, the
circle × circle branch: its two clearances
(`fillet_offset_circles_external`, `_internal`) now read
`geom_core::Circles` (`reach()`, `internal()`), bit for bit as before,
but the candidate centres are still placed at
`along = (d² + ρ₁² − ρ₂²)/2d` down the link and `√(ρ₁² − along²)`
across it. That is the form the torus arms used before
`sect/circle-crossing-once`: over 800,000 seeded pairs (radii twelve
decades apart, centres up to 10³ from the origin) it put a crossing up
to 9.4·10⁵ ulps of scale off one circle, where `Circles2::crossings`
stays within 3.2.

## What the taker owes

Whether the centres take `Circles`' placement. `offset_circles` feeds
`arc_fillet_trims`, the ratified construction whose argument identity
`arc_fillet.rs` calls the whole bit-identity contract, so a move of its
emitted points is weighed on whether the new points are right, beside
`arc-fillet-corner-circle-pair-cancels-at-unequal-radii` (the
squared-radius rule LB4, which may or may not reach this site: its
offset radii are `R ± r`, not roots of squares).
