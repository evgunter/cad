---
id: a-line-edge-lying-on-a-wall-keeps-the-door
kind: issue
title: An undeclared line edge lying on a partner's wall keeps the door, with no lying-on lane for lines
status: open
opened: 2026-10-06
priority: P1
cost: M
refs: [a-rim-lying-on-a-wall-across-its-seam-ruling-keeps-the-door]
---


## What

A prism standing inside the tube with one vertical edge on the tube's
wall refuses `CurvedPierceUnsupported` in every order, undeclared. The
probe, measured on branch `tang/lying-on-arc-splits-at-a-ruling`
(not committed):

- the tube: `rod_z(R = 1, 0, H = 2)` of
  `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`;
- the prism: the square `(1, 0)`, `(0.6, 0.4)`, `(0.2, 0)`,
  `(0.6, −0.4)`, turned `0.3` or `0.5` rad about `z`, extruded from
  `z = 0.5` by `1` (its edge inside one wall face) or from `z = 1` by
  `2` (its edge crossing the tube's top rim at `z = 2`);
- `t ∪ b`, `b ∪ t` and `t ∖ b`: all twelve refuse on the prism's edge
  against the wall face.

Both of the edge's parents are planes transverse to the wall, so they
are decided distinct from it, as a rim circle's are.

## Why

`reduce::curved_face_arm`'s `(Zero, Zero)` arm takes a lying-on edge
only on `SpanVerdict::LiesOn`, which the circle root door answers. A
line lying on a wall is a ruling, and the line root door answers it
`SpanVerdict::Constant`, which falls to the `_ => Err(frontier())` arm
whatever its parents are. `lying_on` itself reads circles only (its
`Curve3::Circle` match).

## Direction

The same lane as the rim's: a ruling whose parents are all decided
distinct from the wall (`parents_distinct_from`) is an ON event, its
interior read by `carrier_cross::boundary_crossing` (lines are in its
closed forms), split where it crosses the face's boundary and placed
by its ends otherwise. A `Constant` from a line merely in band of the
wall is not that case: the lane needs the line decided ON the wall
(the residual at both ends `Zero` and its axis distance constant), not
only parallel to the axis.

## The ellipse beside it

`lying_on` matches `Curve3::Circle` and returns `None` for anything
else, and `carrier_cross::boundary_crossing` answers `Unread` for an
ellipse. So an ellipse lying on a wall with parents decided distinct
keeps the door too. No fixture was found: the usual source of an
ellipse on a wall is a section of that same wall, whose parent shares
the carrier and is refused before this lane (the cosurface question
the D10 hold covers).

## An ellipse in the partner's boundary

The other side of the same gap. `carrier_cross::meetings` has no
closed form for a circle against an ellipse (its `_ => return Ok(None)`
arm), so `boundary_crossing` answers `Unread` for a lying-on arc whose
face's boundary holds an ellipse. Then `lying_on` keeps the door unless
certificate (a) or (b) already holds. This is pinned:
`sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`a_turned_rim_on_a_wall_bounded_by_an_ellipse_keeps_the_door`, the
turned sunk dome on a tube whose bottom is cut by the plane
`z = 0.5 + 0.2·x`.

The closed form exists. An ellipse is a plane section, so its meetings
with a circle are the circle's meetings with the ellipse's plane: the
same circle-against-plane roots `splitting::plane_crossing_lane` already
certifies, each then kept if it lies on the ellipse. That is complete
except when the circle lies in the ellipse's plane, which needs the
coplanar conic pair.
