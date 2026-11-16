---
id: shoelace2-sums-about-the-chart-origin-so-a-thin-face-far-up-a-chart-can-read-the-wrong-orientation
kind: issue
title: planar::shoelace2 sums about the chart origin, so a thin trim polygon at a large chart coordinate can get its area's sign from rounding and refuse TrimLoopOrientation on a valid face
status: open
opened: 2026-11-14
priority: P3
cost: E
---


## Finding

Found by the review of TESSLANE's winding-rule unit (`tesslane/winding-rule`),
2026-11-14. Latent, and older than that unit.

`mesh::planar::shoelace2` sums `p.u·q.v − q.u·p.v` over the polygon's sides as
the coordinates stand, that is about the chart's origin. Over the reals the sum
does not depend on the anchor. In floats each term is of size `|u|·|v|` while
the sum is the polygon's area, so a thin polygon far from the origin loses its
area to cancellation: a sliver of a tall cylinder's wall at chart `v` near `H`
has terms near `u·H` and an area near `Δu·Δv`, and once `Δu·Δv` falls under an
ulp of `u·H` the sum's SIGN is rounding.

`trimmed::trim_orientation` reads `PolygonOrientation::of_signed_area` off that
sum and, on the cylinder and sphere arms, compares it with the face's sense. A
sign from rounding there refuses `TrimLoopOrientation` on a valid face, or
`MissingEntity { "degenerate trimmed boundary" }` if the sum lands on zero. The
emit passes (`planar`, `curved`, `trimmed`) read the same sign to wind their
triangles, where a wrong one flips a whole patch.

`walk::loop_area` anchors its fold at the loop's own bbox centre for exactly
this reason, and its docs carry the argument.

## What would witness it

A cylinder wall trimmed to a sliver (`Δu·Δv` a few ulps of `u·v`) with its `v`
range placed far along the axis from the surface's `origin`, meshed through
`tessellate`: the face is valid and refuses `TrimLoopOrientation`. A unit row
on `shoelace2` with a thin rectangle translated to large `v` shows the sign
without a body.

## What closing it takes

Sum about a polygon-local anchor (the first point, or the bbox centre as
`loop_area` does). That moves what the planar lane computes, so its digests are
re-derived with the cause named.
