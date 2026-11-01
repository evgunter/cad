---
id: coincident-line-wedges-are-read-at-one-point-of-a-turning-face
kind: issue
title: Tier 3′ and the Boolean gate read two coincident line edges' wedges at one point, which a face whose tangent plane turns along the line does not stand for
status: open
opened: 2026-10-25
priority: P3
cost: M
refs: [tier-3-prime-reads-no-side-between-coincident-edges-on-one-line, tier-3-prime-reads-no-side-between-coincident-curved-edges]
---


Found by review 1 of PR 5529 (branch `restread/coincident-wedges`),
by reading; filed in its fix pass.

## What

`census.rs` `line_wedges` reads each pair of coincident line edges'
material wedges at one point, the middle of the span the two share
(`wedge`, through `face_normal::face_outward_normal_at`). Tier 3′'s
pass 5 (`ee_overlap_sides`) and the Boolean's result gate
(`boolean/ops.rs` `coincident_wedges_disjoint`) both answer for the
whole span from that point.

For a planar face that holds: its sheet at the line is the same
direction all along it. A line can also lie in a curved face along a
ruling: a cylinder or cone, whose tangent plane is constant along a
ruling, and a ruled surface that is not developable (a hyperbolic
paraboloid, a twisted NURBS or loft face), whose tangent plane turns
about the ruling as it runs. On the second kind each sheet turns along
the span, so the two edges' wedges can be disjoint at the middle and
overlap towards one end, where one sheet has turned past the other.
Two arcs of a circle that are disjoint at one point say nothing about
the arcs at another.

## Witness shape

Two solids touching along a ruling `L` of a twisted face: solid A a
lofted block one of whose faces is the hyperbolic paraboloid
`z = xy` over `[0, 1]²`, with `L` the ruling `x = 0`; solid B a
brick whose face lies along `L` in a plane that agrees with the
paraboloid's tangent plane at `y = ½` (`z = ½ x` there) and leans
into A's material on one side of it. Their two edges on `L` read
disjoint at `y = ½` and overlap over the half of the span past it.
With the contact recorded, tier 3′ passes the pair today; the
Boolean's gate passes a result carrying the same two edges.

No shipped door is known to build this; the lofts and NURBS faces
`sweep` builds would be the place to look.

## Owed

The reading answers for the span only where each sheet keeps one
direction along it, which the face's surface can say (a plane, or a
ruling of a developable surface). Elsewhere, either read the wedges
where they could change order (where a sheet of one crosses a sheet of
the other along the span, the roots of one sign along the ruling) or
refuse named, as `TouchUnread::Unreadable` does for a face with no
sheet.
