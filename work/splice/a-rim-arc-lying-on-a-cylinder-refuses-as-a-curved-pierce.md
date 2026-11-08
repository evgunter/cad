---
id: a-rim-arc-lying-on-a-cylinder-refuses-as-a-curved-pierce
kind: issue
title: A circle arc of one operand lying on the other's cylinder refuses CurvedPierceUnsupported where a mate holds the two rims: klein wall 3 stops there
status: closed
opened: 2026-11-03
priority: P0
cost: M
branch: splice/rim-arc-on-cylinder
closed: 2026-11-04
pr: 6481
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

## Closed (SPLICE, PR 6481, 2026-11-04)

`reduce::parents_distinct_from` takes the carrier ladder's `Distinct` as before. On `OutsideInventory` with a `Surface::Nurbs` parent it now asks `spline_off_carrier`, a one-way reading that answers Distinct or unread, never "same". The net is cut into its Bézier pieces at the distinct knot breakpoints (`Knots::runs()`), and the sheet reads Distinct only when every piece has a point decided off the carrier. The certificate is the identity theorem on the piece's polynomial, so it is gated on an allow-list of the carriers whose decided residual is the whole real variety: Plane, Cylinder, Sphere, the double Cone (`|h|`), and Torus where `R − r` is decided Positive. A Spindle, whose residual sees one sheet of its quartic, and any future kind stay unread. `carrier_pair_relation`'s inventory is not widened. Klein walls 3 and 4 now stop at `GermFrameUnsupported { a_face: 6v1 Cone, b_face: 4v1 Nurbs }` at every ε (SECTARMS' cone stage). The sweep row `a_rim_arc_on_a_cylinder_beside_a_spline_wall` stops at `EdgeUndescribed { Crease }` (the describer reading a C¹ double knot by multiplicity, evidence on `a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`) and at 1e-6 at an in-band `CurvedPierceEscalated`. Single FULL review: APPROVE-WITH-FIXES. Its spindle counterexample and margin gap were fixed in the fix pass, with the reviewer's rows adopted. Residue filed on ONECOPY: `a-knot-vectors-breakpoints-are-collected-four-times-with-no-door`, `the-spline-polynomial-identity-certificate-has-two-homes` (both P3).
