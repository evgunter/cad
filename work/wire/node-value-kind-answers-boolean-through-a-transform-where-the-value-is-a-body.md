---
id: node-value-kind-answers-boolean-through-a-transform-where-the-value-is-a-body
kind: issue
title: node_value_kind answers the boolean family for a transform over a boolean, where the transform's value is a body
status: open
opened: 2026-09-19
priority: P0
cost: D
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
same holds for any consumer that reads such a transform through
`node_operand`, though none can reach it today (the section door's
reference is an input the schedule evaluated first).

MSOLVE-7 made the walk refuse a non-placeable source at the transform
(`wire::placeable_family`) and left the family a placeable source
answers with untouched. The fix is one arm: a chain of one or more
transforms over a `BOOLEAN` source answers `BODY`, because that is the
shape the placer's map yields; a row beside `msolve3_placer_refused`'s
transform-of-pattern row would pin it against the twin's own refusal.
`eval/mod.rs` is this program's, hence the slate.
