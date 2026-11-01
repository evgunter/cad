---
id: a-torus-touch-along-a-whole-parallel-refuses-r-tan
kind: issue
title: A torus touching a coaxial or axis-normal partner along a whole parallel refuses R-tan
status: open
opened: 2026-10-06
priority: P1
cost: M
design: true
refs: [4159]
---


Found by `torus-touch-off-the-faces-refuses-at-the-section-pass`
(branch `reach/torus-touch-off-faces`), which gave the torus arms a
touch at an elliptic extreme and left this shape alone, as its brief
allowed.

## Measured (classifier rows on that branch)

`crates/topo/src/boolean/section_cert_rows.rs`:

- `coaxial_partners_meet_the_torus_in_parallels`: a coaxial wall of
  radius `R + r` is `Tangent("section_torus_coaxial_wall")`;
- `a_plane_tangent_to_the_tube_off_its_outer_half_refuses_as_a_tangency`:
  the plane `z = r` on the top circle is R-tan;
- `torus_tangencies_off_an_elliptic_extreme_are_not_touches`: a ball
  centred on the axis, radius `R + r`, is
  `Tangent("section_torus_sphere_near_tube")`.

Each touches the torus along a whole parallel. A coaxial torus whose
tube is tangent to this one's (`section_torus_coaxial_tube_reach`) is
the same shape.

## Why W2 does not clear it

The item that found it hoped W2 could clear these "as it clears the
crossing pose", the neighbouring sections being parallels. They are
only in the exact pose. The pose a decided `Zero` admits includes an
axis offset of up to the band (`axis_pose` decides `Coaxial` on an
offset `Zero`), or a plane tilt `s` with `s·R` in the band. There a
wall of radius `R + r − δ` cuts the torus in a crescent on one side,
and a plane `z = r − δ` tilted by `s` cuts a scrape oval. Both are
null on the torus, so W2 does not apply. The one-point `Touch`
(`section_cert`'s "A touch") does not apply as written either: its
argument reads one small loop about `at`.

## The question

The touch argument's path step seems to need less than a small loop.
It needs a CONNECTED region on the carrier, holding `at` and every
section component of every admitted pose, where the carriers stand
within the margin of each other. A thin annulus about the parallel is
one. If that is right, a touch along a parallel clears when one point
of it places `Out` of a face, on a pair with no event. Before anyone
builds on that, it should be weighed (designers first) against L1's
premise S, the margin the sweep records contacts at. Sibling:
`a-torus-touch-at-a-definite-non-elliptic-point-refuses-r-tan`.
