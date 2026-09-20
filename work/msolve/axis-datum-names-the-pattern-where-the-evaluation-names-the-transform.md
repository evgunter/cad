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

Fixed by MSOLVE-7: `axis_datum(doc, pattern, axis)` returns the seated
pair (`eval::Seated`) and seats each arm itself — a missing or
wrong-kind operand at the pattern; anything the classifier refuses on
the operand's transform chain carried with the classifier's seat —
and `pattern_map` no longer wraps its result with `here`. To seat at
the transform, `eval::node_value_kind(doc, id)` returns the raising
node beside the kind and refuses, at a transform, a source that is not
placeable (`wire::placeable_family`, the recipe-side reading of
`placeable_operand`'s rule) as well as an input that is no live node;
the wire door's one caller drops the seat, which it explains. Pinned
beside `axis_datum` on a pattern over a real body with hand-pushed
transforms, every row asserting BOTH roads (the derivation's placer and
kind against the evaluation's own failed node and kind, variant and
fields):
`mate::member::tests::a_dangling_input_of_the_axis_transform_seats_at_that_transform_on_both_roads`,
`a_dangling_input_two_transforms_down_seats_at_the_deeper_transform_on_both_roads`,
`a_missing_axis_operand_seats_at_the_pattern_on_both_roads`,
`a_frame_datum_as_the_axis_seats_at_the_pattern_on_both_roads`,
`a_body_as_the_axis_seats_at_the_pattern_on_both_roads`,
`a_transform_over_the_body_as_the_axis_seats_at_the_pattern_on_both_roads`,
`two_transforms_over_the_body_as_the_axis_seat_at_the_pattern_on_both_roads`,
and — the one shape `apply` admits, found by the correctness arm —
`a_transform_over_a_datum_as_the_axis_seats_at_the_transform_on_both_roads`.
