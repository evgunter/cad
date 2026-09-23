---
id: placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate
kind: issue
title: check_reference sites a Part's non-evaluating index at the pattern below it, so PlacerRefused names a node that evaluates
status: open
priority: P3
cost: E
opened: 2026-09-22
---

## Finding

`crates/editor-core/src/mate/member.rs`, `check_reference`, the
`Part`-agreement loop. The `Part`'s own index expression is evaluated
as

```rust
let selected = count_of(level.node, index, SlotId::Instance)?;
```

and `count_of`'s refusal is `refused(node, …)` →
`MateFault::PlacerRefused { placer: node, .. }`. `level.node` is the
PATTERN below the `Part`, not the `Part` whose `Instance` slot the
expression belongs to; `SlotId::Instance` is not a slot a pattern has.
The `let … else` above it (`MissingInput { input: part }`) is sited at
`level.node` the same way.

`MateFault::PlacerRefused`'s own `placer` doc says it is *"the node
whose evaluation raised the refusal … the node an author goes and
fixes, and the node the evaluation itself fails"*. For a `Part` index
that does not evaluate, the evaluation fails the `Part` (its slot is
its own) and the pattern evaluates `Ok` — so the mate's row reads
*"node P refuses — …"* about a healthy pattern, beside a `Part` row
that is `Failed` with the real cause.

**Confidence: likely.** The attribution is read off the code and is
certain; reachability is not demonstrated: it needs a `Count`
expression in a `Part`'s index that fails to evaluate at the
document's bindings, which this lane did not construct. The
`MissingInput` arm looks unreachable (the walk reaches the `Part` as
an `Instance` pass-through; a `SplitHalf` `Part` stops the walk as
`DanglingHead`). Same class as the closed
`axis-datum-names-the-pattern-where-the-evaluation-names-the-transform`
(one condition, two seats).

Filed by CHROME (`chrome/badge-attribution`, PR 3090) while reading
which node each `MateFault` arm names; the viewer tree carries the
payload's words verbatim, so the fix is here, not there.
