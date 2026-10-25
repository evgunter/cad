---
id: node-value-kind-answers-boolean-through-a-transform-where-the-value-is-a-body
kind: issue
title: node_value_kind answers the boolean family for a transform over a boolean, where the transform's value is a body
status: closed
opened: 2026-09-19
priority: P0
cost: D
closed: 2026-10-23
---


(Filed from MSOLVE-7's fix pass, 2026-09-19; by reading, not by
execution.) `eval::node_value_kind` (`crates/editor-core/src/eval/
mod.rs`) answers a transform chain with its SOURCE's family, and a
`Node::Boolean` source answers `family::BOOLEAN`. The evaluation's
transform reads a boolean's non-empty result as a body
(`wire::placeable_operand`, the `Boolean(Body { .. })` arm →
`Placeable::Body`) and its value is then a `ValuePayload::Body`, whose
`kind_name` is `family::BODY`. So for a circular pattern whose axis
operand is a transform over a boolean, the derivation road refuses
`WrongOperand { expected: "datum axis", found: "boolean" }` at the
pattern and the evaluation refuses `WrongOperand { expected: "datum
axis", found: "body" }` at the same pattern: one seat, two words. The
wire's `node_operand`, which could not reach it either, left in
PR 4741 with the loft section read that was its one consumer.

MSOLVE-7 made the walk refuse a non-placeable source at the transform
(`wire::placeable_family`) and left the family a placeable source
answers with untouched. The fix is one arm: a chain of one or more
transforms over a `BOOLEAN` source answers `BODY`, because that is the
shape the placer's map yields; a row beside `msolve3_placer_refused`'s
transform-of-pattern row would pin it against the twin's own refusal.
`eval/mod.rs` is this program's, hence the slate.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. Still true (`node_value_kind` maps a placed `Subtract` to the boolean family). Its one consumer is a circular pattern's axis (`mate/member.rs` `axis_datum`), and `Transform`, `Datum` and `Pattern` all retire in stage 3; probably moot. Parked on `transform-retires-into-a-placement`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Closed (2026-10-23)

Moot in stage 3. `node_value_kind` (removed in `61736a3186`),
`placeable_family`, `Node::Transform` and the `expected: "datum axis"`
refusal no longer exist anywhere under `crates/`. A circular pattern's
axis is no longer a node operand that is typed by family. It is
`PatternKind::Circular { axis: RoleRead }`: an `Axis` pose read, or a
frame's axis read through its role
(`crates/editor-core/src/node.rs:1136-1139`). No transform chain is left
to answer a family through, and no derivation road is left to disagree
with the evaluation.
