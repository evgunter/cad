---
id: a-section-edge-of-a-wall-grazing-a-plane-face-is-uncertified-at-1e-12
kind: issue
title: A section edge a cylinder wall grazing a plane face at 0.03° mints refuses DescriptionUncertified (AnalyticRung3 KernelLimit) at 1e-12
status: open
opened: 2026-11-26
priority: P3
cost: M
---


Found by the `heldrecords/minted-door` lane's stress row,
`crates/sweep/tests/a_pierce_held_in_a_curved_face.rs`
`a_tool_face_grazing_the_holding_face_decides_or_refuses_and_never_ships_a_wrong_body`,
which pins it as a typed limit.

## What

Two prisms joined along an edge-in-face record, against a cylinder
(`Grazing::new(79.98, 89.98)`) whose wall meets the plane face holding
the record at about 0.03° from tangent. At `CAD_TOLERANCE_EPS=1e-12`
every op, either operand first, refuses in the join:

    Join(BooleanJoin(Euler(DescriptionUncertified { edge: Minted, error: AnalyticRung3(KernelLimit { operand: Plane, bound: 1.137e-12, margin: MarginDiag(Value(1.137e-12, ..)), width: 1.91e-11, extent: 0.00144 }) })))

At the default ε the same pose builds or refuses `Unread`. The refusal
is typed, not a wrong body: the minted section edge's analytic rung 3
meets the kernel's rounding floor at the plane.

## Way through

Decide whether a section edge this close to tangent is a kernel limit
to state (and name it so in the refusal), or a rung that a tighter bound
on the plane's operand passes.
