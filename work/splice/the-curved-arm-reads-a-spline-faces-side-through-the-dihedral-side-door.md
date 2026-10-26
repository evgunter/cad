---
id: the-curved-arm-reads-a-spline-faces-side-through-the-dihedral-side-door
kind: issue
title: The sweep's curved arm still refuses a spline face before reading a side; the dihedral side door is not yet its spline arm
status: parked
opened: 2026-10-26
priority: P1
cost: E
refs: [a-span-is-read-against-a-faces-boundary-edges-first, the-side-of-a-spline-face-is-read-by-no-dihedral]
blocked_on: [a-point-has-no-place-in-a-nurbs-faces-trim]
---



Filed by the SPLICE lane that closed
`the-side-of-a-spline-face-is-read-by-no-dihedral`.

`geom_brep::dihedral_side` (`crates/geom-brep/src/dihedral_side.rs`)
reads which side of a face a second face leaves a shared edge on, for a
spline face as for an analytic one, through the scalar's NURBS lane
(`T::nurbs_lane()`). Nothing in the boolean calls it yet. The one site
that has to is `curved_face_arm` in `crates/topo/src/boolean/reduce.rs`,
which refuses `CurvedBooleanUnsupported` on a NURBS or `Approx` face
before any side is read ("The residual sides one would read,
`implicit_residual` and `classify_dihedral`, are poison on a spline
surface"). That file is the boundary-first lane's
(`a-span-is-read-against-a-faces-boundary-edges-first`), so the wiring
waits for it.

Once the boundary-first reading has found an edge's run along a spline
face's boundary edge, the edge's parent faces' sides against that face
are `dihedral_side(parent, spline face, p, walk, reach, lane, band)` at
a point of the shared run. `walk` is an `EdgeWalk`: the topo seam mints
it from the parent face's own half-edge as
`EdgeWalk::along(curve, t, he == edge.he_plus)`, and this row writes that
mint (a `Body` helper taking the half-edge), so no caller assembles the
bit by hand. A `Tangent`, `Crease` or in-band answer keeps its
escalation.

**Parked on point-in-face.** The door decides a spline face's foot on
its CARRIER and reads no trim. A carrier that passes within the band of
the point on a sheet the face does not use gives the foot on that sheet
(the hairpin witness in `a-point-has-no-place-in-a-nurbs-faces-trim`).
So the side is the face's only once the foot is placed in the face. The
wiring places it through that row's NURBS arm before it consumes the
side.

The one-sided cover's side reading (`read_cover_side` in
`crates/topo/src/boolean/mod.rs`) samples `implicit_residual` on the
target: on a spline target that reads poison, every sample escalates,
and the cover stays `Unread`. That is sound (no cover is claimed) but
it loses the cover. A spline target there needs the curve × face
clause's foot reading, not this door: a cover's side is global, and
this door's side is local to the edge.

Witness: lily wall 8 (`demos/tour/src/lily.rs`) moves off
`CurvedBooleanUnsupported { operand: B, kind: Nurbs }` once this row,
the boundary-first row and the two flush rows land.
