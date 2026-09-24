---
id: body-seat-reads-through-the-placer-chain
kind: issue
title: viewer::combine::denotes_body judges a body seat by node kind, and after ruling 2137 a Transform's value is a body iff its input's is and a Pattern's never is — the gate must read through the placer chain, and the two combine_ops pins re-pin
status: open
opened: 2026-09-08
refs: [2173, 2137]
priority: P1
cost: D
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

## Two more prose sites (DOOR lane, 2026-09-21)

Evidence added, not a second row (implementer-discipline §6). The DOOR
row `node-placer-field-docs-say-body-where-instances-are-accepted`
repaired the kernel-side declaration
(`crates/editor-core/src/node.rs`: `Node::Transform` and `Node::Pattern`
now say they take a placer's operand — a body, a boolean's non-empty
result, or an `Instances` value taken whole — and `Node::Part` no longer
claims to be the only road to a nested pattern). Its sweep found the
viewer carrying **verbatim copies of the two field docs it fixed**:

- `crates/viewer/src/session/op.rs:~551` — `/// The body placed.`
- `crates/viewer/src/session/op.rs:~571` — `/// The body replicated.`

Same two sentences, same narrowing, and NOT among the three
`tools.rs`/`combine.rs`/`pane/create.rs` sentences this row already
lists — so they are two more words this row owes, unless CHROME would
rather they rode with whoever re-pins the seat. `session/op.rs` is
CHROME's, VIEW's and VSEAM's by `scripts/work.py territory`; VSEAM has
its own prose rows on this file
(`work/vseam/op-rs-cites-environmental-facts-at-its-old-path.md`), so a
`git mv` of this note's subject there is reasonable if CHROME prefers.

The siblings outside the viewer were filed as their own rows:
`work/bind/python-transform-door-doc-says-an-upstream-body.md`,
`work/vdoc/mate-tool-flow-header-says-a-patterns-input-is-one-body.md`
and `work/tint/msolve2-header-says-a-pattern-over-a-pattern-does-not-evaluate.md`.
