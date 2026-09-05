---
id: transform-refuses-a-patterns-instances-value
kind: issue
title: Node::Transform takes one body, so a transform over a pattern refuses WrongOperand although one rigid map of N instances is well-defined
status: open
opened: 2026-09-05
---


Found by MSOLVE-1 (PR 1929, deviation 3, confirmed by its correctness
review NOTE-6); filed by the MSOLVE orchestrator. The node vocabulary
is DOCM's ground; the mate consequence is MSOLVE's.

`wire_transform` (`crates/editor-core/src/eval/wire.rs`) takes one
body through `body_operand`; a pattern's value is `Instances`, so
`Transform { input: pattern }` refuses `WrongOperand { expected:
"body", found: "instances" }`. The instantiate door already says one
rigid map carries every solid of a multi-solid value ("a rigid map of a
body is a rigid map of every solid in it"), so the refusal is a fence
of the operand check, not of the math. Consequence for mates: the
member walk admits transform-of-pattern and the solve places it, but
no such document evaluates; MSOLVE-1's row pins both halves. Decide
whether `Transform` (and by the same argument the other single-body
placers) accepts `Instances`, or whether the fence is intended and the
walk should refuse the shape in the mate's own voice.
