---
id: split-gate-spiric-edge-arm-has-no-row
kind: issue
title: The split gate's spiric-edge arm has no row: no public door builds a split operand with a spiric edge
status: open
opened: 2026-10-02
priority: P3
cost: M
---


Filed by the split-gate lane (PR 3843).

`splitting/classify.rs` `edge_clears` serves `Spiric` and `Nurbs`
carriers alike: an edge clears behind its own reach box
(`census::edge_reach_in`, which has a spiric arm) or behind the box of
either face it bounds. The NURBS half has rows
(`sweep/tests/reach_split_gate_per_face.rs`: the loft prism passed whole,
and the spline rim whose belly crosses the plane). The spiric half has
none: no public construction builds a split operand with a spiric
edge. `census::edge_reach_in`'s docs record the same gap for the census,
and the boolean refuses the kind at its operand gate. The arm's reach
itself is held in an aimed frame by a hand-built row
(`boxes::tests::the_spiric_edge_reach_holds_the_curve_in_an_aimed_frame`);
what is missing is the gate's verdict on a body that carries one.

**When** a door mints a spiric edge on a solid (a plane × torus section
the boolean or the split keeps, or a hollowed partial revolve that
passes tier 3), add the twin of the spline-rim row: an edge whose ends
sit on one side of the plane and whose span crosses it must refuse
`CurvedEdgeUnsupported`, and one wholly clear must pass.
