---
id: mate-tool-flow-header-says-a-patterns-input-is-one-body
kind: issue
title: mate_tool_flow's nested_session header justifies its shape with 'a pattern's input is one', which ruling 2137 retired
status: open
opened: 2026-09-21
priority: P4
---



(DOOR lane, from the class sweep on
`work/door/node-placer-field-docs-say-body-where-instances-are-accepted.md`.)

`crates/viewer/tests/mate_tool_flow.rs`'s `nested_session` header
(~`:789`) says the fixture is *"`Pattern` over `Part { Instance(1) }`
over `Pattern` over the post: the shape a nested copy is reachable
through, since a pattern's value is many bodies and a pattern's input
is one."*

The **justification** is retired. Under the ruling on
`work/eval/transform-refuses-a-patterns-instances-value.md` (PR 2137;
recoverable at `git show 9c515cb1:work/eval/transform-refuses-a-patterns-instances-value.md`)
`wire::placeable_operand` admits an `Instances` value taken whole, so a
`Pattern` directly over a `Pattern` evaluates —
`crates/editor-core/tests/eval6_placers_over_instances.rs`'s
`nested_doc` builds exactly that, with no `Part` between the levels.
The fixture's shape is still a legitimate thing to test (a `Part`
between two patterns is a document that replicates ONE chosen copy, and
it is the shape the picks below want), but "the shape a nested copy is
reachable through" now reads as a claim about the only reachable shape,
which is false.

The fix is the header's clause: say the `Part` chooses one copy, rather
than that it is the only road. Nothing about the assertions moves; the
fixture builds the same document either way.

Owner note: `crates/viewer/tests/*` is VDOC's, S-TCOST's and S-TINT's by
declaration (`scripts/work.py territory`); filed here because the defect
is prose, which is this program's subject. The kernel-side sibling of
this sentence is filed on S-TINT's slate as
`msolve2-header-says-a-pattern-over-a-pattern-does-not-evaluate`
(`crates/editor-core/tests/msolve2_member_chain.rs`'s module header,
which states the same premise as fact rather than as a justification).

- `crates/viewer/tests/mate_tool_flow.rs` — `nested_session`'s doc comment
