---
id: edge-mint-doors-read-the-nurbs-lane-from-the-policy
kind: issue
title: Body::set_edge_curve and the other edge-mint doors still certify through the lane-free door, so every operation that re-mints a plane x NURBS edge refuses it at f64; they should read AtRestPolicy::nurbs_lane() as transform_rigid now does
status: open
opened: 2026-10-01
priority: P1
cost: M
refs: [graft-recertifies-through-the-narrow-lane, euler-rebased-run-recertifies-through-the-plain-door]
---


This is the second unit of the converged design recorded in
`graft-recertifies-through-the-narrow-lane` ("Decided").
`transform_rigid` and the void graft were the first.

`Body::set_edge_curve` lives in `impl<T: Decide> Body<T>` and
certifies lane-free. Every operation that re-mints an edge goes
through it: `loft`, `extrude`, `blend/surgery`, `revolve/upgrade`,
`replace_faces_offset` and `set_face_surfaces_describing` (`attach.rs`).
So each of them refuses the plane × NURBS class at f64. Raising it to
`Decide + AtRestPolicy` and reading `T::nurbs_lane()` lets the plain
name carry the scalar's full rights. Once that holds,
`set_edge_curve_nurbs_lane` and `certify_nurbs_lane` can go.

The nurbs-lane lane (PR 3678) counted 33 direct non-test
`.set_edge_curve(` call sites: 25 in topo and 8 in sweep. Measure which
operations can actually receive the class before sizing the tests.
TOPO's `euler-rebased-run-recertifies-through-the-plain-door` is the
same shape one layer down, so take it together or say why not.
