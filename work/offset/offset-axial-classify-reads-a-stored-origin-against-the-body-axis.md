---
id: offset-axial-classify-reads-a-stored-origin-against-the-body-axis
kind: issue
title: offset_axial's classify reads a cylinder's and a plane's stored origin against the body's axis after a tilt levered at the body's extent
status: open
opened: 2026-10-07
priority: P2
cost: M
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---


## What

`classify` (`crates/topo/src/offset_axial.rs`) admits a face's carrier
as axial by a tilt levered at `frame.extent` (`offset_axial_alignment`),
then reads its STORED position against the body's frame axis:

- the cylinder arm: `on_axis(*origin)`, the cylinder's stored origin
  (any point of its axis) against the frame axis. A tilt `θ` in the band
  moves that reading by `θ` times the origin's distance along the axis
  from the body, which `extent` does not bound: a coaxial wall whose
  origin is stored 1000 m along reads off the axis and refuses
  `TogetherNotAxial`, and one whose origin happens to sit on the axis
  far away reads on it while the wall stands off it at the body.
- the plane arm (normal ∥ axis): `Constraint::Station(frame.station(
  *origin))`, the plane's stored origin (any point of the plane) given
  an axial coordinate; the error is `θ` times the origin's RADIAL
  distance from the axis, again unbounded by `extent`.
- `mint_carrier`'s cylinder seam line: `frame.radial(*origin)`, the
  seam line's stored origin against the frame axis, sets the direction
  the seam is translated along.
- `surface_residual`'s cylinder arm measures `p` about the frame axis,
  which is only as good as the classify read above.

The fix's shape is PR 4118's and this item's sibling: read each at the
foot of the body's own points on the axis (`geom_brep::Reach::foot_on`),
the stored origin never. The cone arms' `on_axis(*apex)` read a fixed
point and are not this defect.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep (refs). Read, not
probed.
