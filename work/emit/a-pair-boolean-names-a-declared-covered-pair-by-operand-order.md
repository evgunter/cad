---
id: a-pair-boolean-names-a-declared-covered-pair-by-operand-order
kind: issue
title: A pair boolean names a declared covered pair by operand order: emit_topo reads merge_groups, not covered
status: parked
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
opened: 2026-10-02
design: true
priority: P2
cost: M
---


## What

Found in review of PR 3753 (emit lane, 2026-10-02). The ruling on PR 3734 (fork-log row 37) applies to the n-ary union. It links a union's member faces through the pairs its pairwise judgements consumed: `BooleanNaming::merge_groups`, plus the new `BooleanNaming::covered`. The pair boolean's emission (`emit_topo`, `name_boolean`) still reads only `merge_groups`, through the merge rows (`emit_topo.rs`, the reads of `naming.merge_groups` near lines 653 and 739). It never reads `covered`.

A declared covered pair is two coincident, same-oriented faces where the classification keeps one copy. Which copy is kept, and whether a merge is recorded, depends on which block is operand A. For example, take `a` = x 0..2 and `b` = x 0.5..1.5, both y 0..1 and z 0..1, flush on both caps and both y-walls. `a ∪ b` keeps `a` whole through the containment fallback (`BooleanResultKind::OperandA`). `b ∪ a` takes the section path (`Seamed`); `crates/topo/tests/boolean_covered.rs` pins both paths. So the pair boolean names such faces by operand order: `b`'s faces are cited in one order and not in the other.

The pair boolean does read `DiscardRow::held`, through `borders::Obstacles`, so its `Borders` already see the held region as the union's do.

## The question

Should the pair boolean follow member-space consumption as the union now does? That would make a declared covered pair one parent, `Merged` of both faces, whichever operand is A. The other answer is that the pair boolean keeps naming by the faces the result keeps. The ruling covered the union only, so this is Ev's call.

## Parked on D10 (2026-10-06)

The question is how a pair boolean names a *declared* covered pair.
Declared pairs (`Boolean`/`Union` `declare`) are on the D10 hold's
ground (`work/emit/log.md`, 2026-10-03). Under D10, declarations retire
at intent plan stage 4 (`work/intent/plan.md`: "booleans glue on Zero;
declared pairs … retire"), and coincidence becomes a margined verdict.
When that stage is built, re-ask this as "how does a pair boolean name a
coincident covered pair the verdict glued", if it still arises. Parked on
`d10-one-way-to-say-intent-is-unbuilt`, as the union member-order row
was (PR 4141).
