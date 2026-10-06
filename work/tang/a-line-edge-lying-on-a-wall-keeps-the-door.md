---
id: a-line-edge-lying-on-a-wall-keeps-the-door
kind: issue
title: An undeclared line edge lying on a partner's wall keeps the door, with no lying-on lane for lines
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-10-06
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

## Review tier

SINGLE, FULL: the lying-on lane's ON decision for a ruling decides
which faces an edge belongs to; one full review.

## Closed (2026-10-06, TANG)

The `(Zero, Zero)` arm of `reduce::curved_face_arm` now hands a line's
`SpanVerdict::Constant` to `reduce::lying_on` under the same guard as
an arc's `LiesOn`: every parent decided distinct from the wall
(`parents_distinct_from`). Together with the arm's own endpoint rows,
that is the line decided ON the wall: its residual `Zero` at both ends
and its distance from the axis constant over the span (the axis-parallel
rung of `solid_contain::line_wall_roots` decided `Zero`). `lying_on`
reads a ruling through the interior question alone
(`carrier_cross::boundary_crossing`, which already held lines);
certificates (a) and (b) are circle readings. The defect started at
the arm, which sent every `Constant` to the frontier, and at
`lying_on`'s circle-only match.

`carrier_cross::meetings` now meets a boundary ellipse: a line or a
circle meets it where it meets the ellipse's plane. A curve lying in
that plane stays `Unread`; on a cylinder it cannot arise, since a plane
holding a ruling cuts the wall in rulings and one holding a rim cuts it
in a circle.

Rows (`crates/sweep/tests/a_ruling_lying_on_a_wall.rs`), all red on
2c73fc48 except the band guard:

- the probe, every op in both member orders, at both poses (0.3 and
  0.5 rad) and both extents, plus the prism turned onto each seam
  ruling (0 and π): exact census and contact records, closed-form
  volume, tiers 3 and 3′. A ruling cannot cross a seam ruling: the two
  are parallel, so they share a stretch or nothing, and the seam rows
  are that shared stretch;
- in band of the wall but not on it (a guard, green before too), by
  multiples of the band's zero threshold `z`: ends shifted `±3·z` off
  the wall escalate at the vertex placement; a `±3·z` lean escalates on
  the axis-parallel rung; a `±40·z` lean is a chord and keeps the door;
- a ruling across an ellipse, and the turned sunk dome on the slanted
  tube (`pi_seam_and_kiss_through_the_boolean.rs`,
  `a_turned_rim_on_a_wall_bounded_by_an_ellipse_builds_every_op_undeclared`,
  formerly `…_keeps_the_door`): every op in both member orders builds at
  its closed form, read to `max(1e-8, 10·ε)` because the wall's ellipse
  trim is measured by quadrature, at tiers 3 and 3′.

Filed: `an-ellipse-lying-on-a-wall-keeps-the-door` (P1, this section's
"The ellipse beside it"). Evidence added to
`a-union-keeps-valence-two-vertices-on-the-tubes-seam-rulings`.
