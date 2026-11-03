---
id: a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on
kind: issue
title: A section spiric on one operand's cap plane is asked against that operand's cylinder wall, which shares both its ends, and refuses unread: the tilted rod's β 0.7 union stops there
status: open
opened: 2026-11-03
priority: P1
cost: M
---


Filed by SPLICE's `splice/same-operand-rider` lane, from its Phase 0
reading of the tilted rod's stop (the fourth row of
`an-unread-rider-related-only-by-its-own-vertices-refuses`).

## Finding

`germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
pins the β 0.7 union (`h ∪ c`, spins 0 and π/2) at `"unread rider"`
(`EdgeInFace { CarrierUnread }`, `OTHER_SPINS_STOPS`). Measured on main
at `5f6aaf16` with a probe in `slit::riders` (not committed):

- the edge is a **section chord the join minted** (provenance `Mef` at
  one spin, `Mekr` at the other; its split root is itself, in neither
  operand's input), bounding the half donut's torus wall and the rod's
  plane cap: a spiric, in the cap's plane;
- the face is the **rod's cylinder wall** (the other operand's input
  face, through the graft);
- the two are related only by both ends lying on the wall by structure
  (`Names { ends: true, record: false }`): the chord ends on the rod's
  rim circle, where the cap meets the wall. No vertex-vertex partner,
  no record names the pair, no crossing.

So it is **not** the same-operand case: the edge belongs to no
operand's input, and the lineage exemption leaves it refused, as it
must. What leaves it unread is `slit::carrier_on_surface`'s
`(Spiric | Nurbs, non-spline surface)` arm, which certifies only off a
plane (`clears_plane`); a spiric against a cylinder has no reading.

## What would read it

A spiric lies in its own plane `P` (here one of its own faces' plane).
A cylinder meets `P` along a conic, so the spiric lies along a piece of
the wall only where it runs along that conic. That is a reading of two
curves in one plane (a spiric against a circle or ellipse, the
`geom_brep::curve_pair::meet` table's walk), not of the spiric against
the surface. Where `meet` reads the two as meeting at isolated points
only, the carrier is `Placed::Off` the wall. The same shape answers a
spiric against a cone or sphere (a conic or a circle in `P`), and a
torus wall in `P` (circles).

Rows: the β 0.7 union at spins 0 and π/2 moves on from `"unread rider"`
to its next stop (re-pin `OTHER_SPINS_STOPS` there), and a spiric that
does run along the rim conic still reads as lying on the wall.
