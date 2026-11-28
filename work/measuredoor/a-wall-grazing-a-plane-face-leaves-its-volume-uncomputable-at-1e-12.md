---
id: a-wall-grazing-a-plane-face-leaves-its-volume-uncomputable-at-1e-12
kind: issue
title: A cylinder wall grazing a plane face at 0.14° builds at the default eps but refuses VolumeUncomputable (props_chart_junction_open) at 1e-12
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
(`Grazing::new(79.9, 89.9)`) whose wall meets the plane face holding
the record at about 0.14° from tangent. At the default ε every op builds
sound. At `CAD_TOLERANCE_EPS=1e-12`, every op, either operand first,
refuses at the result gate:

    ResultInvalid { findings: NotFinished { findings: [VolumeUncomputable { solid: SolidKey(1v1), source: Face { face: FaceKey(16v5), source: Escalated { cause: Indeterminate { margin: MarginDiag(Value(0.0, Some(Rejection { sign: Some(Zero), passes: Positive }), ..)), band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("props_chart_junction_open") }, check: Inventory } } }] } }

It is loud, not a wrong body.

## Way through

Read why `props_chart_junction_open` decides a Zero on a grazing face's
chart junction at 1e-12 where the default ε reads it open, and either
pass the face or name the limit in the refusal.
