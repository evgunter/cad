---
id: a-boolean-builds-a-reflex-edge-lying-inside-a-face
kind: issue
title: The boolean builds a reflex edge lying inside a face, and returns one solid for a wall-to-wall trench
status: parked
opened: 2026-10-12
priority: P1
cost: M
blocked_on: [4795]
---

Found by the keel design fork (`[ev]` PR 4795, designer reports in its
body), measured on main `360cf55eb2` at `Tol::witness`.

## What

The slab `[0, 4]² × [0, 1]` less the trench of
`a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section`
(prism over the (y, z) triangle (2, 0), (2.6, 2), (1.4, 2)), with the
keel resting on the floor face z = 0:

- x ∈ [1.5, 2.5] (both ends closed): `topo::subtract` keeps the keel as
  one edge between the two walls lying inside the floor face, with an
  Edge(keel)–Face(floor) record and two Vertex–Face records. Tiers 2, 3
  and 3′ pass. The keel edge's wedge reads ~327° of material; the
  material is two disjoint ~73° wedges with the floor between them.
- x ∈ [1.5, 4.5] (one end open): the same form, with Edge–Face and
  Vertex–Face records.
- x ∈ [−0.5, 4.5] (wall to wall): v14 e23 f11, **one solid and one
  shell**, no records, tiers 2, 3 and 3′ pass, volume 14.8. D1's
  definition of a solid (PR 3901) makes pieces touching only along an
  edge distinct solids; `topo::split`'s rule (b) gives two for the same
  geometry.

The split builds the same bodies in the slit form (two coincident
distinct edges; PR 4622 for the one-end case). One body, two
topologies, by door.

## Fix

Under PR 4795 as written, the boolean builds the slit form: no face
passes through an edge's material wedge, and the keel is two
coincident distinct edges (a zero-area ring where it ends inside the
face). If Ev rules for the rider form instead, the split changes and
this row is re-scoped. Either way the two doors must agree, and the
wall-to-wall case returns two solids. Nearest row:
`work/contacthold/a-bridge-union-fuses-a-declared-tangent-rest-into-one-shell-with-an-edgeless-contact.md`
(the curved analogue).
