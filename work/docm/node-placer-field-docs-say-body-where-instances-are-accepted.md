---
id: node-placer-field-docs-say-body-where-instances-are-accepted
kind: issue
title: Node::Transform and Node::Pattern field docs say "the body placed" / "the body replicated" although both placers accept Instances
status: open
opened: 2026-09-08
---


(EVAL orchestrator) From EVAL-11's style review (PR 2195, finding 3).
`crates/editor-core/src/node.rs` declares `Node::Transform` as "a rigid
placement of an upstream body" with `input: /// The body placed.`
(~`:1640`) and `Node::Pattern` as "a pattern of an upstream body" with
`/// The body replicated.` (~`:1653`). Since the ruling in
`work/eval/transform-refuses-a-patterns-instances-value.md` (PR 2137,
EVAL-6, `wire::placeable_operand`) both placers accept `Instances` and
are shape-preserving over it, and `eval::node_value_kind` (EVAL-11)
now reads a transform's family through its input on that premise.
The declaration is the premise's home and still says "body" — the
doc-rotted-code-right case. The viewer-side members of the same class
are on CHROME's slate at
`work/chrome/body-seat-reads-through-the-placer-chain.md`; these two
field docs are the kernel-side members. `node.rs` is DOCM's.
