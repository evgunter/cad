---
id: curved-lane-triangulates-in-raw-chart-coordinates
kind: issue
title: The curved lane's CDT runs in raw chart coordinates, so a cone or torus whose two axes scale differently may fan an off-row boundary point across many columns
status: closed
opened: 2026-10-21
priority: P3
cost: E
closed: 2026-10-21
pr: 5560
branch: tess/raw-chart-lanes
---


## What

From #5531's FULL review (Q4), the sibling of
`trimmed-cylinder-lane-triangulates-in-raw-angle-by-height`.

`crates/mesh/src/curved.rs` inserts its points into spade at raw chart
coordinates (`insert`, `SpadePoint::new(u, v)`). On a cone the chart is
(radians, metres); on a torus it is (radians, radians) at different radii.
So a lattice that is square in 3-D can be a strip in the CDT's plane. #5531
measured that this is the shape that makes Delaunay fan an off-row point
across many columns. In this lane the off-row points are boundary chord
points: rim chords at the rims' own spacing and meridian chords at theirs,
beside an interior grid sized separately.

Unverified: the lane's domain is a convex rectangle, and its certificates
refuse typed, so a fan here would be a refusal and not a bad mesh.

## To do

Probe first, as the cylinder item does. Take a cone and a torus whose chart
aspect is far from 1 (a long thin cone; a torus with R/r large), and run the
review's twenty δ. If any refuses at a fan, or fans wide under the
falsifier, the curved lane takes the same face-wide power-of-two metric
(`nurbs_cert::lattice_metric`, applied at insertion only), and this item and
the cylinder item can ride one unit. If none does, close both with the
measurement.

## Resolution (2026-10-21): closed by measurement and by routing

**No trimmed face reaches this lane.**
- `crates/mesh/src/tessellate.rs:251-257` (`lane_of`) sends every face with
  an ellipse, B-spline or spiric edge to the trimmed lane
  (`trimmed.rs:187`, `has_trim_carrier`). On a cone, sphere or torus chart,
  that lane refuses typed `UnsupportedCurve` (`trimmed.rs:1235-1260`,
  `trim_frontier`).
- What reaches the curved lane is bounded by lines and circles only, and
  `curved.rs:197` (`require_iso_rectangle_face`) refuses
  `UnsupportedCurvedShape` for any domain that is not an iso rectangle.

So the lane's boundary points are rim chords on iso lines and meridian
chords on the rectangle's sides, and there is no trim point inside the
lattice to fan from. An obliquely cut cone or torus is a trimmed-lane
question, refused today (`work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`).

**Measured anyway**, full revolves at the review's twenty δ, all
watertight, with no refusal:

| body | chart stretch | δ scale | worst volume error |
|---|---|---|---|
| cone, r 20 → 10, h 2 | radians × metres, r ≫ 1 | r | 1.4e-2 |
| cone, r 1 → 0.5, h 1 | — | r | 1.3e-2 |
| cone, r 0.05 → 0.02, h 10 | r ≪ 1 | r (down to 1.5e-5, 4.2M triangles) | 1.1e-4 |
| torus R/r = 2, 20, 20, 100 | radians × radians at R/r | r | 1.4e-2 |
