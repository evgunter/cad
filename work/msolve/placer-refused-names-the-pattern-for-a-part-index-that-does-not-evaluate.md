---
id: placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate
kind: issue
title: check_reference sites a Part's non-evaluating index at the pattern below it, so PlacerRefused names a node that evaluates
status: closed
priority: P3
cost: E
opened: 2026-09-22
parent: MSOLVE-11
closed: 2026-10-01
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

## The tree now draws a click to the named node (2026-09-23, PR 3100)

Since CHROME's PR 3100 (`chrome/placer-link`), a mate row refused with
`PlacerRefused` carries a link, *"see feature N"*, whose click selects
the `placer` (`crates/viewer/src/tree.rs`, `repaired_at`). On this
row's input the click lands on the healthy pattern, while the `Part`
beside it is `Failed` with the real cause. Before, the mis-siting was
only in the words; now it is a gesture that takes the user to the wrong
node, which raises the cost of the kernel defect. The tree draws what
the fault names, and the fix stays here. `tree.rs`'s module header
names this row.

Signed: (CHROME implementer lane, `chrome/placer-link`)

## Closed — the `Part`'s index is refused at the `Part` (PR 3680)

In the `Part`-agreement loop (`mate/member.rs`, `check_reference`),
the `Part`'s own index is evaluated with `count_of(part, …,
SlotId::Instance, PlacerRow::States)`. `PlacerRefused::placer` is now
the `Part`, which fails in its own right with the same refusal. The
`MissingInput` arm is sited at the `Part` too, and the arm says no
door reaches it: the walk recorded the node as a `Part` from the same
document.

The sweep found one more instance of the class. The flat-row overflow
(`names::flat_body_index`) was sited at the pattern below. It is a row
of the value of the pattern the `Part` selects from, so it is now
sited there.

Rows:

- `msolve11_mate_log::a_parts_index_that_does_not_evaluate_is_refused_at_the_part`
  constructs the refusal the finding could not: the index
  `k · i64::MAX + 1` at `k = 1`. It pins that `placer` is the `Part`,
  that the `Part` is `Failed` on its own `Instance` slot, and that the
  pattern is `Ok`.
- `msolve11_mate_log::a_parts_flat_index_past_the_row_width_is_refused_at_the_pattern_it_selects_from`
