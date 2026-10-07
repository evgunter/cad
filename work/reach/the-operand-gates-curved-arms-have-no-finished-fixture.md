---
id: the-operand-gates-curved-arms-have-no-finished-fixture
kind: issue
title: No finished fixture reaches the boolean's pair-scoped curved gate or curved_face_arm: their rows relabelled a surface the at-rest gate refuses
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [boolean-door-adopts-the-finished-body-type, an-approx-face-on-line-edges-has-no-finished-fixture]
---


Left by `boolean-door-adopts-the-finished-body-type` (PR 3987). The
boolean doors and `boolean_reduce` take finished operands. The rows
that reached these two arms built their operands below tier 3, so each
row now pins its fixture's refusal at `AtRestBody::validate`, and no
row reaches either arm through a public door.

- **The pair-scoped curved gate.** This is `CurvedPairUnsupported`, for
  a Cone face whose box reaches the other operand, and its admit side,
  where the box is clear of it.
  - The fixture relabels a brick face to a cone. That strands its four
    line edges (`DescriptionNotAdjacent`) and its loop's pcurves
    (`LoopDiscontinuity`).
  - Affected rows: `review_m3_pr4::curved_face_gate_witness`,
    `a_cone_relabelled_brick_clear_of_the_other_operand_is_refused_at_rest`,
    and rows 4–6 of `sweep/tests/verbs_gate_r1_probes.rs` (cone,
    tilted cone, torus).
- **`reduce::curved_face_arm`'s `CurvedBooleanUnsupported`** (a NURBS
  wall). The placeholder net is `UncertifiableSurface`
  (`review_m3_pr4::a_placeholder_nurbs_wall_is_refused_at_rest`).

What closes it is a finished fixture for each arm: a swept cone (a
`revolve` of a slanted segment) posed with its box reaching the other
operand and posed clear of it, then a tilted cone and a torus likewise.
These are the next probe. This item does not argue that either arm is
unreachable.

## The maximal-faces gate is reachable (2026-10-04)

This item first named the maximal-faces gate as well (`NonMaximalFaces`
and `CoplanarNeighbours`). It said no row reached that gate through a
public door, and offered retiring it as one way to close the item.
**That premise was false.**

The delta review of PR 3987 (`analysis/reach-delta/3987`, MINOR 1)
built three finished operands that reach the gate through the public
union:
- a kinked prism whose kink rests in a wall's chart;
- a split top whose halves share one surface key, with the seam at rest
  in that key's chart;
- a planted disc whose circle rests in the top's chart (axis `−z`).

The fixtures failed to finish only because the split or the plant left
an edge a scaffold. The poses themselves finish, and tier 3 certifies
the at-rest descriptions.

The public rows are restored from those poses:
- `NonMaximalFaces`, through `topo::union` and `boolean_reduce`:
  `m3_pr4_boolean::non_maximal_operand_refuses`,
  `review_f7_pole_r1_probes` P1–P4 and topo `verbs_f7_r2_probes`. Each
  row also pins the scaffold refusal of the body as the split leaves it.
- `CoplanarNeighbours`, offered and executed through each public op:
  the six `offer_rows::neighbours_kinked_*` cases.
- `CoplanarNeighbours`, through the public union:
  `neighbours_across_a_closed_edge`'s (b), at three scales.

Three readings of the gate stay at the gate itself, each because no
finished body of its pose exists:
- **The disc whose diameter lies inside the band** (that file's (c)).
  The at-rest gate's own dihedral read on that circle is undecided, so
  the body does not finish. The row asserts this, then asks
  `topo::test_support::maximal_faces_gate`.
- **The split top bent about its diagonal**
  (`offer_rows::neighbours_bent_far_origin_at_the_gate` and
  `neighbours_bent_at_the_band`). The bent half's far corner stays on
  the walls and lies off its own plane by the bend times its distance
  from the diagonal. The body finishes only where the band reads that
  offset as zero. The bend at the band never reads zero (measured:
  `Surface1Residual` escalated at 1e-9). The far-origin bend reads
  zero at the design tolerance but not at the tolerance its offer
  re-runs at, which is below the offset.
- **The bent disc** (`neighbours_bent_across_a_circle`). Its circle
  lies off the bent plane by the same kind of offset.

## The cone half has finished fixtures (2026-10-06)

`sweep/tests/reach_cone_root_lane.rs::every_op_refuses_a_cone_operand_at_the_pair_gate`
poses revolved frusta (finished bodies, `sweep::revolve`) with their
cone wall's box reaching a turned cube and a tilted rod, and pins
`CurvedPairUnsupported { kind: Cone }` under every op in both orders
(`an-edge-crossing-a-cone-face-has-no-root-lane`). The admit side (a
cone face whose box clears the other operand), the tilted cone, the
torus and `curved_face_arm`'s NURBS refusal are untouched by it.

