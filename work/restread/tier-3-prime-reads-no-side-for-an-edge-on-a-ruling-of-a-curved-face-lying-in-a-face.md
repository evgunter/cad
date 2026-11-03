---
id: tier-3-prime-reads-no-side-for-an-edge-on-a-ruling-of-a-curved-face-lying-in-a-face
kind: issue
title: Tier 3′ reads no side of an edge lying in a face of its own solid where a face of the edge is a cylinder or cone the edge is a ruling of
status: review
opened: 2026-10-27
priority: P1
cost: M
refs: [tier-3-prime-reads-no-side-for-an-edge-with-a-curved-face-lying-in-a-face]
pr: 6415
branch: restread/edge-on-ruling-in-face
---


Found by `tier-3-prime-reads-no-side-for-an-edge-with-a-curved-face-lying-in-a-face`'s
lane (branch `restread/curved-edge-in-face`).

## What

Tier 3′ reads a line edge lying in a planar face of its own solid for
sides (`census.rs` `edge_in_own_face`, through `rest::judge`). A planar
face of the edge, or the face it lies in, now reads wherever its curved
edges stand off the point (`link::Sector::cleared`). A face of the edge
that is itself curved still does not: a cylinder face whose ruling is
the edge, or a cone face whose ruling it is, has no sector
(`census.rs` `Sector::of` answers `TouchUnread::Unreadable` for a face
off a plane), and the census refuses
`FaceThroughEdgeWedgeUndecided { why: Unreadable }`.

No body in the topo or sweep suites reaches this at the default ε: on
the lane's branch every such refusal is gone (its PR body has the
measurement). The 42 bodies its parent row measured were all of the
planar kind. So this is the class's remaining reach, with no witness
yet.

## Fix

Read a cylinder face whose ruling is the edge in the cross-section
normal to the edge, where the configuration is invariant along it.
There the face is a circle arc from the point, with tangent `h` into
the face. Read its side of the face's plane at a real point `q` of the
arc and at the tangent point `p + h·|q − p|`. When both are decided and
agree, or the tangent reads Zero and `q` is decided, the arc between
`p` and `q` lies on `q`'s side, to within a band. Then stand in for the
face with the half-plane through the edge and `q` (a chord), and let
`rest::judge` read it against the face. Where the arc and its chord lie
on one side of the face's plane, so does the lens between them, so the
two wedges meet the face's material alike. A cone ruling's
cross-section turns along the edge, so it needs its own argument.

## Build (2026-11-03, branch `restread/edge-on-ruling-in-face`)

The row's chord, built (`link::FaceSector::chord`, `census::Link::chord_on_edge`), for a cylinder and for a cone. A cone's sections along the edge are one another scaled about its apex, which the face holding the edge holds, so the cone reads by the same argument. A tangent reading `Zero` with the arc's point decided reads by the bend. Every other sheet of the edge is held out of the lens (`link::chords_clear`). Witnesses:

- **Real bodies** (`crates/sweep/tests/an_edge_on_a_ruling_lying_in_its_own_face.rs`): a ridge resting on its floor, its arc wall transverse or tangent, now passes; a zipped keel with an arc wall now refuses `FaceThroughEdgeWedge`. Both refused `FaceThroughEdgeWedgeUndecided { why: Unreadable }` before.
- **Link-level rows** (`crates/topo/src/link/chord_rows.rs`): the tangent crossing and the cone, which no door builds today.

The PR body has the argument, the mutants and the measurement.
