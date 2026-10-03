---
id: seam-zip-grafts-re-certify-through-the-lane-free-door
kind: issue
title: the seam-zip lanes' grafts (setopfinish, the REST lane) re-certify through the lane-free EdgeCurve::certify although their chain now holds AtRestPolicy
status: parked
opened: 2026-10-02
priority: P3
cost: E
parent: graft-recertifies-through-the-narrow-lane
refs: [the-union-fallback-graft-re-certifies-a-disjoint-operand-through-the-lane-free-door]
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---


Filed by the lane that moved the containment fallback's assembly
graft and the sphere re-cut's graft to `Bridge::RemapKeys`
(`the-union-fallback-graft-re-certifies-a-disjoint-operand-through-the-lane-free-door`).

## The sites

`boolean::combine::graft_solid` (`Bridge::Recertify { tol }`, which
re-certifies through the plain `geom_brep::EdgeCurve::certify`, no
plane × NURBS lane) now has two callers, both seam-zip lanes whose
B operand has been through surgery ahead of the graft:
- `boolean::finish::setopfinish` (`T: Decide + AtRestPolicy`), after
  the crossing splits, `weld_pinches` and, for ∖, `Body::revert`;
- `boolean::rest::try_rest_union` (`T: Decide + Bounds + AtRestPolicy`),
  after the reduction's annotations.

On an operand carrying the M7-8 class (an `Intersection` of a plane
and a described NURBS wall) either site would refuse
`GraftRecertify(NurbsLaneUnsupported)` at a scalar whose
`AtRestPolicy::nurbs_lane()` holds the lane. The bound the parent row
said the chain lacked is there now: `boolean_op_with` is
`T: Decide + Bounds + AtRestPolicy`.

## Reachability, measured

Unreachable today: the operand gate (`boolean::reduce::gate_operand_edges`)
refuses a `Curve3::Nurbs` carrier as `CurvedEdgeUnsupported` before
either lane runs, and the crossing layer refuses NURBS faces.

On the whole `topo`, `sweep`, `verbs`, `editor-core` and `pncad`
suites (2026-10-02, instrumented graft), every edge these two sites
re-certified came back with the certificate it carried in, bit for
bit: 194,490 edges at `finish.rs`, 4,807 at `rest.rs`, across `f64`,
`Interval` and `Dual<f64>`. So the surgery ahead of them re-certifies
what it touches; the re-run buys a check, not a different answer.

## What is owed

When the operand gate admits the class: re-certify these grafts
through the policy's lane (`recertify_via`/`certify_via` with
`T::nurbs_lane()`, raising `graft_solid` to `T: AtRestPolicy`), or
carry, with the measurement above as the argument.
