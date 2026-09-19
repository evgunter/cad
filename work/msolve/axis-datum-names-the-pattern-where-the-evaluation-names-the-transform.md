---
id: axis-datum-names-the-pattern-where-the-evaluation-names-the-transform
kind: issue
title: axis_datum's dangling-input refusal is sited at the pattern while the evaluation sites the same MissingInput at the transform
status: closed
opened: 2026-09-08
closed: 2026-09-19
pr: 2885
---


(EVAL orchestrator) Disclosed by EVAL-11 (PR 2195) in its function doc
and PR body; given its file here. `eval::node_value_kind(doc, node)`
walks a transform chain and returns `Err(MissingInput { input })` for
a dangling `Transform.input`, the same kind `eval_node` refuses with;
but the recipe road's caller `mate/member.rs` `axis_datum` runs under
`pattern_map`'s `here`, so the refusal is sited at the PATTERN, while
the evaluation sites the same condition at the TRANSFORM (and poisons
the pattern through it). One condition, two seats. Unreachable through
`apply` (dependents cascade on delete; the load validator holds
liveness, `edit.rs` ~`:340`), so no row pins it. The honest fix is
`axis_datum` returning a sited `Refused` naming the transform — a
change in MSOLVE's file, MSOLVE's call.

## Closed (2026-09-19, PR 2885)

Fixed by MSOLVE-7: `axis_datum(doc, pattern, axis)` returns the sited
`Refused` pair and seats each arm itself — a missing or wrong-kind
operand at the pattern, a dangling transform input met while
classifying at THAT transform — and `pattern_map` no longer wraps its
result with `here`. To know the transform, `eval::node_value_kind`
takes the operand's id and returns the raising node beside the kind
(`Err(Box<(at, kind)>)`); the wire door's caller drops the id and
keeps its own seat. Pinned beside `axis_datum` on hand-built
documents: `mate::member::tests::a_dangling_transform_input_below_a_
circular_axis_seats_at_the_transform_on_both_roads` (the derivation
refuses `PlacerRefused { placer: <the transform>, error:
MissingInput { input } }` and the same document's evaluation fails
the transform with the same kind, poisoning the pattern through it),
with `a_missing_axis_operand_seats_at_the_pattern` and
`a_wrong_operand_kind_through_a_live_transform_seats_at_the_pattern`
holding every other arm's seat.
