---
id: f7-same-operand-coincidence-offers-a-declaration-no-door-takes
kind: issue
title: topo: the F7 gate's same-operand UndeclaredCoincidence offers 'declare the coincidence', which no declaration can settle
status: open
opened: 2026-09-30
---


(TOPO, the §5 second pass of PR 3506's fix pass: the siblings of the
F7 gate's parallelism escalation, which the fix pass routed to its own
decision.)

## What

`boolean::reduce::gate_maximal_faces` (`crates/topo/src/boolean/reduce.rs`)
compares two neighbouring faces of ONE operand with `declared: false`.
Its `PlaneEqError::Undeclared` arm (a decided or in-band zero offset,
no shared source) raises `BooleanError::UndeclaredCoincidence` with
both pair entries on the same operand, and that variant's `Display`
(`crates/topo/src/boolean/mod.rs`) ends "Recourse: declare the
coincidence, move the geometry, or lower the tolerance".

Face-pair declarations name pairs across the two operands
(`FacePairDeclaration`), so no declaration settles a same-operand pair;
the lever that reaches a pass there is F7's own, "merge those faces
first (merge_coplanar_faces)", which `NonMaximalFaces` (the gate's
definite sibling) names. PR 3506's fix pass gave the gate's plane-rung
escalations that lever (`BooleanDecision::Neighbours`); this arm is
the one left.

## Repair shape

When `pair[0].0 == pair[1].0`, end the refusal in the gate's lever
(`refusal_routes::NEIGHBOUR_LEVER`) with the tolerance an in-band
margin gives, and no declaration.
