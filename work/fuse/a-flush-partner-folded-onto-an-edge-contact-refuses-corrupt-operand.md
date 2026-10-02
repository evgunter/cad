---
id: a-flush-partner-folded-onto-an-edge-contact-refuses-corrupt-operand
kind: issue
title: A union whose accumulator already holds a non-manifold edge contact refuses CorruptOperand { operand: B } when a flush partner folds onto it
status: dispatched
opened: 2026-10-02
priority: P1
cost: M
branch: fuse/corrupt-operand-edge-contact
---


## Measured (FUSE measurement lane, 2026-10-02, main at bdfdda30c)

A scratch probe in `crates/topo/tests` folded three bricks with
`union_with(acc, next, flush_declarations(..) + carried)`, carrying the
previous step's contact records as `carried_a` with class `Rest`:
`a` = x∈(0,1), `b` = x∈(0.5,1.5), both y∈(0,1), z∈(0,1), declared
flush; `c` = x∈(0.5,1.5), y∈(−1,0), z∈(−1,0), which touches the
`a ∪ b` rim along an edge only (a non-manifold edge contact). In the
orders that fold `c` first against `b` and then bring `a` in (`b,c,a`,
and `a,c,b` for the x∈(0.5,1.5) variant), step 2 refuses
`CorruptOperand { operand: B }` on today's kernel, with no
canonical-form change in play. Variants with `c` at x∈(0.25,0.75) and
x∈(−0.5,0.5) refuse the same way in `b,c,a`.

Not yet reproduced through the public door (`Node::Union` in
editor-core), and the probe's own carriage of records may contribute,
so the first step is that reproduction. If it holds, a boolean output
is not a legal operand of the next boolean, which DESIGN requires of
every output; priority rises to P0 then. The probe was scratch and was
removed.

Same probe: some orders of these documents already fail tier 3′ today
(`ERR(VV, EEOverlap)` in `c,b,a` of the x∈(0.5,1.5) variant, and in
`a,c,b` and `c,a,b` of the x∈(0.25,0.75) one). Those were not examined
either; reproduce them in the same step.
