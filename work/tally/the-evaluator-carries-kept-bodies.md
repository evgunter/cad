---
id: the-evaluator-carries-kept-bodies
kind: issue
title: The editor re-finishes every boolean operand at the seat, including the body the previous boolean already gated: the evaluator should carry kept bodies
status: open
opened: 2026-10-03
priority: P2
cost: M
refs: [3987]
---


Left by `boolean-door-adopts-the-finished-body-type` (PR 3987, the dual
review's F6).

The boolean doors take `&AtRestBody`. The editor's evaluator holds
`Body` values and stamps provenance after each node, so the boolean
seat finishes each operand itself: `finished_operand`
(`crates/editor-core/src/eval/wire.rs`) clones the operand and runs
`T::gate_at_rest_kept` (full tier 3). A boolean's result is gated at the
door, given up with `into_body`, stamped, and gated again at the next
seat, so an editor chain pays tier 3 twice per intermediate body,
against DESIGN D1's "pays that gate once, at the door that built it".

Measured by the review (lane `reach-dual3987-r1`, a timer around
`finished_operand` over the editor-core suite, ci profile, ε 1e-9):
14,417 seat gates, 42.2 s summed, median 0.82 ms, p90 5.4 ms, against
27.2 s for the door's own result gate on the same suite.

What closes it: values carry `AtRestBody` through the evaluator (the
node output keeps the door's verdict; provenance stamping keeps it, or
re-gates only what it changes), so the seat finishes only a body no
door finished.
