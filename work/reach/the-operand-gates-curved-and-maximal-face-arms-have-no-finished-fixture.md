---
id: the-operand-gates-curved-and-maximal-face-arms-have-no-finished-fixture
kind: issue
title: No finished fixture reaches the boolean's NonMaximalFaces gate, its pair-scoped curved gate or curved_face_arm: their rows relabelled a surface the at-rest gate refuses
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [boolean-door-adopts-the-finished-body-type, an-approx-face-on-line-edges-has-no-finished-fixture]
---


Left by `boolean-door-adopts-the-finished-body-type` (PR 3987): the
boolean doors and `boolean_reduce` take finished operands, and the rows
that reached these arms built operands below tier 3, so each row now
pins its fixture's refusal at `AtRestBody::validate` and the arm is
reached by no row through a public door.

- **`NonMaximalFaces`** (`reduce::gate_maximal_faces`): every fixture
  split a face and left the seam a scaffold, which the at-rest gate
  refuses `ScaffoldAtRest` — `m3_pr4_boolean::non_maximal_operand_refuses`,
  `review_f7_pole_r1_probes` P1–P4, topo `verbs_f7_r2_probes`. The
  in-crate `offer_rows` gate-site cases still execute the
  `CoplanarNeighbours` arm on the gate directly.
- **The pair-scoped curved gate** (`CurvedPairUnsupported` for a Cone
  face whose box reaches the other operand, and its admit side, the box
  clear of it): the fixture relabels a brick face to a cone, stranding
  its four line edges (`DescriptionNotAdjacent`) and its loop's pcurves
  (`LoopDiscontinuity`) — `review_m3_pr4::curved_face_gate_witness`,
  `a_cone_relabelled_brick_clear_of_the_other_operand_is_refused_at_rest`,
  `sweep/tests/verbs_gate_r1_probes.rs` rows 4–6 (cone, tilted cone,
  torus).
- **`reduce::curved_face_arm`'s `CurvedBooleanUnsupported`** (a NURBS
  wall): the placeholder net is `UncertifiableSurface` —
  `review_m3_pr4::a_placeholder_nurbs_wall_is_refused_at_rest`.

What closes it: finished fixtures for each arm — a non-maximal operand
whose seam carries an honest description (two coplanar faces sharing a
described edge), a swept cone (`revolve` of a slanted segment) posed
with its box reaching and clearing the other operand, a tilted cone and
a torus likewise — or a statement per arm that it is unreachable from a
finished operand, and its retirement.
