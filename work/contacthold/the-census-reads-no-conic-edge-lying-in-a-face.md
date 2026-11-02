---
id: the-census-reads-no-conic-edge-lying-in-a-face
kind: issue
title: The census's edge-in-face passes read straight edges only, so a circle lying in a face it does not bound passes unread
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane]
---

Found by a PARTING designer (2026-11-01), read and measured: census
pass 4 and `sweep_edge_in_curved_face` (`census.rs`) take line edges
only, so a closed circle lying in a plane face it does not bound, with
both walls going into the face's material, passes the full census (the
fixture: a slab `brick((-3,3),(-1,0),(-3,3))` less the full revolve
about y of the triangle `(1,0),(0.5,-0.5),(1.5,-0.5)`, merged coplanar).
Also read: `sweep_edge_in_curved_face` leaves a touch from outside "to
the records", yet nothing demands that record, so the census passes a
contact it found with no record behind it.

The same designer reads the curve-touch confirm as "the edge's carrier
lies on the face's surface, and the face holds the piece" (C3 names the
locus as the edge's carrier), which `slit::carrier_on_surface` already
reads for a conic in a plane: worth handing to whoever weighs
`an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`.
