---
id: a-loft-of-fewer-than-two-sections-refuses-at-evaluation
kind: issue
title: A loft of fewer than two sections refuses at evaluation though its section count is authored data the doors hold
status: open
opened: 2026-10-20
priority: P3
cost: E
---


## Finding

Found by the sweep of WIRE's `wire/empty-blend-at-the-door` unit,
which moved an empty blend selection's refusal from evaluation to the
edit and load doors (`Select::fault`) on the ground that how many
names a selection holds is authored data. The sweep looked for other
counts evaluation refuses that are readable off the document as
written.

`crates/editor-core/src/eval/wire.rs` `wire_loft` refuses
`NodeErrorKind::Skin(SkinError::TooFewSections { have, need: 2 })`
when `profiles.len() < 2`, before anything is skinned. A loft's
section list is `Node::Loft`'s own operand list, fixed when the node
is written, so the count is as much the doors' fact as a blend's edge
count; `Node::input_fault` (`crates/editor-core/src/node.rs`) and the
load door's walk over it (`persist/check.rs`) are where both doors
already ask a node's operand shape. Python reaches it with
`Node.loft([], 2)` or one section.

Not the same as a union's members: `wire_union` gives an empty
member list a meaning (the typed empty value), so its count has no
bound to hold.

## What is owed

A ruling on whether the edit and load doors refuse a loft of fewer
than two sections (an `InputFault` arm, with `SkinError::TooFewSections`
then unreachable from a document), or the evaluation refusal stays and
says why a door does not hold it.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).
