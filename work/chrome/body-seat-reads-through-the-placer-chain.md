---
id: body-seat-reads-through-the-placer-chain
kind: issue
title: viewer::combine::denotes_body judges a body seat by node kind, and after ruling 2137 a Transform's value is a body iff its input's is and a Pattern's never is — the gate must read through the placer chain, and the two combine_ops pins re-pin
status: open
opened: 2026-09-08
refs: [2173, 2137]
---

(EVAL orchestrator) From EVAL-6 (PR 2173), which built ruling 2137
(the placers are shape-preserving over the value: `Transform` and
`Pattern` accept `Instances` and yield `Instances`). The viewer's
seat gate `crates/viewer/src/combine.rs::denotes_body` decides by
node KIND and tracks the evaluator's `body_operand`; under the ruling
a `Transform` over a pattern evaluates to `Instances`, so the gate's
`Node::Transform { .. } => true` admits a pick that the boolean seat
then refuses at evaluation. The shape the announcement names: read
through the placer chain by node kind, recursively through the
document (`Transform { input }` denotes a body iff `input` does;
`Pattern` never; `Part` always). Interim, as EVAL-6 left it: the
seat still admits a `Transform` by kind and a transform-of-pattern
pick refuses at evaluation instead of at the door. EVAL-6 edited one
CHROME test to keep CI green
(`crates/viewer/tests/combine_ops.rs::the_body_seat_tracks_the_evaluators_operand_door`,
the pattern candidate as a named exception beside the sweep one);
that exemption and
`::several_bodies_are_not_one_body_at_a_seat` are the rows CHROME's
re-pin replaces. Three "the transform tool: one body" sentences
(`viewer/src/tools.rs:~62`, `combine.rs:~161`, `pane/create.rs:~921`)
are true of the tool's seat and not of the node; worth a word each.
Citations accurate at `829b37e21`.
