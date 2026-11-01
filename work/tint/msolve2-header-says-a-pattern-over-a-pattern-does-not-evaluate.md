---
id: msolve2-header-says-a-pattern-over-a-pattern-does-not-evaluate
kind: issue
title: msolve2_member_chain's module header states 'a pattern OVER a pattern does not evaluate', which is false since ruling 2137
status: open
opened: 2026-09-21
priority: P4
---



(DOOR lane, from the class sweep on
`work/door/node-placer-field-docs-say-body-where-instances-are-accepted.md`.)

`crates/editor-core/tests/msolve2_member_chain.rs`'s module header
(~`:4`) opens: *"A pattern's value is many bodies and a pattern's input
is one, so a pattern OVER a pattern does not evaluate. The nested shape
a user can build runs through `Node::Part { select: Instance(i) }` …"*

**Both halves are false now.** Under the ruling on
`work/eval/transform-refuses-a-patterns-instances-value.md` (PR 2137;
recoverable at `git show 9c515cb1:work/eval/transform-refuses-a-patterns-instances-value.md`)
`wire::placeable_operand` admits an `Instances` value taken whole, so
`Pattern { input: <a pattern> }` evaluates and yields `Instances` laid
out placement-major (`names::flat_body_index`). The proof is in this
crate's own suite:
`crates/editor-core/tests/eval6_placers_over_instances.rs`'s `nested_doc`
builds `linear(linear(cube))` with no `Part` between the levels, and
`msolve1_transform_aware.rs::a10` pins that a nested pattern head is a
member.

This is a suite-integrity defect of the header-claim kind: the header
states a kernel property as the REASON its rows take the shape they do,
and no assertion in the file can fail when that property stops holding.
The rows themselves are unaffected — `Pattern` over `Part` over
`Pattern` is still a document a user builds, and still the one whose
member chain this suite is about. What is wrong is the sentence that
says it is the only one.

The fix is the header's first paragraph: say that a `Part` between two
pattern levels selects WHICH copy is replicated (and that a placer takes
an `Instances` value whole, so the un-projected nest is a different
document), not that the nest is otherwise unbuildable.

Owner note: `crates/editor-core/tests/*` is S-TCOST's and S-TINT's by
declaration (`scripts/work.py territory`); filed here rather than on
S-TCOST's because no cost claim is involved. If the fit is wrong this is
a `git mv`. The viewer-side sibling is on VDOC's slate as
`mate-tool-flow-header-says-a-patterns-input-is-one-body`.

- `crates/editor-core/tests/msolve2_member_chain.rs` — the `//!` header, first paragraph
