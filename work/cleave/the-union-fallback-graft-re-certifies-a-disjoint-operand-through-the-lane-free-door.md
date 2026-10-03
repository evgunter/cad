---
id: the-union-fallback-graft-re-certifies-a-disjoint-operand-through-the-lane-free-door
kind: issue
title: the boolean union/intersect fallback grafts a disjoint operand with Bridge::Recertify through the lane-free certify, while the subtract fallback now carries certificates; on the M7-8 class it would refuse NurbsLaneNotSupplied at f64
status: closed
opened: 2026-10-01
priority: P2
cost: M
parent: graft-recertifies-through-the-narrow-lane
refs: [graft-recertifies-through-the-narrow-lane]
pr: 3894
branch: cleave/union-graft
closed: 2026-10-03
---


Filed by PR 3678's lane on the CLEAVE orchestrator's adjudication of
that PR's dual review.

## The split

`boolean::ops`'s containment fallback grafts the kept B shells in two
ways. The subtract's cavity goes through the void door
(`boolean::voids::insert_voids`), which since PR 3678 grafts with
`Bridge::RemapKeys` and carries the cavity's certificates. The union
and intersect assembly goes through `boolean::combine::graft_solid`,
which grafts with `Bridge::Recertify { tol }` and re-certifies every
carrier through the plain `EdgeCurve::certify`, which supplies no
plane × NURBS lane. Both graft geometry that is bit for bit the
operand's, after no surgery, so the two arms make different claims
about the same situation. On an operand carrying the M7-8 class (an
`Intersection` of a plane and a described NURBS wall), the union arm
would refuse `GraftRecertify(NurbsLaneNotSupplied)` at `f64`, a scalar
that holds the lane.

## Reachability, measured

Unreachable today: the boolean refuses the class ahead of the graft.
- PR 3678's first reviewer pinned it:
  `topo::tests::review_cleave_nurbs_lane::a_disjoint_union_with_the_m7_8_cube_refuses_before_the_graft`
  (`union(brick, m7_8_cube)` with disjoint operands refuses
  `CurvedEdgeUnsupported { operand: B }`), beside
  `subtracting_an_enclosed_m7_8_cube_refuses_before_the_void_door`
  (`CurvedPairUnsupported { operand: B }`).
- The second reviewer measured the same gate order on the union and
  subtract fallbacks in that PR's review (the orchestrator holds the
  report); the graft row's "Reachability, measured" section records
  the same refusals on main.

Both rows go red the day the operand gate admits the class.

## What is owed

Decide whether the assembly arm should carry certificates as the void
door does (the operand's geometry is untouched, so a carried
certificate is what a fresh one would be, as
`sweep`'s `revert_plane_charts::carried_cavity_certificates_equal_a_fresh_recertification`
pins for the void door), or re-certify through the policy's lane
(`T::nurbs_lane()`, which needs the bound the boolean chain does not
carry). Establish first whether the boolean's `Recertify` sites carry
anything a carried certificate would not (surgery on the operand ahead
of the graft), which PR 3678 did not check.
