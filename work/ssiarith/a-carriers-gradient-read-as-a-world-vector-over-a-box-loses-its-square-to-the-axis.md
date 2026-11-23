---
id: a-carriers-gradient-read-as-a-world-vector-over-a-box-loses-its-square-to-the-axis
kind: issue
title: ssi's implicit forms enclose a cylinder's gradient as a world vector over a box, whose readers dot it with a direction, so a component the true gradient has exactly zero is read wide (the shape topo::injective's Psi::rate fixed)
status: open
opened: 2026-11-22
priority: P2
cost: M
---


## Finding

Filed by EMBED's lane `embed/iso-end` (PR 7327, 2026-11-22), at review.

`topo::injective`'s carrier separator read a cylinder's gradient as a
Vec3 enclosure over a cell's image box, then dotted it with the cell's
chart direction. The true gradient is square to the axis at every point.
The box's enclosure holds directions with an axial part, and along a
ruling the direction is the axis, so the dot read only that slop: ±1.2e-5
over a cell 1.2e-5 tall at radius 0.006. That PR reads the rate along the
direction instead, the direction less its axial part.

The same shape is in ssi's implicit forms:

- `ssi/residual.rs`, `Implicit::gradient` (`:350`): `frame_gradient`
  over the frame box, then rotated to world as
  `frame[0]·g.x + frame[1]·g.y + frame[2]·g.z`.
  - In the frame, the cylinder's third component is exactly 0
    (`(2x, 2y, 0)`), and the torus's are read per component.
  - In world coordinates the three mix, so a world direction along the
    axis dotted with `g` reads the box's width.
- `ssi/enclose.rs`, the wall's `gradient` (`:608`): the form's gradient
  over the wall's box, handed as `[Interval; 3]` to readers such as
  `boundary::plane_partial`, which divide by it.

**Checked:** that both sites build the vector over a box and hand it on
as a vector. Not checked: whether any reader dots it with a direction
along the axis (a ruling of a cylinder or cone operand), and whether such
a reading moves a verdict. A probe is a thin box along a ruling, at a
radius below 1 and away from the origin.
