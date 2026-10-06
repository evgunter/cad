---
id: split-carrier-gate-rows-read-the-gate-past-the-door
kind: issue
title: The split's carrier-gate rows whose fixture is not a finished body read the gate past the door: no finished fixture reaches the torus, spline, approx or spline-edge arm
status: open
opened: 2026-10-06
priority: P3
cost: M
---



Left by `split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), which makes the split's doors take
`AtRestBody`. The rows below built their operands below tier 3, so each
now pins that refusal and reads the gate through a test-support door past
the finished-body gate (`topo::test_support::split_carrier_gate`,
`split_reduce_unfinished`). No row reaches these arms through a public
door. This mirrors REACH's
`the-operand-gates-curved-arms-have-no-finished-fixture` for the
Boolean's gate.

- `topo/tests/split_gate_per_face.rs` (both rows): the unit cube with its
  top face relabelled to a sphere, torus, NURBS or approximated spline
  surface strands the top's edge descriptions and pcurves
  (`DescriptionNotAdjacent`, `Pcurve`).
- `sweep/tests/reach_split_gate_per_face.rs`
  `a_spline_edge_whose_belly_crosses_the_plane_refuses`: the spline
  re-description's pcurves do not certify, and `mint_pcurves` refuses
  them (`RowInterval`).
- the same file's `a_sphere_face_whose_side_is_not_certified_keeps_the_ball`:
  the reverted zone is refused for its sense (`CurvedSenseInverted`).
  That one is the class the gate guards (an imported face whose loop
  bounds its complement), and the at-rest gate now refuses it first.
- `topo/tests/review_m3_pr2.rs` R3 and R4: a prism with a straight
  profile corner keeps the edge between its coplanar walls a scaffold
  (`ScaffoldAtRest`), so the pose has no finished body; they read the
  reduction past the door.

The sphere arm is reached by finished fixtures through the public door
(`reach_split_gate_per_face::a_plane_that_may_meet_the_face_refuses_naming_it`,
revolved sphere caps and torus roundings). What closes this item is a
finished fixture for each remaining arm (a NURBS- or approx-walled body
posed so the plane may meet the wall, a body with a certified spline edge
between armed faces), or a sentence saying the arm is unreachable from a
finished body and the arm's retirement.
