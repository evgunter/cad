---
id: a-rim-arc-lying-on-a-cylinder-refuses-as-a-curved-pierce
kind: issue
title: A circle arc of one operand lying on the other's cylinder refuses CurvedPierceUnsupported where a mate holds the two rims: klein wall 3 stops there
status: dispatched
opened: 2026-11-03
priority: P0
cost: M
branch: splice/rim-arc-on-cylinder
---


Filed by FRAME's `frame/turned-onto` lane, which moved klein wall 3.

## What

The sweep start frame is now the world frame turned onto the start tangent by the least turn (`RigidMotion::turned_onto`). The klein loop's start tangent is exactly `+z`, so its start frame is the world frame and its two circle sections start on `+x`; the old frame started them on `−y`. The body is the same (two centred circles), but the loop's vertices and seams now lie in the bottle's plane of symmetry `y = 0`.

Klein wall 3's union (`demos/tour/src/klein.rs`, `wall3_pinned`; `demos/tour/tests/klein.rs`, `verbs_gate_r1_probes`) now refuses `CurvedPierceUnsupported { operand: B, face: 4v1, edge: 1v1 }` at the default ε. Measured on main `1df88a2b96` plus this branch:

- edge 1v1 of the loop is a `Circle` arc of its start cap's outer rim, from `(0.275, 0, 3)` to `(−0.275, 0, 3)`;
- face 4v1 of the bulb is the neck `Cylinder { origin: (0, 0, 3), axis: +z, radius: 0.275, u_ref: +x }`, so the arc lies on it, and the arc's first vertex is the cylinder's seam point.

The loop's cap sits flat on the neck's rim (the annular REST mate), so this arc and the neck's top rim are one circle. Before the roll moved, the union stopped earlier, at the crease (`a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`), and with the seam moved into `y = 0` but before PR 6441 it stopped at the spline split (`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`, closed). So whether this arc refused under the old roll is not known: nothing reached it.

## The question

The curved pierce door takes no conic lying on the face it is read against ("an undeclared on-carrier edge or conic", `topo::BooleanError::CurvedPierceUnsupported`'s docs). Here the coincidence is the mate's own rim. Whether the arc should be cleared as on the face's boundary (the two rims are one circle) or as a declared coincidence is SPLICE's call; this row records the stop and its witness.
